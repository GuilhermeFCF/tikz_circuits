use crate::*;

#[derive(Component)]
pub struct Selectable;

pub struct Selected;

impl Component for Selected {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                error!("Hook on non-existing entity");
                return;
            };
            transform.scale *= 1.05;
        });
        hooks.on_remove(|mut world, entity, _| {
            let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                error!("Hook on non-existing entity");
                return;
            };
            transform.scale /= 1.05;
        });
    }
}

pub fn despawn_selected(mut commands: Commands, selected: Single<Entity, With<Selected>>) {
    commands.trigger_targets(DeleteComponent, *selected);
}
