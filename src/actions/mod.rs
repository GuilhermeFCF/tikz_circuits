use crate::graph::RemoveFromGraph;
use crate::structs::{Anchored, ComponentLabel, CursorPosition, TikzComponent};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub mod draw_components;
pub mod select_node;

#[allow(clippy::type_complexity)]
pub fn move_entity(
    mut commands: Commands, cursor_positon: Res<CursorPosition>,
    component: Single<Entity, (With<select_node::Selected>, With<Anchored>)>,
) {
    commands
        .entity(*component)
        .insert(Anchored(cursor_positon.pos));
}

#[derive(Event)]
pub struct DeleteComponent;

pub fn delete_component(trigger: Trigger<DeleteComponent>, mut commands: Commands) {
    commands.entity(trigger.entity()).despawn_recursive();
    commands.trigger(RemoveFromGraph(trigger.entity()));
}

#[derive(Event)]
pub struct UpdateComponentLabel;

pub fn update_component_label(
    _: Trigger<UpdateComponentLabel>, mut components: Query<(&mut ComponentLabel, &TikzComponent)>,
) {
    let mut map = HashMap::<String, u32>::new();
    for (mut label, typec) in components.iter_mut() {
        let f_type = match typec {
            TikzComponent::AndGate => "AND",
            TikzComponent::OrGate => "OR",
            TikzComponent::XorGate => "XOR",
            TikzComponent::NotGate => "NOT",
            TikzComponent::AmpOp => "AOP",
            TikzComponent::Transformer => "T",
            TikzComponent::Transistor => "S",
            _ => "E",
        };

        let count = map.entry(f_type.to_string()).or_insert(0);
        *count += 1;

        *label = ComponentLabel {
            label: format!("{f_type}{count}"),
        };
    }
}
