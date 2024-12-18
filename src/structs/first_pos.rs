use crate::structs::{Marker, Selected};
use bevy::prelude::*;

pub struct FirstPos;

impl Component for FirstPos {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
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
