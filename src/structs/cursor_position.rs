use super::Position;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub pos: Position,
    pub within_grid: bool,
}

impl CursorPosition {
    pub fn update_pos(&mut self, pos: impl Into<Position>) {
        let pos: Position = pos.into();
        *self = Self {
            pos,
            within_grid: pos.within_grid(),
        }
    }
}

pub fn get_cursor_position(
    mut cursor: ResMut<CursorPosition>,
    window: Single<&Window>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *q_camera;
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };
    cursor.update_pos(point);
}
