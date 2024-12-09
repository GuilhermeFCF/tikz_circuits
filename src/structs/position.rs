use bevy::prelude::*;

use crate::{GRID_COUNT, GRID_SIZE};

// use super::RoundState;

#[derive(Debug, Default, Component, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (((self.x) * 100.0) as i32).hash(state);
        (((self.y) * 100.0) as i32).hash(state);
    }
}

impl Position {
    pub const FAR: Self = Self {
        x: -100.0,
        y: -100.0,
    };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    const HALF_SIZE: f32 = GRID_SIZE * GRID_COUNT as f32 / 2.0;
    pub const fn within_grid(&self) -> bool {
        self.x >= (-Self::HALF_SIZE + 160.0)
            && self.x <= (Self::HALF_SIZE + 160.0)
            && self.y >= -Self::HALF_SIZE
            && self.y <= Self::HALF_SIZE
    }

    pub fn close_to(&self, other: impl Into<Self>) -> bool {
        self.distance(&other.into()) < GRID_SIZE / 2.0
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*self - *other).len()
    }

    pub fn is_round(&self) -> bool {
        *self == self.round()
    }

    pub fn round(&self) -> Self {
        Self {
            x: (self.x / GRID_SIZE).round() * GRID_SIZE,
            y: (self.y / GRID_SIZE).round() * GRID_SIZE,
        }
    }

    pub fn round_to_tuple(&self) -> (isize, isize) {
        (
            ((self.x * 1000.0).round() / 1000.0) as isize,
            ((self.y * 1000.0).round() / 1000.0) as isize,
        )
    }

    pub fn tikz_coords(&self) -> Self {
        let mut x = self.x / (2.0 * GRID_SIZE);
        let mut y = self.y / (2.0 * GRID_SIZE);
        info!("x: {x} y: {y}");
        if x == -0.0 {
            x = 0.0;
        }
        if y == -0.0 {
            y = 0.0;
        }
        Self { x, y }
    }

    pub fn walk(&self, angle: f32, len: f32) -> Self {
        *self
            + Self {
                x: angle.cos() * len,
                y: angle.sin() * len,
            }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}

impl Ord for Position {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        self.x
            .partial_cmp(&o.x)
            .unwrap()
            .then(self.y.partial_cmp(&o.y).unwrap())
    }
}

impl Eq for Position {}

impl From<Vec2> for Position {
    fn from(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Position> for Vec2 {
    fn from(v: Position) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Position> for Vec3 {
    fn from(v: Position) -> Self {
        Vec3 {
            x: v.x,
            y: v.y,
            z: 0.0,
        }
    }
}

impl From<Vec3> for Position {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
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

impl std::ops::Mul<f32> for Position {
    type Output = Position;

    fn mul(self, rhs: f32) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f32> for Position {
    type Output = Position;

    fn div(self, rhs: f32) -> Self::Output {
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
impl std::ops::Sub<f32> for Position {
    type Output = Position;

    fn sub(self, rhs: f32) -> Self::Output {
        Position {
            x: self.x - rhs,
            y: self.y - rhs,
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

impl std::ops::Add<f32> for Position {
    type Output = Position;

    fn add(self, rhs: f32) -> Self::Output {
        Position {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
