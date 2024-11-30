#![allow(dead_code)]

use bevy::prelude::*;

/// Entity should contain a tikz node component and a global position.
#[derive(Component)]
pub enum ComponentStructure {
    Node(Entity),
    To([Entity; 2]),
}

#[derive(Component)]
pub struct TikzNode {
    pub(crate) label: String,
}

impl TikzNode {
    pub fn new(label: &str) -> Self {
        let label = label.to_string();
        Self { label }
    }
}
