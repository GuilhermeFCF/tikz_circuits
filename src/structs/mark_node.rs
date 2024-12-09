use core::f32;

use crate::*;

#[derive(Component)]
pub struct Markable;

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

pub fn mark_node(
    mut commands: Commands,
    q_nodes: Query<(Entity, &GlobalTransform), With<Markable>>,
    cursor_pos: Res<CursorPosition>,
    marker: Query<Entity, With<Marker>>,
) {
    let cursor = cursor_pos.pos;

    let closest_entity = q_nodes
        .iter()
        .fold(
            (Entity::PLACEHOLDER, f32::MAX),
            |(closest_ent, closest_dist), (node_ent, node_transform)| {
                let node_pos = Position::from(node_transform.translation());
                let dist = cursor.distance(&node_pos);
                if dist < closest_dist {
                    (node_ent, dist)
                } else {
                    (closest_ent, closest_dist)
                }
            },
        )
        .0;

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
