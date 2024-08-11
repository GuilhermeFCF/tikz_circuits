#![allow(dead_code)]

use std::collections::HashMap;

use super::Position;
use bevy::prelude::*;

#[derive(Component)]
pub enum ComponentStructure {
    Node(Position),
    To([Position; 2]),
}

pub struct TikzNode {
    pub(crate) pos: Position,
    pub(crate) label: String,
}

impl TikzNode {
    pub fn new(pos: Position, label: &str) -> Self {
        let label = label.to_string();
        Self { pos, label }
    }
}

#[derive(Component)]
pub struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

// #[derive(Component)]
// pub struct TikzNodes(pub Vec<TikzNode>);

#[derive(Resource, Default)]
pub struct TikzNodes(HashMap<Entity, Vec<TikzNode>>);

impl TikzNodes {
    pub fn insert(&mut self, ent: Entity, v: Vec<TikzNode>) {
        self.0.insert(ent, v);
    }

    pub fn get(&self, ent: Entity) -> Option<&Vec<TikzNode>> {
        self.0.get(&ent)
    }
}
