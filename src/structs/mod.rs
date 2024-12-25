use crate::*;

use bevy::ecs::{component::ComponentId, world::DeferredWorld};

mod anchor;
mod cursor_position;
mod position;
mod tikz_component;

pub use anchor::*;
pub use cursor_position::*;
pub use position::*;
pub use tikz_component::*;

#[derive(Component)]
pub struct CircuitText;

#[derive(Component, Clone, Copy, Default)]
pub struct BuildInfo {
    pub angle: f32,
    pub len: f32,
}

impl BuildInfo {
    pub fn new(angle: f32, len: f32) -> Self {
        Self { angle, len }
    }
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Angle: {}, Len: {}", self.angle, self.len)
    }
}

// NOTE:This label is what appear in the circuit
#[derive(Component, Clone)]
#[component(on_insert = on_insert_hook)]
pub struct Info {
    pub label: String,
    pub scale: String,
}

// NOTE: This considers that the "label" or "text" entity is the first child.
fn on_insert_hook(mut world: DeferredWorld, entity: Entity, _component: ComponentId) {
    let Some(children) = world.entity(entity).get::<Children>() else {
        return;
    };

    let text_ent = children[0];
    let new_text = Text2d::new(world.get::<Info>(entity).unwrap().label.clone());
    if let Some(mut text) = world.get_mut::<Text2d>(text_ent) {
        *text = new_text;
    }
}

impl Info {
    pub fn with_scale(&mut self, scale: String) -> Self {
        Self {
            label: self.label.clone(),
            scale,
        }
    }

    pub fn with_label(&mut self, label: String) -> Self {
        Self {
            label,
            scale: self.scale.clone(),
        }
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            label: Default::default(),
            scale: 1.0.to_string(),
        }
    }
}
