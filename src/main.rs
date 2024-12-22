use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
    utils::HashMap,
};

mod actions;
mod components;
mod create;
mod graph;
mod input;
mod input_widget;
mod structs;
mod ui;
use input_widget::TextInputPlugin;

use structs::TikzComponent;

const TEXT_SCALE: f32 = 0.6;
const GRID_SIZE: f32 = 16.0;
const GRID_COUNT: u32 = 40;
const OFFSET: f32 = GRID_COUNT as f32 * GRID_SIZE / 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TextInputPlugin)
        .add_plugins(ui::UiPlugin)
        .insert_resource(structs::TikzComponent::Resistor)
        .insert_resource(structs::CursorPosition::default())
        .insert_resource(create::CurrentFile(
            "/home/guilherme/projects/circuits/test.tex".to_string(),
        ))
        .add_plugins(graph::GraphPlugin)
        .add_systems(Startup, (setup, components::load_handles))
        .add_systems(
            Update,
            (
                structs::get_cursor_position,
                actions::select_node::despawn_selected.run_if(input_just_pressed(KeyCode::Delete)),
                actions::move_entity.run_if(input_pressed(MouseButton::Right)),
                input::remove_all.run_if(input_just_pressed(MouseButton::Middle)),
                input::change_current_component,
                input::camera_movement,
                input::cancel_action.run_if(input_just_pressed(KeyCode::Escape)),
                input::zoom_scale,
            ),
        )
        .add_observer(create::create)
        .add_observer(create::update_file)
        .add_observer(actions::draw_components::draw_initial_component)
        .add_observer(actions::delete_component)
        .add_observer(actions::update_component_label)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Visibility::default(),
            Transform::from_xyz(160., 0., 0.),
            structs::ZeroMarker,
        ))
        .with_children(|commands| {
            commands.spawn((
                Sprite {
                    color: Color::srgb(1., 0., 0.),
                    ..default()
                },
                Transform::from_scale(Vec3::new(16., 0.5, 1.0)),
            ));
            commands.spawn((
                Sprite {
                    color: Color::srgb(1., 0., 0.),
                    ..default()
                },
                Transform::from_scale(Vec3::new(0.5, 16., 1.)),
            ));
        });

    commands.spawn((
        Sprite::default(),
        Transform::from_scale(Vec3::splat(2.0)),
        structs::CursorIdentifier,
    ));
    let count = 10 * GRID_COUNT;
    for x in -(count as isize) / 2..=count as isize / 2 {
        let x = x as f32 * GRID_SIZE;
        commands.spawn((
            Sprite {
                color: Color::srgb(0.08, 0.08, 0.08),
                ..default()
            },
            Transform::from_xyz(x + 160., 0., -100.0).with_scale(Vec3::new(0.5, 4000., 0.)),
        ));
        commands.spawn((
            Sprite {
                color: Color::srgb(0.08, 0.08, 0.08),
                ..default()
            },
            Transform::from_xyz(160., x, -100.0).with_scale(Vec3::new(4000., 0.5, 0.)),
        ));
    }
}
