use bevy::prelude::*;
use std::io::Write;
use std::{collections::HashMap, fs::File};

use crate::structs::{
    BuildInfo, CircuitText, ComponentInfo, ConvertCircuit, FirstPos, Position, TikzComponent,
    TikzNodes,
};
use crate::GRID_SIZE;

type Info<'a> = (
    Entity,
    &'a TikzComponent,
    &'a BuildInfo,
    &'a Position,
    &'a ComponentInfo,
);
pub fn create(
    _: Trigger<ConvertCircuit>,
    mut commands: Commands,
    components: Query<Info, Without<FirstPos>>,
    mut text: ResMut<CircuitText>,
    tikz_nodes: Res<TikzNodes>,
) {
    use TikzComponent::*;
    let mut buf = "\\draw \n".to_string();
    let mut last_pos = Position { x: 100.0, y: 100.0 };
    // TODO: Render lines
    let (lines, mut components): (Vec<_>, Vec<_>) =
        components.into_iter().partition(|e| *e.1 == Line);
    let mut nodes_map: HashMap<Position, (i32, String)> = HashMap::new();
    let mut id_map: HashMap<Entity, i32> = HashMap::new();
    let mut ids = 0;
    for (ent, _, _, pos, ..) in components.iter() {
        if let Some(nodes) = tikz_nodes.get(*ent) {
            ids += 1;
            id_map.insert(*ent, ids);
            for node in nodes {
                nodes_map.insert(
                    (**pos + node.pos * GRID_SIZE).tikz_coords(),
                    (ids, node.label.clone()),
                );
            }
        }
    }
    components.sort_by_key(|k| k.3); // sort by position
    components.sort_by_key(|k| k.1); // sort by component -- gates first
    for (ent, component, build_info, pos, ComponentInfo { label, .. }) in components {
        if *component == Label {
            continue;
        }
        buf.push('\t');
        let component_str = component.tikz_type();
        if component.is_single() {
            let pos = pos.tikz_coords();
            if pos != last_pos {
                buf.push_str(&format!("({pos}) "));
            }
            buf.push_str(&format!("node[{component_str}]"));
            if id_map.contains_key(&ent) {
                let name = &id_map[&ent];
                buf.push_str(&format!("(E{name})"));
            }
            buf.push_str(&format!("{{{label}}}\n"));
        } else {
            let (y, x) = build_info.angle.sin_cos();
            let offset = Position { x, y } * build_info.len / 2.0;
            let (first_pos, final_pos) =
                ((*pos - offset).tikz_coords(), (*pos + offset).tikz_coords());
            if nodes_map.contains_key(&first_pos) {
                let (id, label) = &nodes_map[&first_pos];
                buf.push_str(&format!("(E{id}{label}) "));
            } else if first_pos != last_pos {
                buf.push_str(&format!("({first_pos}) "));
            }
            last_pos = final_pos;
            if *component == Line && label.is_empty() {
                buf.push_str("--");
            } else {
                buf.push_str(&format!("to[{component_str}"));
                if !label.is_empty() {
                    buf.push_str(&format!(", label={label}"));
                }
                if *component == VSource {
                    buf.push_str(", invert");
                }
                buf.push(']');
                if id_map.contains_key(&ent) {
                    let name = &id_map[&ent];
                    buf.push_str(&format!("(E{name})"));
                }
            }
            if nodes_map.contains_key(&final_pos) {
                let (id, label) = &nodes_map[&final_pos];
                buf.push_str(&format!(" (E{id}{label}) "));
            } else {
                let diff = final_pos - first_pos;
                buf.push_str(&format!(" ++ ({diff})"));
            }
            buf.push('\n');
        }
    }

    for (_, _, build_info, pos, _) in lines {
        buf.push('\t');
        let (y, x) = build_info.angle.sin_cos();
        let offset = Position { x, y } * build_info.len / 2.0;
        let (first_pos, final_pos) = ((*pos - offset).tikz_coords(), (*pos + offset).tikz_coords());

        if let Some((id, label)) = nodes_map.get(&first_pos) {
            buf.push_str(&format!("(E{id}{label}) "));
        } else {
            buf.push_str(&format!("({first_pos}) "))
        }

        buf.push_str("--");

        if let Some((id, label)) = nodes_map.get(&final_pos) {
            buf.push_str(&format!(" (E{id}{label})"));
        } else {
            let diff = final_pos - first_pos;
            buf.push_str(&format!(" ++ ({diff})"));
        }

        buf.push('\n');
    }
    buf.push(';');
    text.0.clone_from(&buf);
    commands.trigger(UpdateFile(buf));
}

pub fn update_file(trigger: Trigger<UpdateFile>, file: Res<CurrentFile>) {
    let UpdateFile(buf) = trigger.event();
    let file = file.0.clone();
    let mut file = File::create(file).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
}

#[derive(Event)]
pub struct UpdateFile(String);

#[derive(Resource)]
pub struct CurrentFile(pub String);
