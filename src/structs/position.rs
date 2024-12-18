use bevy::prelude::*;

use crate::GRID_SIZE;

// use super::RoundState;

#[derive(Debug, Default, Eq, PartialOrd, Ord, Hash, Component, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn tikz_coords(&self) -> Vec2 {
        Vec2 {
            x: self.x as f32 / (2.0 * GRID_SIZE),
            y: self.y as f32 / (2.0 * GRID_SIZE),
        }
    }
}

impl From<Vec2> for Position {
    fn from(vec: Vec2) -> Self {
        Self {
            x: vec.x as isize,
            y: vec.y as isize,
        }
    }
}

impl std::ops::Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl std::ops::Mul<isize> for Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<isize> for Position {
    type Output = Position;

    fn div(self, rhs: isize) -> Self::Output {
        Position {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
