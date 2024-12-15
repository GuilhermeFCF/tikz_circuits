use bevy::app::AppExit;
use graph::UpdateGraph;
use structs::{
    BuildInfo, CreateComponent, CreateSingleComponent, DeleteAll, FirstPos, InitiateComponent,
    Selected,
};

use crate::*;

fn close_to(pos: Vec2, other_pos: Vec2) -> bool {
    pos.distance(other_pos) < GRID_SIZE
}

pub fn handle_left_click(
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
    marked_node: Single<Entity, With<Marker>>,
    selectable: Query<(Entity, &GlobalTransform), With<Selectable>>,
    selected: Query<(Entity, &GlobalTransform), With<Selected>>,
) {
    if !cursor_position.within_grid {
        return;
    }
    let node_entity = *marked_node;
    let cursor = cursor_position.pos;
    if let Ok((selected_entity, selected_transform)) = selected.get_single() {
        commands.entity(selected_entity).remove::<Selected>();
        let selected_pos = selected_transform.translation().truncate();
        if close_to(cursor, selected_pos) {
            return;
        }
    }
    for (ent, transform) in &selectable {
        let selected_pos = transform.translation().truncate();
        if close_to(cursor, selected_pos) {
            commands.entity(ent).insert(Selected);
            return;
        }
    }
    commands.trigger(InitiateComponent { ent: node_entity })
}

pub fn on_initial_component(
    trigger: Trigger<InitiateComponent>,
    mut commands: Commands,
    dots: Query<Entity, With<FirstPos>>,
    cc: Res<TikzComponent>,
    transform_query: Query<&GlobalTransform>,
) {
    let InitiateComponent { ent } = trigger.event();
    let transform = transform_query.get(*ent).unwrap();

    if cc.is_single() {
        commands.trigger(CreateSingleComponent { node: *ent });
        return;
    }

    let Ok(dot_ent) = dots.get_single() else {
        commands.entity(*ent).insert(FirstPos);
        return;
    };

    let dot_pos = transform_query.get(dot_ent).unwrap();
    commands.entity(dot_ent).remove::<FirstPos>();

    if transform == dot_pos {
        return;
    }

    commands.trigger(CreateComponent::new(dot_ent, *ent));
}

pub fn on_create_single_component(
    trigger: Trigger<CreateSingleComponent>,
    mut commands: Commands,
    cc: Res<TikzComponent>,
    handles: Res<Handles>,
    material: ResMut<Assets<ColorMaterial>>,
    transform_query: Query<&GlobalTransform>,
) {
    use TikzComponent::*;
    let CreateSingleComponent { node } = trigger.event();
    let pos = transform_query.get(*node).unwrap().translation();
    let id = match *cc {
        Dot => create_dot(
            &mut commands,
            pos,
            Color::Srgba(Srgba::gray(0.5)),
            Vec3::new(6.0, 6.0, 1.0),
        ),
        Ground => create_with_mesh(
            &mut commands,
            handles,
            material,
            pos,
            pos,
            *cc,
            1.5 * GRID_SIZE,
        ),
        x if x.is_gate() => create_with_mesh(
            &mut commands,
            handles,
            material,
            pos,
            pos,
            *cc,
            2f32 * GRID_SIZE,
        ),
        _ => unreachable!(),
    };
    let structure = ComponentStructure::Node(pos.truncate());
    let component_entity = commands
        .entity(id)
        .insert((
            *cc,
            Selectable,
            structure,
            ComponentLabel {
                label: "".to_string(),
            },
            Anchored(pos.truncate()),
        ))
        .id();
    commands.trigger(UpdateGraph(structure, component_entity));
}

pub fn on_create_component(
    trigger: Trigger<CreateComponent>,
    mut commands: Commands,
    cc: Res<TikzComponent>,
    handles: Res<Handles>,
    material: ResMut<Assets<ColorMaterial>>,
    transform_query: Query<&GlobalTransform>,
) {
    let CreateComponent { initial, fin } = trigger.event();
    let initial_pos = transform_query.get(*initial).unwrap().translation();
    let final_pos = transform_query.get(*fin).unwrap().translation();
    let middle = (initial_pos + final_pos) / 2.0;
    let len = (final_pos - initial_pos).length();
    let angle = (final_pos.y - initial_pos.y).atan2(final_pos.x - initial_pos.x);
    let component = match *cc {
        x if x.is_single() => return,
        TikzComponent::Line => create_line(&mut commands, middle, angle, len),
        _ => create_with_mesh(
            &mut commands,
            handles,
            material,
            initial_pos,
            final_pos,
            *cc,
            1.5 * GRID_SIZE,
        ),
    };
    let structure = ComponentStructure::To([initial_pos.truncate(), final_pos.truncate()]);
    let component_entity = commands
        .entity(component)
        .insert((
            *cc,
            Selectable,
            structure,
            ComponentLabel {
                label: "".to_string(),
            },
            Anchored(initial_pos.truncate()),
            BuildInfo::new(angle, len),
        ))
        .id();
    commands.trigger(UpdateGraph(structure, component_entity));
}

type Filter = (Without<FirstPos>, With<TikzComponent>);
pub fn remove_all(_: Trigger<DeleteAll>, mut commands: Commands, q_points: Query<Entity, Filter>) {
    if q_points.is_empty() {
        return;
    }
    let entities: Vec<_> = q_points.into_iter().collect();
    commands.trigger_targets(DeleteComponent, entities)
}

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
