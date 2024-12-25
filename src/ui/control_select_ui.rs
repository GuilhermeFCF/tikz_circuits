use crate::{create::ConvertCircuit, input_widget::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct SelectedInterface;

#[derive(Component)]
pub struct SelectedComponentIdentifier;

#[derive(Component)]
pub struct LabelIdentifier;

#[derive(Component)]
pub struct ScaleIdentifier;

pub fn enable_selected_ui(
    _: Trigger<OnAdd, crate::actions::select_node::Selected>,
    mut selected_ui: Single<&mut Visibility, With<SelectedInterface>>,
    mut component_identifier: Single<
        &mut Text,
        (
            With<SelectedComponentIdentifier>,
            Without<LabelIdentifier>,
            Without<ScaleIdentifier>,
        ),
    >,
    mut label_identifier: Single<
        &mut TextInputValue,
        (
            With<LabelIdentifier>,
            Without<SelectedComponentIdentifier>,
            Without<ScaleIdentifier>,
        ),
    >,
    mut scale_identifier: Single<
        &mut TextInputValue,
        (
            With<ScaleIdentifier>,
            Without<SelectedComponentIdentifier>,
            Without<LabelIdentifier>,
        ),
    >,
    selected: Single<
        (&crate::TikzComponent, &crate::structs::Info),
        With<crate::actions::select_node::Selected>,
    >,
) {
    let (cc, info) = *selected;

    component_identifier.0 = cc.to_string();
    label_identifier.0 = info.label.clone();
    scale_identifier.0 = info.scale.clone();

    **selected_ui = Visibility::Inherited;
}

pub fn disable_selected_ui(
    _: Trigger<OnRemove, crate::actions::select_node::Selected>,
    selected_interface: Single<&mut Visibility, With<SelectedInterface>>,
) {
    let mut visibility = selected_interface.into_inner();
    *visibility = Visibility::Hidden;
}

pub fn submit_event(
    trigger: Trigger<TextInputSubmitEvent>,
    mut commands: Commands,
    mut focused: ResMut<super::FocusedInputText>,
    is_label: Query<&LabelIdentifier>,
    mut selected: Single<
        (Entity, &mut crate::structs::Info),
        With<crate::actions::select_node::Selected>,
    >,
) {
    let new_value = trigger.event();
    let entity = trigger.entity();

    let info = match is_label.get(entity) {
        Ok(_) => selected.1.with_label(new_value.value.clone()),
        Err(_) => selected.1.with_scale(new_value.value.clone()),
    };
    commands.entity(selected.0).insert(info);
    commands.trigger(ConvertCircuit);
    *focused = super::FocusedInputText(Entity::PLACEHOLDER);
}
