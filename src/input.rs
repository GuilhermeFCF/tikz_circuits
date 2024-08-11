use bevy::app::AppExit;
use components::{Components, Handles};
use structs::{
    BuildInfo, CreateComponent, CreateSingleComponent, DeleteAll, DeleteComponent, FirstPos,
    InitiateComponent, Position, Selected,
};

use crate::*;

pub fn handle_left_click(
    mut commands: Commands,
    cursor: Res<CursorPosition>,
    q_points: Query<(Entity, &Position, Has<Selected>), Without<FirstPos>>,
    selected: Query<Entity, With<Selected>>,
) {
    if !cursor.within_grid() {
        return;
    }

    let mut selecting = false;
    for (ent, pos, is_selected) in &q_points {
        if cursor.close_to(pos) {
            for sel in &selected {
                commands.entity(sel).remove::<Selected>();
            }
            if is_selected {
                commands.entity(ent).remove::<Selected>();
            } else {
                commands.entity(ent).insert(Selected);
            }
            selecting = true;
            break;
        }
    }
    if !selecting {
        commands.trigger(InitiateComponent { pos: **cursor });
    }
}

pub fn on_initial_component(
    trigger: Trigger<InitiateComponent>,
    mut commands: Commands,
    dots: Query<(Entity, &Position), With<FirstPos>>,
    cc: Res<TikzComponent>,
    rs: Res<State<RoundState>>,
) {
    let InitiateComponent { pos } = trigger.event();
    let pos = pos.round_on_state(&rs);
    if let Ok(dot) = dots.get_single() {
        if pos == *dot.1 {
            return;
        }
        commands.entity(dot.0).despawn_recursive();
        commands.trigger(CreateComponent {
            initial: *dot.1,
            fin: pos,
        });
        commands.trigger(ConvertCircuit);
    } else if cc.is_single() {
        commands.trigger(CreateSingleComponent { pos });
        commands.trigger(ConvertCircuit);
    } else {
        let dot = Components::create_dot(
            &mut commands,
            pos,
            Color::srgba(0.8, 0.5, 0.8, 1.0),
            Vec3::new(5.0, 5.0, 1.0),
        );
        commands.entity(dot).insert(FirstPos);
    }
}

pub fn on_create_single_component(
    trigger: Trigger<CreateSingleComponent>,
    mut commands: Commands,
    cc: Res<TikzComponent>,
    handles: Res<Handles>,
    tikz_nodes: ResMut<TikzNodes>,
) {
    use TikzComponent::*;
    let CreateSingleComponent { pos } = trigger.event();
    let id = match *cc {
        Dot => Components::create_dot(
            &mut commands,
            *pos,
            Color::Srgba(Srgba::RED),
            Vec3::new(8.0, 8.0, 1.0),
        ),
        Ground => Components::create_with_mesh(&mut commands, handles, *pos, *pos, *cc),
        x if x.is_gate() => {
            Components::create_gate(&mut commands, handles, *pos, *pos, *cc, tikz_nodes)
        }
        _ => unreachable!(),
    };
    commands
        .entity(id)
        .insert(*pos)
        .insert(*cc)
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
) {
    let CreateComponent { initial, fin } = trigger.event();
    let middle = (*initial + *fin) / 2.0;
    let len = (*fin - *initial).len();
    let angle = (fin.y - initial.y).atan2(fin.x - initial.x);
    let component = match *cc {
        TikzComponent::Line => {
            Components::create_line(&mut commands, middle, angle, Color::WHITE, len)
        }
        _ => Components::create_with_mesh(&mut commands, handles, *initial, *fin, *cc),
    };
    commands
        .entity(component)
        .insert(*cc)
        .insert(middle)
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

    mut next_round_state: ResMut<NextState<RoundState>>,
    rs: Res<State<RoundState>>,
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

    if *key_map == KeyCode::NumpadAdd {
        next_round_state.set(match rs.get() {
            RoundState::Round => RoundState::NoRound,
            RoundState::NoRound => RoundState::Round,
        });
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
