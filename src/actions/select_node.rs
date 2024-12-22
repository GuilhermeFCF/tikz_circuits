use crate::actions::DeleteComponent;
use bevy::prelude::*;

use super::draw_components::ActualComponent;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

pub fn on_add_selected(
    trigger: Trigger<OnAdd, Selected>,
    mut query: Query<(&Parent, &mut Transform), (With<Parent>, With<ActualComponent>)>,
) {
    let entity = trigger.entity();
    for (parent, mut transform) in query.iter_mut() {
        if parent.get() == entity {
            transform.scale *= 1.5;
        }
    }
}

pub fn on_remove_selected(
    trigger: Trigger<OnRemove, Selected>,
    mut query: Query<(&Parent, &mut Transform), (With<Parent>, With<ActualComponent>)>,
) {
    let entity = trigger.entity();
    for (parent, mut transform) in query.iter_mut() {
        if parent.get() == entity {
            transform.scale /= 1.5;
        }
    }
}
pub fn despawn_selected(mut commands: Commands, selected: Single<Entity, With<Selected>>) {
    commands.trigger_targets(DeleteComponent, *selected);
}
