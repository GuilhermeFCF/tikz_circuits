use core::f32;

use crate::*;

pub struct Marker;

impl Component for Marker {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        const SCALE: f32 = 3.0;
        hooks.on_add(|mut world, entity, _| {
            if world.get::<TikzComponent>(entity).is_none()
                && world.get::<FirstPos>(entity).is_none()
            {
                let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                    error!("Hook on non-existing entity");
                    return;
                };
                transform.scale *= SCALE;
            }
        });
        hooks.on_remove(|mut world, entity, _| {
            if world.get::<TikzComponent>(entity).is_none()
                && world.get::<FirstPos>(entity).is_none()
            {
                let Some(mut transform) = world.get_mut::<Transform>(entity) else {
                    error!("Hook on non-existing entity");
                    return;
                };
                transform.scale /= SCALE;
            }
        });
    }
}

#[allow(clippy::complexity)]
pub fn mark_node(
    mut commands: Commands,
    q_nodes: Query<(Entity, &GlobalTransform), With<TikzNode>>,
    cursor_pos: Res<CursorPosition>,
    marker: Query<Entity, With<Marker>>,
) {
    let cursor = cursor_pos.0;
    let mut closest = f32::MAX;
    let mut closest_entity = Entity::PLACEHOLDER;
    for (ent, node) in &q_nodes {
        let pos = Position::from(node.translation());
        if cursor.distance(&pos) - closest < f32::EPSILON {
            closest = cursor.distance(&pos);
            closest_entity = ent;
        }
    }

    if closest_entity != Entity::PLACEHOLDER {
        if let Ok(entity) = marker.get_single() {
            if entity == closest_entity {
                return;
            }
            commands.entity(entity).remove::<Marker>();
        }
        commands.entity(closest_entity).insert(Marker);
    }
}
