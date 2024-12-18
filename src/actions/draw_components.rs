use crate::components::InfoMeshes;
use bevy::prelude::*;

use crate::components::Handles;
use crate::graph::UpdateGraph;
use crate::{structs::*, TEXT_SCALE};

use crate::GRID_SIZE;

fn label(height: f32) -> impl Bundle {
    (
        Text2d::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(0.0, height * GRID_SIZE, 0.0).with_scale(Vec3::splat(TEXT_SCALE)),
    )
}

fn initial_component(comp_type: TikzComponent, translation: Vec3, angle: f32) -> impl Bundle {
    (
        comp_type,
        Visibility::default(),
        Transform::from_translation(translation).with_rotation(Quat::from_rotation_z(angle)),
        Selectable,
        ComponentLabel {
            label: "".to_string(),
        },
        Info::default(),
    )
}
#[derive(Event)]
pub struct InitiateComponent {
    pub ent: Entity,
}

pub fn draw_initial_component(
    trigger: Trigger<InitiateComponent>,
    mut commands: Commands,
    dots: Query<Entity, With<FirstPos>>,
    cc: Res<TikzComponent>,
    transform_query: Query<&GlobalTransform>,
    handles: Res<Handles>,
    material: ResMut<Assets<ColorMaterial>>,
) {
    let InitiateComponent { ent } = trigger.event();
    let transform = transform_query.get(*ent).unwrap();

    let text_height = cc.get_label_height();
    let initial = transform_query.get(*ent).unwrap().translation();
    match *cc {
        TikzComponent::Dot => {
            commands
                .spawn(initial_component(*cc, initial, 0.0))
                .with_children(|p| {
                    p.spawn(label(text_height));

                    p.spawn(Sprite::from_color(
                        Color::Srgba(Srgba::gray(0.5)),
                        Vec2::splat(4.0),
                    ));
                });
            return;
        }
        x if x.is_single() => {
            draw_from_mesh(&mut commands, *cc, handles, material, initial, initial);
            return;
        }
        _ => {}
    }

    let Ok(dot_ent) = dots.get_single() else {
        commands.entity(*ent).insert(FirstPos);
        return;
    };

    let dot_transform = transform_query.get(dot_ent).unwrap();
    commands.entity(dot_ent).remove::<FirstPos>();

    // NOTE: Could add aditional requirements, like having a certain distance for a certain type
    // of component
    if transform == dot_transform {
        return;
    }

    let initial = transform_query.get(dot_ent).unwrap().translation();
    let fin = transform_query.get(*ent).unwrap().translation();
    info!("{ent}, {dot_ent}");
    let middle = (initial + fin) / 2.0;
    let len = (fin - initial).length();
    let angle = (fin.y - initial.y).atan2(fin.x - initial.x);
    info!("{fin}, {middle}, {len}, {angle}");
    if *cc == TikzComponent::Line {
        commands
            .spawn(initial_component(*cc, middle, angle))
            .with_children(|p| {
                p.spawn(label(text_height));

                p.spawn((
                    Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    Transform::from_scale(Vec3::new(len, 0.5, 1.0)),
                ));
            });
        return;
    }
    draw_from_mesh(&mut commands, *cc, handles, material, initial, fin);
}

pub fn draw_from_mesh(
    commands: &mut Commands,
    cc: TikzComponent,
    handles: Res<Handles>,
    mut material: ResMut<Assets<ColorMaterial>>,
    initial: Vec3,
    fin: Vec3,
) {
    const SIZE: f32 = GRID_SIZE * 1.5;
    let middle = (initial + fin) / 2.0;
    let len = (fin - initial).length();
    let angle = (fin.y - initial.y).atan2(fin.x - initial.x);
    let text_height = cc.get_label_height();
    let structure = match cc {
        x if x.is_single() => ComponentStructure::Node(initial.truncate()),
        _ => ComponentStructure::To([initial.truncate(), fin.truncate()]),
    };
    let component = commands
        .spawn((
            initial_component(cc, middle, angle),
            BuildInfo::new(angle, len),
            structure,
            Anchored(initial.truncate()),
        ))
        .with_children(|p| {
            p.spawn(label(text_height));

            let InfoMeshes { meshes } = handles.0.get(&cc).unwrap();
            for mesh in meshes.iter() {
                p.spawn((
                    mesh.clone(),
                    MeshMaterial2d(material.add(Color::WHITE)),
                    Transform::from_scale(Vec3::new(SIZE, SIZE, 1.0)),
                ));
            }

            if len > SIZE && !cc.is_single() {
                let half_line = (len - SIZE) / 2.0;
                let offset = (SIZE + half_line) / 2.0;
                let width = 0.5;
                p.spawn((
                    Sprite::default(),
                    Transform::from_xyz(-offset, 0.0, 0.0)
                        .with_scale(Vec3::new(half_line, width, 1.0)),
                ));

                p.spawn((
                    Sprite::default(),
                    Transform::from_xyz(offset, 0.0, 0.0)
                        .with_scale(Vec3::new(half_line, width, 1.0)),
                ));
            }
        })
        .id();
    if cc.is_gate() {
        fill_gate_labels(component, commands, cc);
    }

    if cc == TikzComponent::AmpOp {
        fill_amp_labels(component, commands);
    }
    commands.trigger(UpdateGraph(structure, component));
}

fn fill_amp_labels(amp: Entity, commands: &mut Commands) {
    let minus_in = commands
        .spawn((
            Transform::from_xyz(-2.0 * GRID_SIZE, GRID_SIZE, 0.0),
            ComponentLabel {
                label: ".-".to_string(),
            },
        ))
        .id();
    let plus_in = commands
        .spawn((
            Transform::from_xyz(-2.0 * GRID_SIZE, -GRID_SIZE, 0.0),
            ComponentLabel {
                label: ".+".to_string(),
            },
        ))
        .id();
    let out = commands
        .spawn((
            Transform::from_xyz(3.0 * GRID_SIZE, 0., 0.0),
            ComponentLabel {
                label: ".out".to_string(),
            },
        ))
        .id();
    commands.entity(amp).add_children(&[minus_in, plus_in, out]);
}
fn fill_gate_labels(gate: Entity, commands: &mut Commands, comp_type: TikzComponent) {
    use TikzComponent::*;
    match comp_type {
        AndGate | OrGate | XorGate => {
            let in1 = commands
                .spawn((
                    Transform::from_xyz(-2.0 * GRID_SIZE, GRID_SIZE, 1.0),
                    ComponentLabel {
                        label: ".in 1".to_string(),
                    },
                ))
                .id();

            let in2 = commands
                .spawn((
                    Transform::from_xyz(-2.0 * GRID_SIZE, -GRID_SIZE, 1.0),
                    ComponentLabel {
                        label: ".in 2".to_string(),
                    },
                ))
                .id();

            let out = commands
                .spawn((
                    Transform::from_xyz(2.0 * GRID_SIZE, 0.0, 1.0),
                    ComponentLabel {
                        label: ".out".to_string(),
                    },
                ))
                .id();
            info!("{in1} {in2} {out}");

            commands.entity(gate).add_children(&[in1, in2, out]);
        }

        NotGate => {
            let in1 = commands
                .spawn((
                    Transform::from_xyz(-1.0 * GRID_SIZE, -GRID_SIZE, 1.0),
                    ComponentLabel {
                        label: ".in".to_string(),
                    },
                ))
                .id();

            let out = commands
                .spawn((
                    Transform::from_xyz(1.0 * GRID_SIZE, 0.0, 1.0),
                    ComponentLabel {
                        label: ".out".to_string(),
                    },
                ))
                .id();
            commands.entity(gate).add_children(&[in1, out]);
        }
        _ => unreachable!("Only gate type"),
    }
}
