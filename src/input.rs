use crate::actions::*;
use bevy::{app::AppExit, input::mouse::MouseWheel};
use structs::TikzComponent;

use crate::*;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum MouseMode {
    #[default]
    SelectAndCreate,
    Pan,
    Create,
}

pub fn cancel_action(mut commands: Commands, selected: Query<Entity, With<select_node::Selected>>) {
    if let Ok(ent) = selected.get_single() {
        commands.entity(ent).remove::<select_node::Selected>();
    }
}

#[derive(Event)]
pub struct RemoveAll;

pub fn remove_all(
    _: Trigger<RemoveAll>, mut commands: Commands, q_points: Query<Entity, With<TikzComponent>>,
) {
    if q_points.is_empty() {
        return;
    }
    let entities: Vec<_> = q_points.into_iter().collect();
    commands.trigger_targets(DeleteComponent, entities)
}

pub fn change_current_component(
    mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, mut cc: ResMut<TikzComponent>,
    mut exit: EventWriter<AppExit>, focused: Res<ui::FocusedInputText>,
) {
    if focused.0 != Entity::PLACEHOLDER {
        return;
    }

    let Some(key_map) = keys.get_just_pressed().next() else {
        return;
    };

    if *key_map == KeyCode::KeyQ {
        exit.send(AppExit::Success);
        return;
    }

    if *key_map == KeyCode::Backquote {
        commands.trigger(RemoveAll);
        return;
    }

    *cc = match key_map {
        KeyCode::KeyW => TikzComponent::Line,
        KeyCode::KeyR => TikzComponent::Resistor,
        KeyCode::KeyC => TikzComponent::Capacitor,
        KeyCode::KeyL => TikzComponent::Inductor,
        KeyCode::KeyV => TikzComponent::VSource,
        KeyCode::KeyI => TikzComponent::ISource,
        KeyCode::KeyP => TikzComponent::Dot,
        KeyCode::KeyG => TikzComponent::Ground,
        _ => return,
    }
}

// TODO: Make so its zooms into a specific position.
pub fn zoom_scale(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera: Single<&mut OrthographicProjection, With<Camera2d>>,
) {
    const ZOOM_SPEED: f32 = 0.1;
    const MIN_SCALE: f32 = 0.25;
    const MAX_SCALE: f32 = 1.5;

    for event in mouse_wheel_events.read() {
        let zoom_change = -event.y * ZOOM_SPEED;
        camera.scale = (camera.scale + zoom_change).clamp(MIN_SCALE, MAX_SCALE);
    }
}
