use crate::*;
use bevy::{
    render::render_asset::RenderAssetUsages,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::hashbrown::HashMap,
};

pub struct Components;
impl Components {
    const TEXT_SCALE: f32 = 0.6;
    const CIRCLE_RADIUS: f32 = 0.5;
    const CIRCLE_RESOLUTION: usize = 500;
    const RESISTOR: [[f32; 3]; 18] = [
        [-0.5, 0.0, 0.0],
        [-0.375, 0.0, 0.0],
        [-0.325, 0.25, 0.0],
        [-0.275, 0.0, 0.0],
        [-0.225, -0.25, 0.0],
        [-0.175, 0.0, 0.0],
        [-0.125, 0.25, 0.0],
        [-0.075, 0.0, 0.0],
        [-0.025, -0.25, 0.0],
        [0.025, 0.0, 0.0],
        [0.075, 0.25, 0.0],
        [0.125, 0.0, 0.0],
        [0.175, -0.25, 0.0],
        [0.225, 0.0, 0.0],
        [0.275, 0.25, 0.0],
        [0.325, 0.0, 0.0],
        [0.375, 0.0, 0.0],
        [0.5, 0.0, 0.0],
    ];

    const CAPACITOR: [[f32; 3]; 8] = [
        [-0.5, 0.0, 0.0],
        [-0.1, 0.0, 0.0],
        [-0.1, 0.5, 0.0],
        [-0.1, -0.5, 0.0],
        [0.5, 0.0, 0.0],
        [0.1, 0.0, 0.0],
        [0.1, 0.5, 0.0],
        [0.1, -0.5, 0.0],
    ];

    const GROUND: [[f32; 3]; 8] = [
        [0.0, 0.0, 0.0],
        [0.0, -0.20, 0.0],
        [-0.4, -0.20, 0.0],
        [0.4, -0.20, 0.0],
        [-0.25, -0.35, 0.0],
        [0.25, -0.35, 0.0],
        [-0.1, -0.50, 0.0],
        [0.1, -0.50, 0.0],
    ];

    const PLUS: [[f32; 3]; 4] = [
        [-0.15, 0.0, 0.0],
        [0.15, 0.0, 0.0],
        [0.0, 0.15, 0.0],
        [0.0, -0.15, 0.0],
    ];

    const MINUS: [[f32; 3]; 2] = [[0.0, 0.15, 0.0], [0.0, -0.15, 0.0]];

    const ARROW: [[f32; 3]; 6] = [
        [-0.2, 0.0, 0.0],
        [0.2, 0.0, 0.0],
        [0.08, 0.12, 0.0],
        [0.2, 0.0, 0.0],
        [0.08, -0.12, 0.0],
        [0.2, 0.0, 0.0],
    ];

    pub fn create_with_mesh(
        commands: &mut Commands,
        handles: Res<Handles>,
        initial: Position,
        fin: Position,
        comp_type: TikzComponent,
    ) -> Entity {
        let pos = (initial + fin) / 2.0;
        let len = (fin - initial).len();
        let angle = (fin.y - initial.y).atan2(fin.x - initial.x);
        const SIZE: f32 = GRID_SIZE * 1.5;
        info!("Creating mesh of type {comp_type} with pos ({pos}) and angle: {angle}");
        commands
            .spawn(SpatialBundle::from_transform(Transform {
                translation: pos.into(),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            }))
            .insert(ComponentInfo::default())
            .insert(TikzNode {
                label: comp_type.to_string(),
            })
            .with_children(|p| {
                p.spawn(Text2dBundle {
                    text: Text::default().with_justify(JustifyText::Center),
                    transform: Transform::from_xyz(0.0, SIZE, 0.0)
                        .with_scale(Vec3::splat(Components::TEXT_SCALE)),
                    ..default()
                });
                let Some(info) = handles.0.get(&comp_type) else {
                    warn!("No handle found for type {comp_type}");
                    return;
                };
                let Info { meshes, material } = info;
                for mesh in meshes.iter() {
                    p.spawn(MaterialMesh2dBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform {
                            scale: Vec3::new(SIZE, SIZE, 1.0),
                            ..default()
                        },
                        ..default()
                    });
                }
                if len > SIZE && !comp_type.is_gate() {
                    let half_line = (len - SIZE) / 2.0;
                    let offset = (SIZE + half_line) / 2.0;
                    let width = 0.5;
                    p.spawn(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(-offset, 0.0, 0.0),
                            scale: Vec3::new(half_line, width, 1.0),
                            ..default()
                        },
                        ..default()
                    });
                    p.spawn(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(offset, 0.0, 0.0),
                            scale: Vec3::new(half_line, width, 1.0),
                            ..default()
                        },
                        ..default()
                    });
                }
            })
            .id()
    }

    pub fn create_gate(
        commands: &mut Commands,
        handles: Res<Handles>,
        initial: Position,
        fin: Position,
        comp_type: TikzComponent,
    ) -> Entity {
        assert!(
            comp_type.is_gate(),
            "Function create_gate is supposed to be used with gate type"
        );

        let gate = Components::create_with_mesh(commands, handles, initial, fin, comp_type);
        use TikzComponent::*;
        match comp_type {
            AndGate | OrGate | XorGate => {
                let in1 = commands
                    .spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(-2.0 * GRID_SIZE, GRID_SIZE, 0.0)
                                .with_scale(Vec3::splat(1.0)),
                            ..default()
                        },
                        TikzNode {
                            label: ".in 1".to_string(),
                        },
                    ))
                    .id();

                let in2 = commands
                    .spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(-2.0 * GRID_SIZE, -GRID_SIZE, 0.0)
                                .with_scale(Vec3::splat(1.0)),
                            ..default()
                        },
                        TikzNode {
                            label: ".in 2".to_string(),
                        },
                    ))
                    .id();
                let out = commands
                    .spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(2.0 * GRID_SIZE, 0.0, 0.0)
                                .with_scale(Vec3::splat(1.0)),
                            ..default()
                        },
                        TikzNode {
                            label: ".out".to_string(),
                        },
                    ))
                    .id();
                info!("{in1} {in2} {out}");

                commands.entity(gate).push_children(&[in1, in2, out]);
            }

            NotGate => {}
            _ => unreachable!("Only gate type"),
        }
        gate
    }

    fn create_circle() -> Vec<[f32; 3]> {
        Components::create_circle_from_radius(Self::CIRCLE_RADIUS, Position { x: 0.0, y: 0.0 })
    }

    fn create_circle_from_radius(radius: f32, p: Position) -> Vec<[f32; 3]> {
        let mut circle = Vec::with_capacity(Self::CIRCLE_RESOLUTION);
        for i in 0..Self::CIRCLE_RESOLUTION {
            let x = radius * (i as f32).cos() + p.x;
            let y = radius * (i as f32).sin() + p.y;

            circle.push([x, y, 0.0]);
        }
        circle
    }

    fn create_arc(p0: Vec3, p1: Vec3, p2: Vec3) -> Vec<[f32; 3]> {
        let mut arc = Vec::with_capacity(Self::CIRCLE_RESOLUTION);
        for i in 0..Self::CIRCLE_RESOLUTION {
            let t = i as f32 / Self::CIRCLE_RESOLUTION as f32;
            let point = p0 * (1.0 - t).powi(2) + p1 * 2.0 * (1.0 - t) * t + p2 * t.powi(2);
            arc.push(point.into());
        }
        arc
    }

    fn create_coil(coils: usize) -> Vec<[f32; 3]> {
        let size = 500;
        let mut circle = Vec::with_capacity(coils * size);
        let radius = 1f32 / coils as f32;
        for current in [-1.5 * radius, -0.5 * radius, 0.5 * radius, 1.5 * radius] {
            for i in 0..size {
                let x = current + radius * (i as f32).cos() * 2f32 / coils as f32;
                let y = radius * (i as f32).sin() * 3f32 / coils as f32 * 1.35;
                if y < f32::EPSILON {
                    continue;
                }

                circle.push([x, y, 0.0]);
            }
        }
        circle
    }

    pub fn create_line(
        commands: &mut Commands,
        pos: Position,
        angle: f32,
        color: Color,
        len: f32,
    ) -> Entity {
        commands
            .spawn(SpatialBundle::from_transform(Transform {
                translation: pos.into(),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            }))
            .insert(ComponentInfo::default())
            .with_children(|p| {
                p.spawn(SpriteBundle {
                    transform: Transform {
                        scale: Vec3::new(len, 0.5, 1.0),
                        ..default()
                    },
                    sprite: Sprite { color, ..default() },
                    ..default()
                });
                p.spawn(Text2dBundle {
                    text: Text::default().with_justify(JustifyText::Center),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.75 * GRID_SIZE, 0.0))
                        .with_scale(Vec3::splat(Components::TEXT_SCALE)),
                    ..default()
                });
            })
            .id()
    }

    pub fn create_dot(commands: &mut Commands, pos: Position, color: Color, scale: Vec3) -> Entity {
        commands
            .spawn(SpatialBundle::from_transform(Transform {
                translation: pos.into(),
                ..default()
            }))
            .insert(TikzComponent::Dot)
            .insert(pos)
            .insert(ComponentInfo {
                label: "".to_string(),
                ..default()
            })
            .insert(BuildInfo {
                angle: 0.0,
                len: 0.0,
            })
            .with_children(|p| {
                p.spawn(SpriteBundle {
                    transform: Transform { scale, ..default() },
                    sprite: Sprite { color, ..default() },
                    ..default()
                });
                p.spawn(Text2dBundle {
                    text: Text::default().with_justify(JustifyText::Center),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.75 * GRID_SIZE, 0.0))
                        .with_scale(Vec3::splat(Components::TEXT_SCALE)),
                    ..default()
                });
            })
            .id()
    }
}

#[derive(Resource)]
pub struct Handles(pub HashMap<TikzComponent, Info>);

pub struct Info {
    meshes: Vec<Mesh2dHandle>,
    material: Handle<ColorMaterial>,
}

pub fn load_handles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use bevy::render::mesh::PrimitiveTopology as Topology;
    let mut map = HashMap::new();
    let material = materials.add(Color::WHITE);
    {
        // Resistor
        let mut mesh = Mesh::new(Topology::LineStrip, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Components::RESISTOR.to_vec());
        let mesh = vec![meshes.add(mesh).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::Resistor, info);
    }

    {
        // Capacitor
        let mut mesh = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Components::CAPACITOR.to_vec());
        let mesh = vec![meshes.add(mesh).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::Capacitor, info);
    }
    {
        // Inductor
        let mut mesh = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Components::create_coil(4));
        let mesh = vec![meshes.add(mesh).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::Inductor, info);
    }
    {
        // VSource
        let mut circ = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD);
        circ.insert_attribute(Mesh::ATTRIBUTE_POSITION, Components::create_circle());
        let mut plus = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        plus.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            Components::PLUS
                .iter()
                .map(|[mut x, y, z]| {
                    x += 0.1666;
                    [x, *y, *z]
                })
                .collect::<Vec<_>>(),
        );
        let mut minus = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        minus.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            Components::MINUS
                .iter()
                .map(|[mut x, y, z]| {
                    x -= 0.1666;
                    [x, *y, *z]
                })
                .collect::<Vec<_>>(),
        );
        let mesh = vec![
            meshes.add(plus).into(),
            meshes.add(circ).into(),
            meshes.add(minus).into(),
        ];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::VSource, info);
    }
    {
        // ISource
        let mut circ = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD);
        circ.insert_attribute(Mesh::ATTRIBUTE_POSITION, Components::create_circle());
        let mut arrow = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        arrow.insert_attribute(Mesh::ATTRIBUTE_POSITION, Components::ARROW.to_vec());
        let mesh = vec![meshes.add(circ).into(), meshes.add(arrow).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::ISource, info);
    }
    {
        // Ground
        let mut mesh = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Components::GROUND.to_vec());
        let mesh = vec![meshes.add(mesh).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::Ground, info);
    }

    {
        // AND PORT
        let l = vec![
            [0.5, 1.0, 0.0],
            [-0.6666, 1.0, 0.0],
            [-0.6666, -1.0, 0.0],
            [0.5, -1.0, 0.0],
        ];
        let arc = Components::create_arc(
            l[0].into(),
            Vec3::new(2.0 - l[0][0], 0.0, 0.0),
            l.last().copied().unwrap().into(),
        );
        let line = Mesh::new(Topology::LineStrip, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, l);
        let arc = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, arc);
        let lines = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    [-0.6666, 0.6666, 0.0],
                    [-1.3333, 0.6666, 0.0],
                    [-0.6666, -0.6666, 0.0],
                    [-1.3333, -0.6666, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.3333, 0.0, 0.0],
                ],
            );
        let mesh = vec![
            meshes.add(line).into(),
            meshes.add(arc).into(),
            meshes.add(lines).into(),
        ];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::AndGate, info);
    }

    {
        // OR PORT
        let l = [[-0.6666, 1.0, 0.0], [-0.6666, -1.0, 0.0]];
        let right = Vec3::new(1., 0.0, 0.0);
        let mut arc = Components::create_arc(l[0].into(), Vec3::new(0.6666, 1., 0.0), right);
        let mut lower_arc = Components::create_arc(l[1].into(), Vec3::new(0.6666, -1., 0.0), right);
        let mut back_arc =
            Components::create_arc(l[0].into(), Vec3::new(0.0, 0.0, 0.0), l[1].into());
        arc.append(&mut back_arc);
        arc.append(&mut lower_arc);
        let arcs = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, arc);
        let lines = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    [-0.5, 0.6666, 0.0],
                    [-1.3333, 0.6666, 0.0],
                    [-0.5, -0.6666, 0.0],
                    [-1.3333, -0.6666, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.3333, 0.0, 0.0],
                ],
            );
        let mesh = vec![meshes.add(arcs).into(), meshes.add(lines).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::OrGate, info);
    }

    {
        // NOT PORT
        let radius = 1. / 6.;
        let lines = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    // Triangle
                    [-0.6666, 0.6666, 0.0],
                    [0.6666, 0.0, 0.0],
                    //
                    [-0.6666, -0.6666, 0.0],
                    [0.6666, 0.0, 0.0],
                    //
                    [-0.6666, 0.6666, 0.0],
                    [-0.6666, -0.6666, 0.0],
                    // front line
                    [0.6666 + 2. * radius, 0.0, 0.0],
                    [1.3333, 0.0, 0.0],
                    // Back line
                    [-0.6666, 0.0, 0.0],
                    [-1.3333, 0.0, 0.0],
                ],
            );
        let circle = Components::create_circle_from_radius(
            radius,
            Position {
                x: 1.0 - radius,
                y: 0.0,
            },
        );
        let circle = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, circle);
        let mesh = vec![meshes.add(lines).into(), meshes.add(circle).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::NotGate, info);
    }

    {
        // XOR PORT
        let l = [[-0.6666, 1.0, 0.0], [-0.6666, -1.0, 0.0]];
        let right = Vec3::new(1., 0.0, 0.0);
        let mut arc = Components::create_arc(l[0].into(), Vec3::new(0.6666, 1., 0.0), right);
        let mut lower_arc = Components::create_arc(l[1].into(), Vec3::new(0.6666, -1., 0.0), right);
        let mut back_arc =
            Components::create_arc(l[0].into(), Vec3::new(0.0, 0.0, 0.0), l[1].into());
        arc.append(&mut back_arc);
        arc.append(&mut lower_arc);
        let mut back_arc = Components::create_arc(
            Vec3::from_array(l[0]) + Vec3::new(-0.2, 0.0, 0.0),
            Vec3::new(-0.2, 0.0, 0.0),
            Vec3::from_array(l[1]) + Vec3::new(-0.2, 0.0, 0.0),
        );
        arc.append(&mut back_arc);
        let arcs = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, arc);
        let lines = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    [-0.5, 0.6666, 0.0],
                    [-1.3333, 0.6666, 0.0],
                    [-0.5, -0.6666, 0.0],
                    [-1.3333, -0.6666, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.3333, 0.0, 0.0],
                ],
            );
        let mesh = vec![meshes.add(arcs).into(), meshes.add(lines).into()];
        let info = Info {
            meshes: mesh,
            material: material.clone(),
        };
        map.insert(TikzComponent::XorGate, info);
    }
    commands.insert_resource(Handles(map));
}
