use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use crate::ConvertCircuit;

use super::{BuildInfo, ComponentStructure, Position};

#[derive(Component)]
#[component(on_insert = on_insert_hook)]
#[require(BuildInfo, ComponentStructure(|| ComponentStructure::Node(Position::default())))]
pub struct Anchored(pub Position);

fn on_insert_hook(mut world: DeferredWorld, entity: Entity, _component: ComponentId) {
    let build_info = *world.get::<BuildInfo>(entity).unwrap();
    let Anchored(pos) = *world.get::<Anchored>(entity).unwrap();
    let final_pos = pos.walk(build_info.angle, build_info.len);

    let mut structure = world.get_mut::<ComponentStructure>(entity).unwrap();
    *structure = match *structure {
        ComponentStructure::Node(_) => ComponentStructure::Node(pos),
        ComponentStructure::To(_) => ComponentStructure::To([pos, final_pos]),
    };
    let middle = (final_pos + pos) / 2.0;

    let mut transform = world.get_mut::<Transform>(entity).unwrap();
    transform.translation = middle.into();

    world.trigger::<ConvertCircuit>(ConvertCircuit);
}
