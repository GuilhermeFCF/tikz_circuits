use crate::{structs, ui};
use bevy::prelude::*;

use super::GRID_SIZE;

#[derive(Component)]
pub struct CursorIdentifier;

#[derive(Component)]
pub struct ZeroMarker;

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub pos: Vec2,
}

impl CursorPosition {
    pub fn update_pos(&mut self, pos: impl Into<Vec2>) {
        let pos: Vec2 = pos.into();
        *self = Self { pos }
    }
}

pub fn get_cursor_position(
    mut cursor: ResMut<CursorPosition>, window: Single<&Window>,
    q_camera: Single<(&Camera, &OrthographicProjection, &GlobalTransform)>,
    mut cursor_identifier: Single<&mut Transform, With<CursorIdentifier>>,
    mut ui_pos: Single<&mut Text, With<ui::PositionIdentifier>>,
) {
    let (camera, projection, camera_transform) = *q_camera;

    let scale = match projection.scale {
        0.25 => 0.25,
        x if x <= 0.5 => 0.5,
        _ => 1.,
    };
    let precision = scale * GRID_SIZE;

    if let Some(point) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        let point = (point / precision).round() * precision;

        let tikz = structs::Position::from(point).tikz_coords();
        ui_pos.0 = format!("M({}, {}) T({}, {})", point.x, point.y, tikz.x, tikz.y);

        cursor.update_pos(point);
        cursor_identifier.translation = point.extend(0.);
    }
}
