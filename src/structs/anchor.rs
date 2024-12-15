use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use crate::ConvertCircuit;

use super::{BuildInfo, ComponentStructure};

#[derive(Component)]
#[component(on_insert = on_insert_hook)]
#[require(BuildInfo, ComponentStructure(|| ComponentStructure::Node(Vec2::default())))]
pub struct Anchored(pub Vec2);

fn on_insert_hook(mut world: DeferredWorld, entity: Entity, _component: ComponentId) {
    let build_info = *world.get::<BuildInfo>(entity).unwrap();
    let Anchored(anchor) = *world.get::<Anchored>(entity).unwrap();
    let other_pos =
        anchor + build_info.len * Vec2::new(build_info.angle.cos(), build_info.angle.sin());

    let mut structure = world.get_mut::<ComponentStructure>(entity).unwrap();
    *structure = match *structure {
        ComponentStructure::Node(_) => ComponentStructure::Node(anchor),
        ComponentStructure::To(_) => ComponentStructure::To([anchor, other_pos]),
    };
    let middle = anchor.midpoint(other_pos);

    let mut transform = world.get_mut::<Transform>(entity).unwrap();
    transform.translation = (middle, 0.0).into();

    world.trigger::<ConvertCircuit>(ConvertCircuit);
}
