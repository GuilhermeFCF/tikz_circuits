use bevy::prelude::*;

use crate::structs::TikzComponent;
mod control_select_ui;
mod helper;

use control_select_ui::*;
use helper::*;

#[derive(Resource, Debug)]
pub struct Focused(pub Entity);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Focused(Entity::PLACEHOLDER))
            .add_systems(Startup, ui)
            .add_systems(
                Update,
                (
                    update_radio.run_if(resource_changed::<TikzComponent>),
                    focus_right_input.run_if(resource_changed::<Focused>),
                ),
            )
            .add_observer(submit_event)
            .add_observer(change_ui_visibility)
            .add_observer(ui_visibility);
    }
}
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
                create_grid(p, 6).with_children(|p| {
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
                p.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.)),
                        row_gap: Val::Px(10.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    BorderColor(spat_color(0.4)),
                ))
                .insert((Visibility::Hidden, SelectedInterface))
                .with_children(|p| {
                    draw_text_with_size(p, "Componente", 15.).insert((
                        Node {
                            width: Val::Percent(100.),
                            justify_self: JustifySelf::Center,
                            ..default()
                        },
                        SelectedComponentIdentifier,
                    ));
                    text_input(p, "Label")
                        .insert(LabelIdentifier)
                        .observe(on_selected_text_input);
                    text_input(p, "Scale")
                        .insert(ScaleIdentifier)
                        .observe(on_selected_text_input);
                });
            });
        });
}
