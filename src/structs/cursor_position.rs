use bevy::prelude::*;

use super::{GRID_COUNT, GRID_SIZE};
#[derive(Resource, Default)]
pub struct CursorPosition {
    pub pos: Vec2,
    pub within_grid: bool,
}

impl CursorPosition {
    pub fn update_pos(&mut self, pos: impl Into<Vec2>) {
        let pos: Vec2 = pos.into();
        *self = Self {
            pos,
            within_grid: within_grid(pos),
        }
    }
}

const fn within_grid(pos: Vec2) -> bool {
    const BOUND: f32 = GRID_COUNT as f32 * GRID_SIZE / 2.0;
    pos.x <= BOUND + 160.0 && pos.x >= -BOUND + 160.0 && pos.y <= BOUND && pos.y >= -BOUND
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
