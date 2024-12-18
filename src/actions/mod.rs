use crate::create::ConvertCircuit;
use crate::graph::RemoveFromGraph;
use crate::structs::{Anchored, ComponentLabel, Info, Marker, Selected, TikzComponent};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub mod draw_components;

#[allow(clippy::type_complexity)]
pub fn move_entity(
    mut commands: Commands,
    marked: Single<&GlobalTransform, With<Marker>>,
    component: Single<Entity, (With<Selected>, With<Anchored>)>,
) {
    commands
        .entity(*component)
        .insert(Anchored(marked.translation().truncate()));
}

#[derive(Event)]
pub enum UpdateInfo {
    Label(String),
    Scale(String),
}

pub fn update_info(trigger: Trigger<UpdateInfo>, mut commands: Commands, query: Query<&Info>) {
    let ent = trigger.entity();
    let info = query.get(ent).unwrap();
    let old_label = info.label.clone();
    let old_scale = info.scale.clone();
    let info = match trigger.event() {
        UpdateInfo::Label(label) => Info {
            label: label.to_string(),
            scale: old_scale,
        },
        UpdateInfo::Scale(scale) => Info {
            label: old_label,
            scale: scale.to_string(),
        },
    };

    commands.entity(ent).insert(info);
    commands.trigger(ConvertCircuit);
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
    _: Trigger<UpdateComponentLabel>,
    mut components: Query<(&mut ComponentLabel, &TikzComponent)>,
) {
    let mut map = HashMap::<String, u32>::new();
    for (mut label, typec) in components.iter_mut() {
        let f_type = match typec {
            TikzComponent::AndGate => "AND",
            TikzComponent::OrGate => "OR",
            TikzComponent::XorGate => "XOR",
            TikzComponent::NotGate => "NOT",
            _ => "E",
        };

        let count = map.entry(f_type.to_string()).or_insert(0);
        *count += 1;

        *label = ComponentLabel {
            label: format!("{f_type}{count}"),
        };
    }
}
