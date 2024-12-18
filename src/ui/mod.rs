use bevy::prelude::*;

use crate::structs::TikzComponent;

mod helper;
pub use helper::focus_right_input;
pub use helper::submit_event;
pub use helper::update_radio;
use helper::*;

pub fn ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Px(300.),
                height: Val::Percent(100.),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            BackgroundColor(spat_color(0.15)),
        ))
        .with_children(|p| {
            create_col(p).with_children(|p| {
                // Title
                heading(p, "Selecione o componente:");

                separator(p);

                //Grid
                create_grid(p).with_children(|p| {
                    use TikzComponent::*;
                    radio_button(p, Resistor, Resistor.to_string());
                    radio_button(p, Capacitor, Capacitor.to_string());
                    radio_button(p, Inductor, Inductor.to_string());
                    radio_button(p, Line, Line.to_string());
                    radio_button(p, AndGate, AndGate.to_string());
                    radio_button(p, OrGate, OrGate.to_string());
                    radio_button(p, XorGate, XorGate.to_string());
                    radio_button(p, NotGate, NotGate.to_string());
                    radio_button(p, Dot, Dot.to_string());
                    radio_button(p, Ground, Ground.to_string());
                    radio_button(p, VSource, VSource.to_string());
                    radio_button(p, ISource, ISource.to_string());
                    radio_button(p, AmpOp, AmpOp.to_string());
                    radio_button(p, Transistor, Transistor.to_string());
                    radio_button(p, Diode, Diode.to_string());
                    radio_button(p, Transformer, Transformer.to_string());
                });

                separator(p);

                // Text comes here
                draw_text(p, "\\draw\n;").insert(crate::structs::CircuitText);

                separator(p);

                // Other configuration of components Label/Scale.
                create_row(p).with_children(|p| {
                    text_input(p);
                    draw_text(p, "Label");
                });

                create_row(p).with_children(|p| {
                    text_input(p);
                    draw_text(p, "Scale");
                });
            });
        });
}
