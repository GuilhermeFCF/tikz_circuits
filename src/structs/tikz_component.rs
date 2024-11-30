use bevy::prelude::*;
#[allow(dead_code)]
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
