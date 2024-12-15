use bevy::{input::common_conditions::input_just_pressed, prelude::*, utils::HashMap};
use petgraph::{graph::NodeIndex, Graph, Undirected};

use crate::{ComponentStructure, ConvertCircuit, Position, GRID_COUNT, GRID_SIZE, OFFSET};

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ComponentGraph::default())
            .insert_resource(PositionToNodeIndex(HashMap::default()))
            .add_observer(update_graph)
            .add_observer(remove_from_graph)
            .add_systems(
                Startup,
                |mut graph: ResMut<ComponentGraph>, mut map: ResMut<PositionToNodeIndex>| {
                    const HALF_COUNT: isize = GRID_COUNT as isize / 2;
                    for x in -HALF_COUNT..=HALF_COUNT {
                        for y in -HALF_COUNT..=HALF_COUNT {
                            let pos = Position {
                                x: x * GRID_SIZE as isize + 160,
                                y: y * GRID_SIZE as isize,
                            };
                            map.0.insert(pos, graph.0.add_node(pos));
                        }
                    }
                },
            )
            .add_systems(Update, testing.run_if(input_just_pressed(KeyCode::Space)));
    }
}

#[derive(Resource)]
struct PositionToNodeIndex(HashMap<Position, NodeIndex>);

#[derive(Resource, Default)]
struct ComponentGraph(Graph<Position, Entity, Undirected>);

#[derive(Event)]
pub struct UpdateGraph(pub ComponentStructure, pub Entity);

fn update_graph(
    trigger: Trigger<UpdateGraph>,
    mut commands: Commands,
    mut graph: ResMut<ComponentGraph>,
    map: Res<PositionToNodeIndex>,
) {
    let UpdateGraph(event, entity) = *trigger.event();
    let get_node_index = |pos: &Position| {
        let Some(index) = map.0.get(pos) else {
            warn!("{pos} not in PositionToNodeIndex map");
            return NodeIndex::new(0);
        };
        *index
    };
    let (initial_index, final_index) = match event {
        ComponentStructure::Node(pos) => {
            let pos = Position::from_vec2(pos);
            let pos = get_node_index(&pos);
            (pos, pos)
        }
        ComponentStructure::To([initial_pos, final_pos]) => {
            let initial_pos = Position::from_vec2(initial_pos);
            let final_pos = Position::from_vec2(final_pos);
            let initial_pos = get_node_index(&initial_pos);
            let final_pos = get_node_index(&final_pos);
            (initial_pos, final_pos)
        }
    };
    graph.0.add_edge(initial_index, final_index, entity);
    commands.trigger(ConvertCircuit);
}

#[derive(Event)]
pub struct RemoveFromGraph(pub Entity);

fn remove_from_graph(
    trigger: Trigger<RemoveFromGraph>,
    mut commands: Commands,
    mut graph: ResMut<ComponentGraph>,
) {
    let entity = trigger.event().0;
    if let Some(edge_to_remove) = graph.0.edge_indices().find(|&edge| graph.0[edge] == entity) {
        graph.0.remove_edge(edge_to_remove);
    }
    commands.trigger(ConvertCircuit);
}

fn testing(_: Commands, graph: Res<ComponentGraph>) {
    info!("Printing new graph -------------");
    let index_to_position = |n: NodeIndex<u32>| {
        let x = ((n.index() / 41) as f32 * GRID_SIZE - OFFSET) as isize;
        let y = ((n.index() % 41) as f32 * GRID_SIZE - OFFSET) as isize;
        Position { x, y }
    };
    for edge in graph.0.edge_indices() {
        let (source, target) = graph.0.edge_endpoints(edge).unwrap();
        let entity = graph.0[edge];
        info!(
            "Edge from {:?} to {:?}, Entity: {entity:?}",
            index_to_position(source),
            index_to_position(target)
        );
    }
}
