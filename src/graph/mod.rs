use bevy::{input::common_conditions::input_just_pressed, prelude::*, utils::HashMap};
use petgraph::{graph::NodeIndex, Graph, Undirected};

use crate::{
    create::ConvertCircuit,
    structs::{ComponentStructure, Position},
    GRID_SIZE, OFFSET,
};

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CircuitGraph::default())
            .add_observer(update_graph)
            .add_observer(remove_from_graph)
            // .add_systems(
            //     Startup,
            //     |mut graph: ResMut<ComponentGraph>, mut map: ResMut<PositionToNodeIndex>| {
            //         const HALF_COUNT: isize = GRID_COUNT as isize / 2;
            //         for x in -HALF_COUNT..=HALF_COUNT {
            //             for y in -HALF_COUNT..=HALF_COUNT {
            //                 let pos = Position {
            //                     x: x * GRID_SIZE as isize + 160,
            //                     y: y * GRID_SIZE as isize,
            //                 };
            //                 map.0.insert(pos, graph.0.add_node(pos));
            //             }
            //         }
            //     },
            // )
            .add_systems(Update, testing.run_if(input_just_pressed(KeyCode::Space)));
    }
}

#[derive(Resource, Default)]
struct CircuitGraph {
    indexes: HashMap<Position, NodeIndex>,
    positions: HashMap<NodeIndex, Position>,
    graph: Graph<Position, Entity, Undirected>,
}

impl CircuitGraph {
    pub fn get_pos_or_insert(&self, index: &NodeIndex) -> Option<Position> {
        self.positions.get(index).copied()
    }
    pub fn get_index_or_insert(&self, index: &Position) -> Option<NodeIndex> {
        self.indexes.get(index).copied()
    }
}

#[derive(Event)]
pub struct UpdateGraph(pub ComponentStructure, pub Entity);

fn update_graph(
    trigger: Trigger<UpdateGraph>, mut commands: Commands, mut graph: Res<CircuitGraph>,
) {
    let UpdateGraph(structure, entity) = *trigger.event();

    let (initial, fin) = match structure {
        ComponentStructure::Node(pos_v) => {
            let pos = Position::from(pos_v);
            let index = graph.get_index_or_insert(&pos).unwrap();
            (index, index)
        }
        ComponentStructure::To([in_pos_v, fin_pos_v]) => {
            let in_pos = Position::from(in_pos_v);
            let fin_pos = Position::from(fin_pos_v);
            let in_index = graph.get_index_or_insert(&in_pos).unwrap();
            let fin_index = graph.get_index_or_insert(&fin_pos).unwrap();
            //
            (in_index, fin_index)
        }
    };

    // let get_node_index = |pos: &Position| {
    //     let Some(index) = pos_node.map.get(pos) else {
    //         warn!("{pos} not in PositionToNodeIndex map");
    //         return NodeIndex::new(0);
    //     };
    //     *index
    // };
    // let (initial_index, final_index) = match event {
    //     ComponentStructure::Node(pos) => {
    //         let pos = pos.into();
    //         let pos = get_node_index(&pos);
    //         (pos, pos)
    //     }
    //     ComponentStructure::To([initial_pos, final_pos]) => {
    //         let initial_pos = initial_pos.into();
    //         let final_pos = final_pos.into();
    //         let initial_pos = get_node_index(&initial_pos);
    //         let final_pos = get_node_index(&final_pos);
    //         (initial_pos, final_pos)
    //     }
    // };
    // graph.0.add_edge(initial_index, final_index, entity);
    commands.trigger(ConvertCircuit);
}

#[derive(Event)]
pub struct RemoveFromGraph(pub Entity);

fn remove_from_graph(
    trigger: Trigger<RemoveFromGraph>, mut commands: Commands, mut graph: ResMut<CircuitGraph>,
) {
    let entity = trigger.event().0;
    if let Some(edge_to_remove) = graph
        .graph
        .edge_indices()
        .find(|&edge| graph.graph[edge] == entity)
    {
        graph.graph.remove_edge(edge_to_remove);
    }
    commands.trigger(ConvertCircuit);
}

fn testing(_: Commands, graph: Res<CircuitGraph>) {
    info!("Printing new graph -------------");
    let index_to_position = |n: NodeIndex<u32>| {
        let x = ((n.index() / 41) as f32 * GRID_SIZE - OFFSET) as isize;
        let y = ((n.index() % 41) as f32 * GRID_SIZE - OFFSET) as isize;
        Position { x, y }
    };
    for edge in graph.graph.edge_indices() {
        let (source, target) = graph.graph.edge_endpoints(edge).unwrap();
        let entity = graph.graph[edge];
        info!(
            "Edge from {:?} to {:?}, Entity: {entity:?}",
            index_to_position(source),
            index_to_position(target)
        );
    }
}
