use crate::actions::UpdateComponentLabel;
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

#[derive(Component, Debug, Copy, Clone)]
pub enum ComponentStructure {
    Node(Vec2),
    To([Vec2; 2]),
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
    Line,
    AmpOp,
    Transistor,
    Diode,
    Transformer,
}

impl TikzComponent {
    #[inline]
    pub fn is_single(&self) -> bool {
        use TikzComponent::*;
        matches!(self, Ground | Dot | AmpOp | Transistor | Transformer) || self.is_gate()
    }

    #[inline]
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
            AmpOp => "op amp",
            Transistor => "npn",
            Diode => "D",
            Transformer => "transformer",
            // Label => panic!("Reaching tikz_type with type label"),
        }
    }

    pub fn get_label_height(&self) -> f32 {
        use TikzComponent::*;
        match self {
            AmpOp => 0.,
            x if x.is_gate() => 2.,
            Line => 0.75,
            _ => 1.5,
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
            VSource => "V",
            ISource => "C",
            AndGate => "And",
            OrGate => "Or",
            XorGate => "Xor",
            NotGate => "Not",
            AmpOp => "AmpOp",
            Transistor => "Transistor",
            Diode => "Diodo",
            Transformer => "Trafo",
        };
        write!(f, "{c}")
    }
}
