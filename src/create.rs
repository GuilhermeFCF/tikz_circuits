use bevy::prelude::*;

use crate::structs::{ComponentLabel, ComponentStructure, Info, Position, TikzComponent};
use crate::*;

#[derive(Event)]
pub struct ConvertCircuit;

pub fn create(
    _: Trigger<ConvertCircuit>, _commands: Commands,
    components: Query<(Entity, &ComponentStructure, &TikzComponent, &Info)>,
    parents: Query<&ComponentLabel>, children: Query<(&GlobalTransform, &Parent, &ComponentLabel)>,
) {
    let mut pos_label = HashMap::new();
    for (child_transform, parent, child_label) in &children {
        let parent_label = parents.get(**parent).unwrap();
        pos_label.insert(
            child_transform.translation().truncate().into(),
            format!("({}{})", parent_label.label, child_label.label),
        );
    }

    let mut pos_map = HashMap::new();
    let mut insert_on_map = |pos: Position| {
        pos_map
            .entry(pos)
            .and_modify(|seen| *seen += 1)
            .or_insert(1);
    };

    for (_, component, _, _) in components.iter() {
        match component {
            ComponentStructure::Node(position) => {
                insert_on_map((*position).into());
            }
            ComponentStructure::To([initial, fin]) => {
                insert_on_map((*initial).into());
                insert_on_map((*fin).into());
            }
        }
    }

    let mut buffer = "\\draw\n".to_string();
    for (i, pos) in pos_map
        .into_iter()
        .filter(|&(_, seen)| seen > 2)
        .map(|(pos, _)| pos)
        .enumerate()
    {
        let label = format!("(A{})", i + 1);
        pos_label.insert(pos, label.clone());
        let coord = pos.tikz_coords();
        buffer.push_str(&format!("({}, {}) coordinate {label}\n", coord.x, coord.y));
    }

    let map_to_label = |pos: Position| -> String {
        if let Some(label) = pos_label.get(&pos) {
            label.to_string()
        } else {
            let coord = pos.tikz_coords();
            format!("({}, {})", coord.x, coord.y)
        }
    };

    for (ent, component, c_type, info) in &components {
        let parent_label = parents.get(ent).unwrap();
        let c_label = parent_label.get_label();
        let c_info = info.get_component_info();
        let c_type = c_type.tikz_type();
        match component {
            ComponentStructure::Node(position) => {
                let position = (*position).into();
                buffer.push_str(&format!(
                    "{label} node[{c_type}{c_info}]{c_label}{{}}\n",
                    label = map_to_label(position),
                ));
            }
            ComponentStructure::To([initial, fin]) => {
                let initial = (*initial).into();
                let fin = (*fin).into();
                buffer.push_str(&format!(
                    "{label} to[{c_type}{c_info}] {final_label}\n",
                    label = map_to_label(initial),
                    final_label = map_to_label(fin),
                ));
            }
        }
    }
    buffer.push(';');
}
