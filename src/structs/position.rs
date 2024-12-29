use bevy::prelude::*;

#[derive(Debug, Default, Eq, PartialOrd, Ord, Hash, Component, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn tikz_coords(&self) -> Vec2 {
        Vec2 {
            x: self.x as f32 * 0.03125 - 5., // 1 / (2 * GRID_SIZE)
            y: self.y as f32 * 0.03125,
        }
    }
}

impl From<Vec2> for Position {
    fn from(vec: Vec2) -> Self {
        Self {
            x: vec.x.round() as isize,
            y: vec.y.round() as isize,
        }
    }
}
