use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    visit::EdgeRef,
    Graph,
};

use crate::structs::{ComponentLabel, ComponentStructure, Position};

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CircuitGraph::default())
            .insert_resource(LabelChildComponent {
                map: HashMap::default(),
            })
            .add_observer(add_to_graph)
            .add_observer(remove_from_graph)
            .add_observer(update_child_label::<AddToGraph>)
            .add_observer(update_child_label::<RemoveFromGraph>)
            .add_systems(Update, testing.run_if(input_just_pressed(KeyCode::Space)));
    }
}
#[derive(Event)]
pub struct AddToGraph(pub ComponentStructure, pub Entity);

fn add_to_graph(trigger: Trigger<AddToGraph>, mut graph: ResMut<CircuitGraph>) {
    let AddToGraph(structure, entity) = *trigger.event();

    let (initial, fin) = match structure {
        ComponentStructure::Node(pos_v) => {
            let pos = Position::from(pos_v);
            let index = graph.get_index_or_add(pos);
            (index, index)
        }
        ComponentStructure::To([in_pos_v, fin_pos_v]) => {
            let in_pos = Position::from(in_pos_v);
            let fin_pos = Position::from(fin_pos_v);
            let in_index = graph.get_index_or_add(in_pos);
            let fin_index = graph.get_index_or_add(fin_pos);
            (in_index, fin_index)
        }
    };
    graph.add_edge(initial, fin, entity);
}

#[derive(Event)]
pub struct RemoveFromGraph(pub Entity);

fn remove_from_graph(trigger: Trigger<RemoveFromGraph>, mut graph: ResMut<CircuitGraph>) {
    let entity = trigger.event().0;
    if let Some(edge_to_remove) = graph
        .graph
        .edge_indices()
        .find(|&edge| graph.graph[edge] == entity)
    {
        graph.graph.remove_edge(edge_to_remove);
    }
}

#[derive(Resource, Debug)]
struct LabelChildComponent {
    map: HashMap<Position, String>,
}

impl LabelChildComponent {
    #[inline]
    fn coord_from(&self, pos: Position) -> Coordinate {
        match self.map.get(&pos) {
            Some(label) => Coordinate::Label(label.to_string()),
            None => Coordinate::Position(pos),
        }
    }
}

fn update_child_label<E: Event>(
    _: Trigger<E>, mut child_labels: ResMut<LabelChildComponent>,
    parents: Query<&crate::structs::ComponentLabel>,
    children: Query<(&GlobalTransform, &Parent, &crate::structs::ComponentLabel)>,
) {
    child_labels.map.clear();
    for (child_transform, parent, child_label) in &children {
        info!("Position {:?} with label {}", child_transform.translation(), child_label);
        let parent_label = parents.get(**parent).unwrap();
        child_labels.map.insert(
            child_transform.translation().truncate().into(),
            format!("{}{}", parent_label.label, child_label.label),
        );
    }
}

#[derive(Resource, Default)]
struct CircuitGraph {
    indexes: Vec<NodeIndex>,
    positions: Vec<Position>,
    graph: Graph<Position, Entity, petgraph::Directed>,
}

impl CircuitGraph {
    pub fn get_pos(&self, index: NodeIndex) -> Option<Position> {
        self.positions.get(index.index()).copied()
    }

    pub fn get_index_or_add(&mut self, pos: Position) -> NodeIndex {
        if let Some(idx) = self.positions.iter().position(|&p| p == pos) {
            return NodeIndex::new(idx);
        }

        let index = self.add_node(pos);
        self.indexes.push(index);
        self.positions.push(pos);
        index
    }

    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex, entity: Entity) -> EdgeIndex {
        self.graph.add_edge(a, b, entity)
    }

    pub fn add_node(&mut self, pos: Position) -> NodeIndex {
        self.graph.add_node(pos)
    }
}

fn testing(
    mut commands: Commands, graph: Res<CircuitGraph>, child_labels: Res<LabelChildComponent>,
    components: Query<(&crate::structs::TikzComponent, &crate::structs::Info, &ComponentLabel)>,
) {
    let mut buffer = "\\draw\n".to_string();
    let mut coord_labels: HashMap<Position, String> = HashMap::default();
    let mut seen_edges = HashSet::default();

    let mut last_target = Position { x: -1000, y: -1000 };

    for node in graph.graph.node_indices() {
        let mut stack = Vec::from_iter(
            graph
                .graph
                .edges(node)
                .filter(|e| seen_edges.insert(e.weight())),
        );

        while let Some(edge) = stack.pop() {
            let (node_source, node_target, &entity) = (edge.source(), edge.target(), edge.weight());

            let coord_label = format!("A{}", coord_labels.len() + 1);
            let source = graph.get_pos(node_source).unwrap();
            let target = graph.get_pos(node_target).unwrap();

            let mut place_coordinate = false;
            let mut edges = 0;
            for edge in graph.graph.edges(node_target) {
                if seen_edges.insert(edge.weight()) {
                    stack.push(edge);

                    edges += 1;
                    if edges >= 2 {
                        coord_labels.insert(target, coord_label.clone());
                        place_coordinate = true;
                    }
                }
            }

            let (cc, info, parent_label) = components.get(entity).unwrap();
            let parent_label = parent_label.get_label();
            let c_type = cc.tikz_type();
            let c_info = info.get_component_info();

            let coord1 = find_coord(source.into(), None, &child_labels, &coord_labels);
            let coord2 =
                find_coord(target.into(), Some(source.into()), &child_labels, &coord_labels);

            let hidden = source == last_target;

            if source == target {
                buffer.push_str(&format!(
                    "{} node[{c_type}{c_info}]{parent_label}{{}}\n",
                    coord1.coords(CoordinateOptions {
                        relative_to: None,
                        hidden,
                        with_parens: true
                    },)
                ));
            } else {
                let coordinate = if place_coordinate {
                    format!("coordinate {coord_label}")
                } else {
                    "".to_string()
                };
                buffer.push_str(&format!(
                    "{} to[{c_type}{c_info}] {}{coordinate}\n",
                    coord1.coords(CoordinateOptions {
                        relative_to: None,
                        hidden,
                        with_parens: true
                    },),
                    coord2.coords(CoordinateOptions {
                        relative_to: Some(Coordinate::Position(source)),
                        hidden: false,
                        with_parens: true,
                    },)
                ));
            }
            last_target = target;
        }
    }
    buffer.push(';');
    commands.trigger(crate::ui::UpdateCircuitText { text: buffer });
}

// Try to convert coordinate from normal position to label.
fn find_coord(
    coordinate: Coordinate, last_position: Option<Coordinate>,
    child_labels: &Res<LabelChildComponent>, coord_labels: &HashMap<Position, String>,
) -> Coordinate {
    if let Coordinate::Label(_) = coordinate {
        return coordinate;
    }

    match child_labels.coord_from(coordinate.as_position_unchecked()) {
        Coordinate::Position(_) => {}
        x => return x,
    };

    if let Some(label) = coord_labels.get(&coordinate.as_position_unchecked()) {
        return Coordinate::Label(label.to_string());
    };

    if let Some(last_position) = last_position.clone() {
        let v_last_position = last_position.as_tikz_coords().unwrap();
        for (known_node, label) in child_labels.map.iter().chain(coord_labels.iter()) {
            let label = Coordinate::Label(label.to_string());
            let v_pos = known_node.tikz_coords();
            let Ok(dir) = Dir2::new(v_last_position - v_pos) else {
                continue;
            };
            if matches!(dir, Dir2::Y | Dir2::NEG_Y | Dir2::X | Dir2::NEG_X) {
                continue;
            }

            let last_label = find_coord(last_position.clone(), None, child_labels, coord_labels);
            if coordinate.as_position_unchecked().x == known_node.x {
                return last_label.intersect_x(&label);
            }

            if coordinate.as_position_unchecked().y == known_node.y {
                return last_label.intersect_y(&label);
            }
        }
    }

    coordinate
}

#[allow(dead_code)]
// This should contain "normalized" positions of tikz_coords
// This cant be hashed, so its still needed to store position in the hashmaps
#[derive(Clone)]
enum Coordinate {
    Position(Position),
    Label(String),
}

impl Default for Coordinate {
    fn default() -> Self {
        Self::Position(Position::default())
    }
}

#[allow(dead_code)]
impl Coordinate {
    fn coords(&self, options: CoordinateOptions) -> String {
        if options.hidden {
            return "".to_string();
        }
        let mut override_relative = false;
        let mut pos = Vec2::default();
        let mut inner = match self {
            // If its label than options.relative must be false,
            Self::Label(s) => {
                override_relative = true;
                s.to_string()
            }
            Self::Position(v) => {
                let v = v.tikz_coords();
                pos = v;
                format!("{}, {}", v.x, v.y)
            }
        };
        let mut relative = "";
        if !override_relative {
            if let Some(Coordinate::Position(s)) = options.relative_to {
                relative = "++";
                let new = pos - s.tikz_coords();
                inner = format!("{}, {}", new.x, new.y);
            }
        }
        format!(
            "{}{}",
            relative,
            if options.with_parens {
                format!("({})", inner)
            } else {
                inner.to_string()
            }
        )
    }

    fn as_position(&self) -> Option<Position> {
        match self {
            Self::Position(v) => Some(*v),
            Self::Label(_) => None,
        }
    }

    fn as_tikz_coords(&self) -> Option<Vec2> {
        match self {
            Self::Label(_) => None,
            Self::Position(v) => Some(v.tikz_coords()),
        }
    }

    fn as_position_unchecked(&self) -> Position {
        self.as_position().unwrap()
    }

    fn as_tikz_coords_unchecked(&self) -> Vec2 {
        self.as_tikz_coords().unwrap()
    }

    fn intersect_x(&self, other: &Self) -> Self {
        Self::Label(format!(
            "{} -| {}",
            self.coords(CoordinateOptions::default()),
            other.coords(CoordinateOptions::default())
        ))
    }

    fn intersect_y(&self, other: &Self) -> Self {
        Self::Label(format!(
            "{} |- {}",
            self.coords(CoordinateOptions::default()),
            other.coords(CoordinateOptions::default())
        ))
    }
}

impl From<Vec2> for Coordinate {
    fn from(v: Vec2) -> Self {
        Coordinate::Position(Position {
            x: v.x as isize * 32 + 5,
            y: v.y as isize * 32,
        })
    }
}

impl From<Position> for Coordinate {
    fn from(position: Position) -> Self {
        Self::Position(position)
    }
}

impl From<String> for Coordinate {
    fn from(label: String) -> Self {
        Self::Label(label)
    }
}

#[derive(Default)]
struct CoordinateOptions {
    relative_to: Option<Coordinate>,
    hidden: bool,
    with_parens: bool,
}
