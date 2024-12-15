use bevy::prelude::*;

use crate::{GRID_COUNT, GRID_SIZE};

// use super::RoundState;

#[derive(Debug, Default, Eq, PartialOrd, Ord, Hash, Component, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub const FAR: Self = Self { x: -100, y: -100 };

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    // pub fn len(&self) -> isize {
    //     (self.x * self.x + self.y * self.y).sqrt()
    // }

    pub fn sq_len(&self) -> isize {
        self.x * self.x + self.y * self.y
    }

    const HALF_SIZE: isize = GRID_SIZE as isize * GRID_COUNT as isize / 2;
    pub const fn within_grid(&self) -> bool {
        self.x >= (-Self::HALF_SIZE + 160)
            && self.x <= (Self::HALF_SIZE + 160)
            && self.y >= -Self::HALF_SIZE
            && self.y <= Self::HALF_SIZE
    }

    pub fn close_to(&self, other: impl Into<Self>) -> bool {
        self.sq_distance(&other.into()) < GRID_SIZE as isize / 2
    }

    pub fn sq_distance(&self, other: &Self) -> isize {
        (*self - *other).sq_len()
    }

    // pub fn is_round(&self) -> bool {
    //     *self == self.round()
    // }

    // pub fn round(&self) -> Self {
    //     Self {
    //         x: (self.x / GRID_SIZE as isize).round() * GRID_SIZE as isize,
    //         y: (self.y / GRID_SIZE as isize).round() * GRID_SIZE as isize,
    //     }
    // }

    // pub fn round_to_tuple(&self) -> (isize, isize) {
    //     (
    //         ((self.x * 1000.0).round() / 1000.0) as isize,
    //         ((self.y * 1000.0).round() / 1000.0) as isize,
    //     )
    // }

    // FIXME: Fix this
    //
    pub fn tikz_coords(&self) -> Vec2 {
        Vec2 {
            x: self.x as f32 / (2.0 * GRID_SIZE),
            y: self.y as f32 / (2.0 * GRID_SIZE),
        }
    }

    pub fn walk(&self, angle: f32, len: f32) -> Self {
        *self
            + Self {
                x: (angle.cos() * len) as isize,
                y: (angle.sin() * len) as isize,
            }
    }
    pub fn from_vec2(vec: Vec2) -> Self {
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
