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

use structs::{Markable, TikzComponent};

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
                structs::mark_node,
                actions::move_entity.run_if(input_pressed(MouseButton::Right)),
                structs::despawn_selected.run_if(input_just_pressed(KeyCode::Delete)),
                input::handle_left_click.run_if(input_just_pressed(MouseButton::Left)),
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
        .add_observer(actions::update_info)
        .add_observer(actions::update_component_label)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Rendering default nodes
    for x_i in 0..GRID_COUNT {
        for y_i in 0..GRID_COUNT {
            let x = x_i as f32 * GRID_SIZE - OFFSET + 160.;
            let y = y_i as f32 * GRID_SIZE - OFFSET;
            commands.spawn((
                Sprite::default(),
                Transform::from_xyz(x, y, -100.0).with_scale(Vec3::splat(1.0)),
                Markable,
            ));
        }
    }
}
