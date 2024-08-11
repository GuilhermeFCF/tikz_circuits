use bevy::prelude::*;

use crate::{
    structs::{
        ComponentInfo, ConvertCircuit, DeleteComponent, FirstPos, Position, RoundState, Selected,
        UpdateLabel,
    },
    CursorPosition,
};

#[allow(clippy::complexity)]
pub fn move_entity(
    cursor: Res<CursorPosition>,
    mut q_points: Query<(&mut Transform, &mut Position), (With<Selected>, Without<FirstPos>)>,
    round_state: Res<State<RoundState>>,
) {
    if !cursor.within_grid() {
        return;
    }
    let cursor = cursor.round_on_state(&round_state);
    for (mut transform, mut component_pos) in &mut q_points {
        transform.translation =
            if round_state.get() == &RoundState::Round && !component_pos.is_round() {
                let unround_pos = cursor + *component_pos - component_pos.round();
                unround_pos.into()
            } else {
                cursor.into()
            };
        *component_pos = transform.translation.into();
    }
}

pub fn update_label(
    trigger: Trigger<UpdateLabel>,
    mut commands: Commands,
    mut q_text: Query<(&mut Text, &Parent)>,
) {
    let ent = trigger.entity();
    let UpdateLabel(label) = trigger.event();
    for (mut text, parent) in &mut q_text {
        if ent == **parent {
            *text = Text::from_section(label, TextStyle::default());
            commands.entity(ent).insert(ComponentInfo {
                label: label.to_string(),
                ..default()
            });
        }
    }
    commands.trigger(ConvertCircuit);
}

pub fn delete_component(trigger: Trigger<DeleteComponent>, mut commands: Commands) {
    commands.entity(trigger.entity()).despawn_recursive();
    commands.trigger(ConvertCircuit);
}
