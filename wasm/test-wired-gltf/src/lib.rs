use bindings::exports::wired::script::types::{Guest, GuestScript};

use crate::bindings::wired::log::api::{log, LogLevel};

#[allow(warnings)]
mod bindings;
mod impls;
mod material;
mod mesh;
mod node;
mod property_tests;

#[derive(Default)]
struct Script {}

impl GuestScript for Script {
    fn new() -> Self {
        log(LogLevel::Info, "Hello from script!");

        material::test_material_api();
        mesh::test_mesh_api();
        node::test_node_api();

        Script::default()
    }

    fn update(&self, _delta: f32) {}
}

fn panic_log(err: &str) {
    log(LogLevel::Error, err);
    panic!("{}", err);
}

struct Api;

impl Guest for Api {
    type Script = Script;
}

bindings::export!(Api with_types_in bindings);
