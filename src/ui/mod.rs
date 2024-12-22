use bevy::prelude::*;

use crate::actions;
use crate::structs;

mod control_select_ui;
mod helper;

use control_select_ui::*;
use helper::*;

#[derive(Component)]
pub struct PositionIdentifier;

#[derive(Resource, Debug)]
pub struct FocusedInputText(pub Entity);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FocusedInputText(Entity::PLACEHOLDER))
            .add_systems(Startup, ui)
            .add_systems(
                Update,
                (
                    update_radio.run_if(resource_changed::<structs::TikzComponent>),
                    focus_right_input.run_if(resource_changed::<FocusedInputText>),
                ),
            )
            .add_observer(submit_event)
            .add_observer(change_ui_visibility)
            .add_observer(ui_visibility);
    }
}

fn close_to(pos: Vec2, other_pos: Vec2) -> bool {
    pos.distance(other_pos) < crate::GRID_SIZE
}

pub fn ui(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .observe(handle_click_on_grid)
        .with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Px(300.),
                    height: Val::Percent(100.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(spat_color(0.15)),
            ))
            .observe(
                |mut trigger: Trigger<Pointer<Click>>, mut focused: ResMut<FocusedInputText>| {
                    *focused = FocusedInputText(Entity::PLACEHOLDER);
                    trigger.propagate(false);
                },
            )
            .with_children(|p| {
                create_col(p).with_children(|p| {
                    // Title
                    heading(p, "Selecione o componente:");

                    separator(p);

                    //Grid
                    create_grid(p, 6).with_children(|p| {
                        use structs::TikzComponent::*;
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
                    draw_text(p, "\\draw\n;").insert(structs::CircuitText);

                    create_row(p).with_children(|p| {
                        p.spawn((
                            Button,
                            Node {
                                padding: UiRect::all(Val::Px(7.)),
                                border: UiRect::all(Val::Px(3.)),
                                ..default()
                            },
                            BorderColor(spat_color(0.1)),
                            BackgroundColor(spat_color(0.2)),
                            BorderRadius::MAX,
                        ))
                        .observe(handle_over_button)
                        .observe(handler_out_button)
                        .observe(handle_click_copy_button)
                        .with_child((Text::new("Copiar"), TextFont::from_font_size(12.)));
                    });

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
                        create_row(p).with_children(|p| {
                            draw_text_with_size(p, "Componente", 15.)
                                .insert(SelectedComponentIdentifier);
                        });
                        text_input(p, "Label")
                            .insert(LabelIdentifier)
                            .observe(on_selected_text_input);
                        text_input(p, "Scale")
                            .insert(ScaleIdentifier)
                            .observe(on_selected_text_input);

                        create_row(p).with_children(|p| {
                            p.spawn((
                                Button,
                                Node {
                                    padding: UiRect::all(Val::Px(7.)),
                                    border: UiRect::all(Val::Px(3.)),
                                    ..default()
                                },
                                BorderColor(spat_color(0.1)),
                                BackgroundColor(spat_color(0.2)),
                                BorderRadius::MAX,
                            ))
                            .observe(handle_over_button)
                            .observe(handler_out_button)
                            .observe(handle_click_delete_button)
                            .with_child((Text::new("Deletar"), TextFont::from_font_size(12.)));
                        });
                    });
                });
            });

            // Right side
            // Position
            p.spawn((
                Node {
                    width: Val::Px(200.),
                    height: Val::Px(20.),
                    border: UiRect::all(Val::Px(3.)),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    align_content: AlignContent::Center,
                    column_gap: Val::Px(5.),
                    ..default()
                },
                PickingBehavior {
                    should_block_lower: true,
                    is_hoverable: false,
                },
                // FocusPolicy::Block,
                BackgroundColor(spat_color(0.15)),
                BorderColor(spat_color(0.4)),
            ))
            .with_children(|p| {
                draw_text(p, "({}, {})").insert((
                    PositionIdentifier,
                    // FocusPolicy::Block,
                    PickingBehavior {
                        should_block_lower: true,
                        is_hoverable: false,
                    },
                ));
            });
        });
}

fn handle_click_on_grid(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    cursor_position: Res<structs::CursorPosition>,
    selectable: Query<(Entity, &GlobalTransform), With<actions::select_node::Selectable>>,
    selected: Query<(Entity, &GlobalTransform), With<actions::select_node::Selected>>,
) {
    if trigger.event().event.button != PointerButton::Primary {
        return;
    }
    if let Ok((selected_entity, selected_transform)) = selected.get_single() {
        commands
            .entity(selected_entity)
            .remove::<actions::select_node::Selected>();
        let selected_pos = selected_transform.translation().truncate();
        if close_to(cursor_position.pos, selected_pos) {
            return;
        }
    }
    for (ent, transform) in &selectable {
        let selected_pos = transform.translation().truncate();
        if close_to(cursor_position.pos, selected_pos) {
            commands.entity(ent).insert(actions::select_node::Selected);
            return;
        }
    }
    commands.trigger(actions::draw_components::InitiateComponent {
        pos: cursor_position.pos,
    })
}
fn handle_over_button(
    trigger: Trigger<Pointer<Over>>,
    mut background: Query<&mut BackgroundColor>,
) {
    let entity = trigger.entity();
    let mut background_color = background.get_mut(entity).unwrap();
    background_color.0 = spat_color(0.25);
}

fn handler_out_button(trigger: Trigger<Pointer<Out>>, mut background: Query<&mut BackgroundColor>) {
    let entity = trigger.entity();
    let mut background_color = background.get_mut(entity).unwrap();
    background_color.0 = spat_color(0.15);
}

fn handle_click_delete_button(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    selected: Single<Entity, With<actions::select_node::Selected>>,
) {
    commands.trigger_targets(actions::DeleteComponent, *selected);
}

fn handle_click_copy_button(
    _: Trigger<Pointer<Click>>,
    circuit: Single<&Text, With<structs::CircuitText>>,
) {
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(circuit.0.clone()).unwrap();
}
