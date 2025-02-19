use bevy::prelude::*;

use crate::actions;
use crate::input;
use crate::structs;

mod circuit_text;
mod control_select_ui;
mod helper;

use circuit_text::*;
use control_select_ui::*;
use helper::*;

pub use circuit_text::{update_file, CurrentFile, UpdateCircuitText};

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
            .add_observer(update_circuit_text)
            .add_observer(enable_selected_ui)
            .add_observer(disable_selected_ui);
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
        .observe(handle_drag_on_grid)
        .with_children(|p| {
            // Left panel
            p.spawn((
                Node {
                    width: Val::Px(300.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                BackgroundColor(spat_color(0.15)),
            ))
            .observe(|mut trigger: Trigger<Pointer<Click>>| trigger.propagate(false))
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
                    draw_text(p, "\\draw\n;").insert(CircuitText);

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
                        .observe(handle_out_button)
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
                            .observe(handle_out_button)
                            .observe(handle_click_delete_button)
                            .with_child((Text::new("Deletar"), TextFont::from_font_size(12.)));
                        });
                    });
                });

                // Buttons P S C
                change_mouse_mode(p);
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
    trigger: Trigger<Pointer<Click>>, mut commands: Commands,
    cursor_position: Res<structs::CursorPosition>,
    selectable: Query<(Entity, &GlobalTransform), With<actions::select_node::Selectable>>,
    selected: Query<(Entity, &GlobalTransform), With<actions::select_node::Selected>>,
    mouse_mode: Res<State<input::MouseMode>>, mut focused: ResMut<FocusedInputText>,
) {
    if trigger.event().event.button != PointerButton::Primary {
        return;
    }

    // Only handle drag on MouseMode::Pan
    if *mouse_mode == input::MouseMode::Pan {
        return;
    }

    *focused = FocusedInputText(Entity::PLACEHOLDER);

    if *mouse_mode == input::MouseMode::SelectAndCreate {
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
    }

    commands.trigger(actions::draw_components::InitiateComponent {
        pos: cursor_position.pos,
    })
}

fn handle_drag_on_grid(
    trigger: Trigger<Pointer<Drag>>, mut camera: Single<&mut Transform, With<Camera2d>>,
    mouse_mode: Res<State<input::MouseMode>>, time: Res<Time>,
) {
    if *mouse_mode != input::MouseMode::Pan && trigger.event().button != PointerButton::Middle {
        return;
    }
    const CAMERA_MIN: Vec3 = Vec3 {
        x: -90.,
        y: -200.,
        z: 0.,
    };

    const CAMERA_MAX: Vec3 = Vec3 {
        x: 250.,
        y: 200.,
        z: 0.,
    };

    let drag = trigger.event().delta * 10.;
    let transform = Vec3 {
        x: camera.translation.x - drag.x,
        y: camera.translation.y + drag.y,
        z: 0.,
    };
    let clamped = transform.clamp(CAMERA_MIN, CAMERA_MAX);
    camera
        .translation
        .smooth_nudge(&clamped, 1.5, time.delta_secs())
}

fn handle_over_button(
    trigger: Trigger<Pointer<Over>>, mut background: Query<&mut BackgroundColor>,
) {
    let entity = trigger.entity();
    let mut background_color = background.get_mut(entity).unwrap();
    background_color.0 = spat_color(0.25);
}

fn handle_out_button(trigger: Trigger<Pointer<Out>>, mut background: Query<&mut BackgroundColor>) {
    let entity = trigger.entity();
    let mut background_color = background.get_mut(entity).unwrap();
    background_color.0 = spat_color(0.15);
}

fn handle_click_delete_button(
    _: Trigger<Pointer<Click>>, mut commands: Commands,
    selected: Single<Entity, With<actions::select_node::Selected>>,
) {
    commands.trigger_targets(actions::DeleteComponent, *selected);
}

fn handle_click_copy_button(_: Trigger<Pointer<Click>>, circuit: Single<&Text, With<CircuitText>>) {
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(circuit.0.clone()).unwrap();
}

fn handle_click_pan_button(
    _: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<input::MouseMode>>,
) {
    next_state.set(input::MouseMode::Pan);
}

fn handle_click_select_button(
    _: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<input::MouseMode>>,
) {
    next_state.set(input::MouseMode::SelectAndCreate);
}

fn handle_click_create_button(
    _: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<input::MouseMode>>,
) {
    next_state.set(input::MouseMode::Create);
}
