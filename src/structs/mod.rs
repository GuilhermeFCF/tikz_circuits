use crate::*;
use bevy::ecs::component::{ComponentHooks, StorageType};

mod component_stuff;
mod mark_node;
mod position;
mod tikz_component;

pub use component_stuff::*;
pub use mark_node::*;
pub use position::*;
pub use tikz_component::*;

#[derive(Resource)]
pub struct CircuitText(pub String);

#[derive(Clone, Copy)]
pub struct FirstPos;

impl Component for FirstPos {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        const SCALE: f32 = 3.0;
        hooks.on_add(|mut world, entity, _| {
            if world.get::<Selected>(entity).is_none() && world.get::<Marker>(entity).is_none() {
                let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                    error!("Hook on non-existing entity");
                    return;
                };
                transform.scale *= SCALE;
            }
        });

        hooks.on_remove(|mut world, entity, _| {
            if world.get::<Selected>(entity).is_none() && world.get::<Marker>(entity).is_none() {
                let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                    error!("Hook on non-existing entity");
                    return;
                };
                transform.scale /= SCALE;
            }
        });
    }
}

#[derive(Event)]
pub struct ConvertCircuit;

#[derive(Event)]
pub struct DeleteAll;

#[derive(Component, Clone, Copy)]
pub struct BuildInfo {
    pub angle: f32,
    pub len: f32,
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Angle: {}, Len: {}", self.angle, self.len)
    }
}

#[derive(Event)]
pub struct DeleteComponent;

#[derive(Event)]
pub struct UpdateLabel(pub String);

#[derive(Component)]
pub struct ComponentInfo {
    pub label: String,
    pub scale: f32,
}

impl ComponentInfo {
    pub fn is_empty(&self) -> bool {
        self.label.is_empty() && self.scale == 1.0
    }
}

impl Default for ComponentInfo {
    fn default() -> Self {
        Self {
            label: Default::default(),
            scale: 1.0,
        }
    }
}

#[derive(Event)]
pub struct InitiateComponent {
    pub pos: Entity,
}

#[derive(Event)]
pub struct CreateComponent {
    pub initial: Entity,
    pub fin: Entity,
}

#[derive(Event)]
pub struct CreateSingleComponent {
    pub node: Entity,
}

pub struct Selected;

impl Component for Selected {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                error!("Hook on non-existing entity");
                return;
            };
            transform.scale *= 1.05;
            world.commands().trigger(ConvertCircuit);
        });
        hooks.on_remove(|mut world, entity, _| {
            let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                error!("Hook on non-existing entity");
                return;
            };
            transform.scale /= 1.05;
            world.commands().trigger(ConvertCircuit);
        });
    }
}
