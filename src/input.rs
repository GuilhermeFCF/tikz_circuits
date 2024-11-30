use bevy::app::AppExit;
use components::{Components, Handles};
use structs::{
    BuildInfo, CreateComponent, CreateSingleComponent, DeleteAll, DeleteComponent, FirstPos,
    InitiateComponent, Position, Selected,
};

use crate::*;

pub fn handle_left_click(
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
    marked_node: Query<(Entity, &GlobalTransform), With<Marker>>,
    components: Query<(Entity, &GlobalTransform), With<TikzComponent>>,
    selected: Query<(Entity, &GlobalTransform), With<Selected>>,
) {
    if !cursor_position.within_grid() {
        return;
    }
    if let Ok((node_entity, node)) = marked_node.get_single() {
        let node_pos = node.translation();
        if let Ok(selected) = selected.get_single() {
            commands.entity(selected.0).remove::<Selected>();
            if node_pos == selected.1.translation() {
                return;
            }
            for (component_entity, component_transform) in &components {
                if component_transform.translation() == node_pos {
                    commands.entity(component_entity).insert(Selected);
                    return;
                }
            }
        } else {
            for (component_entity, component_transform) in &components {
                if component_transform.translation() == node_pos {
                    commands.entity(component_entity).insert(Selected);
                    return;
                }
            }
        }
        commands.trigger(InitiateComponent { pos: node_entity })
    }
}

pub fn on_initial_component(
    trigger: Trigger<InitiateComponent>,
    mut commands: Commands,
    dots: Query<Entity, With<FirstPos>>,
    cc: Res<TikzComponent>,
    transform_query: Query<&GlobalTransform>,
) {
    let InitiateComponent { pos } = trigger.event();
    let transform = transform_query.get(*pos).unwrap();
    if let Ok(dot) = dots.get_single() {
        let dot_pos = transform_query.get(dot).unwrap();
        if transform == dot_pos {
            return;
        }
        commands.entity(dot).remove::<FirstPos>();
        commands.trigger(CreateComponent {
            initial: dot,
            fin: *pos,
        });
        commands.trigger(ConvertCircuit);
    } else if cc.is_single() {
        commands.trigger(CreateSingleComponent { node: *pos });
        commands.trigger(ConvertCircuit);
    } else {
        commands.entity(*pos).insert(FirstPos);
    }
}

pub fn on_create_single_component(
    trigger: Trigger<CreateSingleComponent>,
    mut commands: Commands,
    cc: Res<TikzComponent>,
    handles: Res<Handles>,
    transform_query: Query<&GlobalTransform>,
) {
    use TikzComponent::*;
    let CreateSingleComponent { node } = trigger.event();
    let pos = Position::from(transform_query.get(*node).unwrap().translation());
    let id = match *cc {
        Dot => Components::create_dot(
            &mut commands,
            pos,
            Color::Srgba(Srgba::RED),
            Vec3::new(8.0, 8.0, 1.0),
        ),
        Ground => Components::create_with_mesh(&mut commands, handles, pos, pos, *cc),
        x if x.is_gate() => Components::create_gate(&mut commands, handles, pos, pos, *cc),
        _ => unreachable!(),
    };
    commands
        .entity(id)
        .insert(pos)
        .insert(*cc)
        .insert(ComponentStructure::Node(*node))
        .insert(BuildInfo {
            angle: 0.0,
            len: 0.0,
        });
}

pub fn on_create_component(
    trigger: Trigger<CreateComponent>,
    mut commands: Commands,
    cc: Res<TikzComponent>,
    handles: Res<Handles>,
    transform_query: Query<&GlobalTransform>,
) {
    let CreateComponent { initial, fin } = trigger.event();
    let initial_pos = Position::from(transform_query.get(*initial).unwrap().translation());
    let final_pos = Position::from(transform_query.get(*fin).unwrap().translation());
    let middle = (initial_pos + final_pos) / 2.0;
    let len = (final_pos - initial_pos).len();
    let angle = (final_pos.y - initial_pos.y).atan2(final_pos.x - initial_pos.x);
    let component = match *cc {
        TikzComponent::Line => {
            Components::create_line(&mut commands, middle, angle, Color::WHITE, len)
        }
        _ => Components::create_with_mesh(&mut commands, handles, initial_pos, final_pos, *cc),
    };
    commands
        .entity(component)
        .insert(*cc)
        .insert(middle)
        .insert(ComponentStructure::To([*initial, *fin]))
        .insert(BuildInfo { angle, len });
}

pub fn despawn_selected(mut commands: Commands, selected: Query<Entity, With<Selected>>) {
    if let Ok(entity) = selected.get_single() {
        commands.trigger_targets(DeleteComponent, entity);
    }
}

type Filter = (Without<FirstPos>, With<TikzComponent>);
pub fn remove_all(_: Trigger<DeleteAll>, mut commands: Commands, q_points: Query<Entity, Filter>) {
    if q_points.is_empty() {
        return;
    }
    let entities: Vec<_> = q_points.into_iter().collect();
    commands.trigger_targets(DeleteComponent, entities)
}

#[allow(clippy::too_many_arguments)]
pub fn change_current_component(
    mut egui_context: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    mut cc: ResMut<TikzComponent>,
    mut exit: EventWriter<AppExit>,
) {
    if egui_context.ctx_mut().wants_keyboard_input() {
        return;
    }
    let Some(key_map) = keys.get_just_pressed().next() else {
        return;
    };

    if *key_map == KeyCode::KeyQ {
        exit.send(AppExit::Success);
        return;
    }

    *cc = match key_map {
        KeyCode::KeyW => TikzComponent::Line,
        KeyCode::KeyR => TikzComponent::Resistor,
        KeyCode::KeyC => TikzComponent::Capacitor,
        KeyCode::KeyL => TikzComponent::Inductor,
        KeyCode::KeyV => TikzComponent::VSource,
        KeyCode::KeyI => TikzComponent::ISource,
        KeyCode::KeyP => TikzComponent::Dot,
        KeyCode::KeyG => TikzComponent::Ground,
        _ => return,
    };
}
