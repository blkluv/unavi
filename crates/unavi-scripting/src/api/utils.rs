use std::cell::Cell;

use bevy::{log::warn, prelude::Deref};
use wasm_bridge::component::{Resource, ResourceTable, ResourceTableError};

/// A `Cell<usize>` used for reference counting.
/// Defaults to a value of 1.
#[derive(Debug, Deref)]
pub struct RefCountCell(Cell<usize>);

impl Default for RefCountCell {
    fn default() -> Self {
        Self(Cell::new(1))
    }
}

pub trait RefCount {
    fn ref_count(&self) -> &Cell<usize>;

    /// Increment the ref count by 1.
    /// Returns the new value;
    fn increment(&self) -> usize {
        let count = self.ref_count();
        let new = count.get() + 1;
        count.set(new);
        new
    }

    /// Decrement the ref count by 1.
    /// Returns the new value;
    fn decrement(&self) -> usize {
        let count = self.ref_count();
        let val = count.get();

        if val == 0 {
            warn!("Cannot decrement, ref_count already at 0");
            return 0;
        }

        let new = val - 1;
        count.set(new);
        new
    }
}

// TODO: Could we use `Rc<Node>` instead of `Node` for resource implementation, rather than
// implementing Rc functionality ourselves?

/// A WASM resource that uses reference counting for its lifecycle.
/// This allows multiple copies of a resource to be created for the same data, while
/// ensuring the data is only dropped once all references are dropped.
///
/// New copies of the resource **must** be created using methods from this trait, to ensure
/// the reference count is accurate.
pub trait RefResource: RefCount + Send + Sized + 'static {
    fn new_own(&self, rep: u32) -> Resource<Self> {
        self.increment();
        Resource::new_own(rep)
    }

    fn from_res(
        res: &Resource<Self>,
        table: &ResourceTable,
    ) -> Result<Resource<Self>, ResourceTableError> {
        let data = table.get::<Self>(res)?;
        Ok(data.new_own(res.rep()))
    }

    fn from_rep(rep: u32, table: &ResourceTable) -> Result<Resource<Self>, ResourceTableError> {
        Self::from_res(&Resource::new_own(rep), table)
    }

    /// Decrement the reference count, dropping the resource if there are no more references.
    /// Returns a boolean indicating whether the resource was dropped.
    fn handle_drop(res: Resource<Self>, table: &mut ResourceTable) -> wasm_bridge::Result<bool> {
        let data = table.get(&res)?;
        let count = data.decrement();

        // Table owns a copy of the resource, so we delete when it is the only ref left.
        if count == 1 {
            table.delete(res)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::state::StoreState;

    use super::*;

    pub fn test_drop<T: RefResource + Send>(state: &mut StoreState, res_a: Resource<T>) {
        let res_b = T::from_res(&res_a, &state.table).unwrap();

        let dummy = Resource::new_own(res_a.rep());
        assert!(state.table.get::<T>(&dummy).is_ok());

        T::handle_drop(res_a, &mut state.table).unwrap();
        assert!(state.table.get::<T>(&dummy).is_ok());

        T::handle_drop(res_b, &mut state.table).unwrap();
        let err = state.table.get::<T>(&dummy);
        assert!(err.is_err());
    }

    pub fn test_new<T: RefResource + Send>(state: &mut StoreState, res_a: Resource<T>) {
        let data = state.table.get(&res_a).unwrap();
        let res_b = data.new_own(res_a.rep());
        assert_eq!(res_a.rep(), res_b.rep());

        let res_c = T::from_res(&res_a, &state.table).unwrap();
        assert_eq!(res_a.rep(), res_c.rep());

        let res_d = T::from_rep(res_a.rep(), &state.table).unwrap();
        assert_eq!(res_a.rep(), res_d.rep());
    }

    #[macro_export]
    macro_rules! generate_resource_tests {
        ($name:ident) => {
            concat_idents::concat_idents!(host_trait = Host, $name, {
                #[test]
                #[tracing_test::traced_test]
                fn test_drop() {
                    use super::host_trait;

                    let (mut state, _recv) =
                        $crate::state::StoreState::new("test_drop".to_string());

                    let res = host_trait::new(&mut state).unwrap();

                    $crate::api::utils::tests::test_drop(&mut state, res);
                }

                #[test]
                #[tracing_test::traced_test]
                fn test_new() {
                    use super::host_trait;

                    let (mut state, _recv) = $crate::state::StoreState::new("test_new".to_string());

                    let res = host_trait::new(&mut state).unwrap();

                    $crate::api::utils::tests::test_new(&mut state, res);
                }

                #[test]
                #[tracing_test::traced_test]
                fn test_create_remove() {
                    use wasm_bridge::component::Resource;

                    use $crate::actions::ScriptAction;

                    use super::{host_trait, $name};

                    let (mut state, recv) =
                        $crate::state::StoreState::new("test_create_remove".to_string());

                    let res = host_trait::new(&mut state).unwrap();
                    let id = res.rep();

                    concat_idents::concat_idents!(create_action = Create, $name, {
                        let action = recv.try_recv().unwrap();
                        if let ScriptAction::create_action { id: action_id } = action {
                            assert_eq!(action_id, id);
                        } else {
                            panic!("Action did not match");
                        }
                    });

                    let found = state.table.get::<$name>(&Resource::new_own(id));
                    assert!(found.is_ok());

                    host_trait::drop(&mut state, res).unwrap();

                    concat_idents::concat_idents!(remove_action = Remove, $name, {
                        let action = recv.try_recv().unwrap();
                        if let ScriptAction::remove_action { id: action_id } = action {
                            assert_eq!(action_id, id);
                        } else {
                            panic!("Action did not match");
                        }
                    });

                    let found = state.table.get::<$name>(&Resource::new_own(id));
                    assert!(found.is_err());
                }
            });
        };
    }
}
