use crate::actions::*;
use bevy::{app::AppExit, input::mouse::MouseWheel};
use structs::TikzComponent;

use crate::*;

pub fn cancel_action(mut commands: Commands, selected: Query<Entity, With<select_node::Selected>>) {
    if let Ok(ent) = selected.get_single() {
        commands.entity(ent).remove::<select_node::Selected>();
    }
}

pub fn remove_all(mut commands: Commands, q_points: Query<Entity, With<TikzComponent>>) {
    if q_points.is_empty() {
        return;
    }
    let entities: Vec<_> = q_points.into_iter().collect();
    commands.trigger_targets(DeleteComponent, entities)
}

pub fn change_current_component(
    keys: Res<ButtonInput<KeyCode>>,
    mut cc: ResMut<TikzComponent>,
    mut exit: EventWriter<AppExit>,
    focused: Res<ui::FocusedInputText>,
) {
    let Some(key_map) = keys.get_just_pressed().next() else {
        return;
    };
    if focused.0 != Entity::PLACEHOLDER {
        return;
    }

    if *key_map == KeyCode::KeyQ {
        exit.send(AppExit::Success);
        return;
    }

    *cc = match key_map {
        KeyCode::KeyU => TikzComponent::Line,
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

pub fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    focused: Res<ui::FocusedInputText>,
) {
    if focused.0 != Entity::PLACEHOLDER {
        return;
    }
    use KeyCode::*;

    const CAMERA_BOUNDS: [f32; 4] = [-90., 250., -200., 200.];
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyW) {
        direction.y += 1.;
    }

    if keys.pressed(KeyS) {
        direction.y -= 1.;
    }

    if keys.pressed(KeyA) {
        direction.x -= 1.;
    }

    if keys.pressed(KeyD) {
        direction.x += 1.;
    }

    let mut final_transform = camera.translation + direction.normalize_or_zero() * 100.;
    final_transform.x = final_transform.x.clamp(CAMERA_BOUNDS[0], CAMERA_BOUNDS[1]);
    final_transform.y = final_transform.y.clamp(CAMERA_BOUNDS[2], CAMERA_BOUNDS[3]);
    camera
        .translation
        .smooth_nudge(&final_transform, 1.5, time.delta_secs())
}

pub fn zoom_scale(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera: Single<&mut OrthographicProjection, With<Camera2d>>,
) {
    const ZOOM_SPEED: f32 = 0.1;
    const MIN_SCALE: f32 = 0.35;
    const MAX_SCALE: f32 = 1.5;

    for event in mouse_wheel_events.read() {
        let zoom_change = -event.y * ZOOM_SPEED;
        camera.scale = (camera.scale + zoom_change).clamp(MIN_SCALE, MAX_SCALE);
    }
}
