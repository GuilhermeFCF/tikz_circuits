use bevy::utils::HashMap;

use crate::*;

#[allow(clippy::type_complexity)]
pub fn move_entity(
    mut commands: Commands,
    marked: Single<&GlobalTransform, With<Marker>>,
    component: Single<Entity, (With<Selected>, With<Anchored>)>,
) {
    commands
        .entity(*component)
        .insert(Anchored(marked.translation().into()));
}

#[derive(Event)]
pub struct UpdateLabel {
    label: String,
}

impl UpdateLabel {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

pub fn update_label(trigger: Trigger<UpdateLabel>, mut commands: Commands) {
    let ent = trigger.entity();
    let UpdateLabel { label } = trigger.event();
    commands.entity(ent).insert(Info::from_label(label));
    commands.trigger(ConvertCircuit);
}

#[derive(Event)]
pub struct DeleteComponent;

pub fn delete_component(trigger: Trigger<DeleteComponent>, mut commands: Commands) {
    commands.entity(trigger.entity()).despawn_recursive();
    commands.trigger(ConvertCircuit);
}

#[derive(Event)]
pub struct UpdateComponentLabel;

pub fn update_component_label(
    _: Trigger<UpdateComponentLabel>,
    mut components: Query<(&mut ComponentLabel, &TikzComponent)>,
) {
    let mut map = HashMap::<String, u32>::new();
    for (mut label, typec) in components.iter_mut() {
        let f_type = format_type(*typec);
        let count = map.entry(f_type.clone()).or_insert(0);
        *count += 1;

        *label = ComponentLabel {
            label: format!("{f_type}{count}"),
        };
    }
}

fn format_type(tikz_component: TikzComponent) -> String {
    let a = match tikz_component {
        TikzComponent::AndGate => "AND",
        TikzComponent::OrGate => "OR",
        TikzComponent::XorGate => "XOR",
        TikzComponent::NotGate => "NOT",
        _ => "E",
    };
    a.to_string()
}
