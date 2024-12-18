use crate::actions::DeleteComponent;
use bevy::{app::AppExit, input::mouse::MouseWheel};
use structs::{CursorPosition, FirstPos, Marker, Selectable, Selected, TikzComponent};

use crate::*;

fn close_to(pos: Vec2, other_pos: Vec2) -> bool {
    pos.distance(other_pos) < GRID_SIZE
}

pub fn handle_left_click(
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
    marked_node: Single<Entity, With<Marker>>,
    selectable: Query<(Entity, &GlobalTransform), With<Selectable>>,
    selected: Query<(Entity, &GlobalTransform), With<Selected>>,
    mut focused: ResMut<crate::Focused>,
) {
    *focused = crate::Focused(Entity::PLACEHOLDER);
    if !cursor_position.within_grid {
        return;
    }
    let node_entity = *marked_node;
    let cursor = cursor_position.pos;
    if let Ok((selected_entity, selected_transform)) = selected.get_single() {
        commands.entity(selected_entity).remove::<Selected>();
        let selected_pos = selected_transform.translation().truncate();
        if close_to(cursor, selected_pos) {
            return;
        }
    }
    for (ent, transform) in &selectable {
        let selected_pos = transform.translation().truncate();
        if close_to(cursor, selected_pos) {
            commands.entity(ent).insert(Selected);
            return;
        }
    }
    commands.trigger(actions::draw_components::InitiateComponent { ent: node_entity })
}

pub fn cancel_action(
    mut commands: Commands,
    q_first: Query<Entity, With<crate::structs::FirstPos>>,
    selected: Query<Entity, With<Selected>>,
) {
    if let Ok(ent) = q_first.get_single() {
        commands.entity(ent).remove::<crate::structs::FirstPos>();
    }

    if let Ok(ent) = selected.get_single() {
        commands.entity(ent).remove::<Selected>();
    }
}

pub fn remove_all(
    mut commands: Commands,
    q_points: Query<Entity, (Without<FirstPos>, With<TikzComponent>)>,
) {
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
    focused: Res<Focused>,
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
    focused: Res<Focused>,
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
