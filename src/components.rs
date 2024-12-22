use crate::structs::TikzComponent;
use bevy::prelude::*;
use bevy::{render::render_asset::RenderAssetUsages, utils::hashbrown::HashMap};

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

fn draw_circle_from_radius(radius: f32, p: Vec2) -> Vec<[f32; 3]> {
    let mut circle = Vec::with_capacity(CIRCLE_RESOLUTION);
    for i in 0..CIRCLE_RESOLUTION {
        let x = radius * (i as f32).cos() + p.x;
        let y = radius * (i as f32).sin() + p.y;

        circle.push([x, y, 0.0]);
    }
    circle
}

fn draw_arc(p0: Vec3, p1: Vec3, p2: Vec3) -> Vec<[f32; 3]> {
    let mut arc = Vec::with_capacity(CIRCLE_RESOLUTION);
    for i in 0..CIRCLE_RESOLUTION {
        let t = i as f32 / CIRCLE_RESOLUTION as f32;
        let point = p0 * (1.0 - t).powi(2) + p1 * 2.0 * (1.0 - t) * t + p2 * t.powi(2);
        arc.push(point.into());
    }
    arc
}

fn draw_coil() -> Vec<[f32; 3]> {
    let coils = 4;
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

#[derive(Resource)]
pub struct Handles(pub HashMap<TikzComponent, InfoMeshes>);

pub struct InfoMeshes {
    pub meshes: Vec<Mesh2d>,
}

pub fn load_handles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    use bevy::render::mesh::PrimitiveTopology as Topology;
    let mut map = HashMap::new();
    {
        // Resistor
        let mut mesh = Mesh::new(Topology::LineStrip, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, RESISTOR.to_vec());
        let mesh = vec![meshes.add(mesh).into()];
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::Resistor, info);
    }

    {
        // Capacitor
        let mut mesh = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, CAPACITOR.to_vec());
        let mesh = vec![meshes.add(mesh).into()];
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::Capacitor, info);
    }
    {
        // Inductor
        let mut mesh = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, draw_coil());
        let mesh = vec![meshes.add(mesh).into()];
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::Inductor, info);
    }
    {
        // VSource
        let circ = Mesh2d(meshes.add(Annulus::new(0.45, 0.5)));
        // let mut circ = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD);
        // circ.insert_attribute(Mesh::ATTRIBUTE_POSITION, create_circle());
        let mut plus = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        plus.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            PLUS.iter()
                .map(|[mut x, y, z]| {
                    x += 0.1666;
                    [x, *y, *z]
                })
                .collect::<Vec<_>>(),
        );
        let mut minus = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        minus.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            MINUS
                .iter()
                .map(|[mut x, y, z]| {
                    x -= 0.1666;
                    [x, *y, *z]
                })
                .collect::<Vec<_>>(),
        );
        let mesh = vec![meshes.add(plus).into(), circ, meshes.add(minus).into()];
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::VSource, info);
    }
    {
        // ISource
        let circ = Mesh2d(meshes.add(Annulus::new(0.45, 0.5)));

        let mut arrow = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        arrow.insert_attribute(Mesh::ATTRIBUTE_POSITION, ARROW.to_vec());
        let mesh = vec![circ, meshes.add(arrow).into()];
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::ISource, info);
    }
    {
        // Ground
        let mut mesh = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, GROUND.to_vec());
        let mesh = vec![meshes.add(mesh).into()];
        map.insert(TikzComponent::Ground, InfoMeshes { meshes: mesh });
    }

    {
        // AND PORT
        let l = vec![
            [0.5, 1.0, 0.0],
            [-0.6666, 1.0, 0.0],
            [-0.6666, -1.0, 0.0],
            [0.5, -1.0, 0.0],
        ];

        let a2: Vec<_> = CubicBezier::new([[
            Vec2::new(0.5, 1.0),
            Vec2::new(1.65 - l[0][0], 0.75),
            Vec2::new(1.65 - l[0][0], -0.75),
            Vec2::new(0.5, -1.0),
        ]])
        .to_curve()
        .unwrap()
        .iter_positions(100)
        .map(|e| Vec3::new(e.x, e.y, 0.0))
        .collect();
        let line = Mesh::new(Topology::LineStrip, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, l);
        let arc = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, a2);
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

        map.insert(TikzComponent::AndGate, InfoMeshes { meshes: mesh });
    }

    {
        // OR PORT
        let l = [[-0.6666, 1.0, 0.0], [-0.6666, -1.0, 0.0]];
        let right = Vec3::new(1., 0.0, 0.0);
        let mut arc = draw_arc(l[0].into(), Vec3::new(0.6666, 1., 0.0), right);
        let mut lower_arc = draw_arc(l[1].into(), Vec3::new(0.6666, -1., 0.0), right);
        let mut back_arc = draw_arc(l[0].into(), Vec3::new(0.0, 0.0, 0.0), l[1].into());
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
        map.insert(TikzComponent::OrGate, InfoMeshes { meshes: mesh });
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
        // let circ = Mesh2d(meshes.add(Annulus::new(0.45, 0.5)));
        let circle = draw_circle_from_radius(
            radius,
            Vec2 {
                x: 1.0 - radius,
                y: 0.0,
            },
        );
        let circle = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, circle);
        let mesh = vec![meshes.add(lines).into(), meshes.add(circle).into()];
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::NotGate, info);
    }

    {
        // XOR PORT
        let l = [[-0.6666, 1.0, 0.0], [-0.6666, -1.0, 0.0]];
        let right = Vec3::new(1., 0.0, 0.0);
        let mut upper_arc = draw_arc(l[0].into(), Vec3::new(0.6666, 1., 0.0), right);
        let mut lower_arc = draw_arc(l[1].into(), Vec3::new(0.6666, -1., 0.0), right);
        let mut back_arc = draw_arc(l[0].into(), Vec3::new(0.0, 0.0, 0.0), l[1].into());
        upper_arc.append(&mut back_arc);
        upper_arc.append(&mut lower_arc);
        let mut back_arc = draw_arc(
            Vec3::from_array(l[0]) + Vec3::new(-0.2, 0.0, 0.0),
            Vec3::new(-0.2, 0.0, 0.0),
            Vec3::from_array(l[1]) + Vec3::new(-0.2, 0.0, 0.0),
        );
        upper_arc.append(&mut back_arc);
        let arcs = Mesh::new(Topology::PointList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, upper_arc);
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
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::XorGate, info);
    }

    {
        // AMPOP
        let lines = Mesh::new(Topology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    // Triangle
                    [-0.6666, 1.3333, 0.0],
                    [1.3333, 0.0, 0.0],
                    //
                    [-0.6666, -1.3333, 0.0],
                    [1.3333, 0.0, 0.0],
                    //
                    [-0.6666, 1.3333, 0.0],
                    [-0.6666, -1.3333, 0.0],
                    // out line
                    [1.3333, 0.0, 0.0],
                    [2., 0.0, 0.0],
                    // - in
                    [-0.6666, 0.6666, 0.0],
                    [-1.3333, 0.6666, 0.0],
                    // + in
                    [-0.6666, -0.6666, 0.0],
                    [-1.3333, -0.6666, 0.0],
                    // - line
                    [-0.5, 0.5, 0.0],
                    [-0.3, 0.5, 0.0],
                    // + line
                    [-0.5, -0.5, 0.0],
                    [-0.3, -0.5, 0.0],
                    //
                    [-0.4, -0.4, 0.0],
                    [-0.4, -0.6, 0.0],
                ],
            );
        // let circ = Mesh2d(meshes.add(Annulus::new(0.45, 0.5)));
        let mesh = vec![meshes.add(lines).into()];
        let info = InfoMeshes { meshes: mesh };
        map.insert(TikzComponent::AmpOp, info);
    }
    commands.insert_resource(Handles(map));
}
