use bevy::utils::HashSet;

use self::wired::{
    gltf::{material::Color, node::Transform},
    math::types::{Quat, Vec3},
};

wasm_bridge::component::bindgen!({
    path: "../../wired-protocol/spatial/wit/wired-gltf",
    world: "host",
    with: {
        "wired:gltf/material/material": Material,
        "wired:gltf/mesh/mesh": Mesh,
        "wired:gltf/mesh/primitive": Primitive,
        "wired:gltf/node/node": Node,
    }
});

#[derive(Default)]
pub struct Material {
    pub name: String,
    pub color: Color,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

#[derive(Default)]
pub struct Mesh {
    pub name: String,
    pub primitives: HashSet<u32>,
}

#[derive(Default)]
pub struct Primitive {
    pub material: Option<u32>,
}

#[derive(Default)]
pub struct Node {
    pub children: HashSet<u32>,
    pub mesh: Option<u32>,
    pub name: String,
    pub parent: Option<u32>,
    pub transform: Transform,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: Quat::default(),
            scale: Vec3::splat(1.0),
            translation: Vec3::splat(0.0),
        }
    }
}

impl Vec3 {
    pub fn splat(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl Default for Quat {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}
