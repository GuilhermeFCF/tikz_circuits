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

pub fn change_ui_visibility(
    _: Trigger<OnAdd, crate::structs::Selected>,
    selected_interface: Single<&mut Visibility, With<SelectedInterface>>,
    component_identifier: Single<
        &mut Text,
        (
            With<SelectedComponentIdentifier>,
            Without<LabelIdentifier>,
            Without<ScaleIdentifier>,
        ),
    >,
    label_identifier: Single<
        &mut TextInputValue,
        (
            With<LabelIdentifier>,
            Without<SelectedComponentIdentifier>,
            Without<ScaleIdentifier>,
        ),
    >,
    scale_identifier: Single<
        &mut TextInputValue,
        (
            With<ScaleIdentifier>,
            Without<SelectedComponentIdentifier>,
            Without<LabelIdentifier>,
        ),
    >,
    selected: Single<
        (&crate::TikzComponent, &crate::structs::Info),
        With<crate::structs::Selected>,
    >,
) {
    let (cc, info) = *selected;
    let mut component_identifier_text = component_identifier.into_inner();
    let mut label_identifier_text = label_identifier.into_inner();
    let mut scale_identifier_text = scale_identifier.into_inner();

    **component_identifier_text = cc.to_string();
    label_identifier_text.0 = info.label.clone();
    scale_identifier_text.0 = info.scale.clone();

    let mut visibility = selected_interface.into_inner();
    *visibility = Visibility::Inherited;
}

pub fn ui_visibility(
    _: Trigger<OnRemove, crate::structs::Selected>,
    selected_interface: Single<&mut Visibility, With<SelectedInterface>>,
) {
    let mut visibility = selected_interface.into_inner();
    *visibility = Visibility::Hidden;
}

pub fn submit_event(
    trigger: Trigger<TextInputSubmitEvent>,
    mut commands: Commands,
    mut focused: ResMut<super::Focused>,
    is_label: Query<&LabelIdentifier>,
    mut selected: Single<&mut crate::structs::Info, With<crate::structs::Selected>>,
) {
    let new_value = trigger.event();
    let entity = trigger.entity();

    match is_label.get(entity) {
        Ok(_) => selected.label = new_value.value.clone(),
        Err(_) => selected.scale = new_value.value.clone(),
    }
    commands.trigger(ConvertCircuit);
    *focused = super::Focused(Entity::PLACEHOLDER);
}
