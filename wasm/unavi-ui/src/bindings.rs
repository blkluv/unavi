// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod unavi {
        #[allow(dead_code)]
        pub mod ui {
            #[allow(dead_code, clippy::all)]
            pub mod layout {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                #[doc(hidden)]

                macro_rules! __export_unavi_ui_layout_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_unavi_ui_layout_cabi;
            }
        }
    }
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_lib_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::unavi::ui::layout::__export_unavi_ui_layout_cabi!($ty with_types_in $($path_to_types_root)*::exports::unavi::ui::layout);
  )
}
#[doc(inline)]
pub(crate) use __export_lib_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:lib:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 167] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07.\x01A\x02\x01A\x02\x01\
B\0\x04\x01\x0funavi:ui/layout\x05\0\x04\x01\x0cunavi:ui/lib\x04\0\x0b\x09\x01\0\
\x03lib\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.20\
8.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
