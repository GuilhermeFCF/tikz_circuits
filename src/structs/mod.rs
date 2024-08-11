use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
mod component_stuff;
mod position;
mod tikz_component;

pub use component_stuff::*;
pub use position::*;
pub use tikz_component::*;

#[derive(Resource)]
pub struct CircuitText(pub String);

#[derive(Clone, Copy, Component)]
pub struct FirstPos;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum RoundState {
    Round,
    NoRound,
}

#[derive(Event)]
pub struct ConvertCircuit;

#[derive(Event)]
pub struct DeleteAll;

#[derive(Component)]
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

#[allow(dead_code)]
#[derive(Component)]
pub struct ComponentInfo {
    pub label: String,
    pub scale: f32,
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
    pub pos: Position,
}

#[derive(Event)]
pub struct CreateComponent {
    pub initial: Position,
    pub fin: Position,
}

#[derive(Event)]
pub struct CreateSingleComponent {
    pub pos: Position,
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
