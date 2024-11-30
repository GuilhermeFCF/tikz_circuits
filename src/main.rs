use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};
use bevy_egui::{EguiContexts, EguiPlugin};

mod comp_actions;
mod components;
mod create;
mod input;
mod structs;
mod ui;

pub use comp_actions::*;
pub use components::*;
pub use create::*;
pub use input::*;
pub use structs::*;
pub use ui::*;

const GRID_SIZE: f32 = 16.0;
const GRID_COUNT: u32 = 40;

#[derive(Resource, Deref)]
pub struct CursorPosition(Position);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(TikzComponent::Resistor)
        .insert_resource(CursorPosition(Position::default()))
        .insert_resource(CircuitText(String::default()))
        .insert_resource(CurrentFile(
            "/home/guilherme/projects/circuits/test.tex".to_string(),
        ))
        .add_systems(
            Update,
            (
                ui_system,
                get_cursor_position,
                mark_node,
                move_entity.run_if(input_pressed(MouseButton::Right)),
                handle_left_click.run_if(input_just_pressed(MouseButton::Left)),
                despawn_selected.run_if(input_just_pressed(KeyCode::Delete)),
                middle.run_if(input_just_pressed(MouseButton::Middle)),
                change_current_component,
                cancel_action.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .add_systems(Startup, (setup, load_handles))
        .observe(create)
        .observe(on_initial_component)
        .observe(on_create_single_component)
        .observe(on_create_component)
        .observe(delete_component)
        .observe(update_label)
        .observe(remove_all)
        .observe(update_file)
        .run();
}

fn cancel_action(
    mut commands: Commands,
    q_first: Query<Entity, With<FirstPos>>,
    selected: Query<Entity, With<Selected>>,
) {
    if let Ok(ent) = q_first.get_single() {
        commands.trigger_targets(DeleteComponent, ent);
    }

    if let Ok(ent) = selected.get_single() {
        commands.entity(ent).remove::<Selected>();
    }
}

fn get_cursor_position(
    mut cursor: ResMut<CursorPosition>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor.0 = world_position.into()
    }
}

fn middle(mut commands: Commands) {
    commands.trigger(DeleteAll);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Rendering default nodes
    const OFFSET: f32 = GRID_COUNT as f32 * GRID_SIZE / 2.0;
    for x_i in 0..GRID_COUNT {
        for y_i in 0..GRID_COUNT {
            let x = x_i as f32 * GRID_SIZE - OFFSET + 160.;
            let y = y_i as f32 * GRID_SIZE - OFFSET;
            commands
                .spawn(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::splat(1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(TikzNode {
                    label: "".to_string(),
                });
        }
    }
}
