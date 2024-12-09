use super::Position;
use super::UpdateComponentLabel;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

// NOTE:This label is to call the "coordinate" of that component.
#[derive(Component)]
#[component(on_add = hook)]
#[component(on_remove = hook)]
pub struct ComponentLabel {
    pub label: String,
}

fn hook(mut world: DeferredWorld, _: Entity, _component_id: ComponentId) {
    world.trigger::<UpdateComponentLabel>(UpdateComponentLabel);
}

/// Entity should contain a tikz node component and a global position.
#[derive(Component, Debug)]
pub enum ComponentStructure {
    Node(Position),
    To([Position; 2]),
}

#[derive(Debug, PartialEq, Hash, PartialOrd, Ord, Eq, Component, Copy, Clone, Resource)]
pub enum TikzComponent {
    AndGate,
    OrGate,
    XorGate,
    NotGate,
    Dot,
    Resistor,
    Capacitor,
    Inductor,
    Ground,
    VSource,
    ISource,
    Label,
    Line,
}

impl TikzComponent {
    pub fn is_single(&self) -> bool {
        use TikzComponent::*;
        matches!(self, Ground | Dot) || self.is_gate()
    }

    pub fn is_gate(&self) -> bool {
        use TikzComponent::*;
        matches!(self, AndGate | OrGate | XorGate | NotGate)
    }

    pub fn tikz_type(&self) -> &str {
        use TikzComponent::*;
        match self {
            Resistor => "R",
            Capacitor => "C",
            Inductor => "cute inductor",
            VSource => "V",
            ISource => "I",
            Line => "short",
            Dot => "circ",
            Ground => "ground",
            AndGate => "and port",
            OrGate => "or port",
            XorGate => "xor port",
            NotGate => "not port",
            Label => panic!("Reaching tikz_type with type label"),
        }
    }
}

impl std::fmt::Display for TikzComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TikzComponent::*;
        let c = match self {
            Dot => "Ponto",
            Line => "Linha",
            Resistor => "Resistor",
            Capacitor => "Capacitor",
            Inductor => "Indutor",
            Ground => "Terra",
            VSource => "Fonte Tensão",
            ISource => "Fonte Corrente",
            AndGate => "Porta And",
            OrGate => "Porta Or",
            XorGate => "Porta Xor",
            NotGate => "Porta Not",
            Label => "",
        };
        write!(f, "{c}")
    }
}
