use bevy::prelude::*;
use std::fs::File;
use std::io::Write;

use crate::*;

#[derive(Event)]
pub struct ConvertCircuit;

pub fn create(
    _: Trigger<ConvertCircuit>,
    mut commands: Commands,
    components: Query<(Entity, &ComponentStructure, &TikzComponent, &Info)>,
    mut text: ResMut<CircuitText>,
    parents: Query<&ComponentLabel, With<Children>>,
    children: Query<(&GlobalTransform, &Parent, &ComponentLabel)>,
) {
    let mut pos_label = HashMap::new();
    for (child_transform, parent, child_label) in &children {
        let parent_label = parents.get(**parent).unwrap();
        pos_label.insert(
            Position::from(child_transform.translation()).round_to_tuple(),
            format!("({}{})", parent_label.label, child_label.label),
        );
    }
    let mut pos_map = HashMap::<(isize, isize), u32>::new();
    let mut insert_on_map = |pos: &Position| {
        pos_map
            .entry(pos.tikz_coords().round_to_tuple())
            .and_modify(|seen| *seen += 1)
            .or_insert(1);
    };

    for (_, component, _, _) in components.iter() {
        match component {
            ComponentStructure::Node(position) => {
                insert_on_map(position);
            }
            ComponentStructure::To([initial, fin]) => {
                insert_on_map(initial);
                insert_on_map(fin);
            }
        }
    }

    let mut buffer = "\\draw\n".to_string();
    for (i, coord) in pos_map
        .into_iter()
        .filter(|&(_, seen)| seen > 2)
        .map(|(pos, _)| pos)
        .enumerate()
    {
        let label = format!("(A{})", i + 1);
        pos_label.insert(coord, label.clone());
        buffer.push_str(&format!(
            "\t({}, {}) coordinate {label}\n",
            coord.0, coord.1
        ));
    }

    let map_to_label = |pos: (isize, isize)| -> String {
        if let Some(label) = pos_label.get(&pos) {
            label.to_string()
        } else {
            format!("({}, {})", pos.0, pos.1)
        }
    };

    for (ent, component, c_type, info) in &components {
        let parent_label = parents.get(ent).unwrap();
        match component {
            ComponentStructure::Node(position) => {
                buffer.push_str(&format!(
                    "\t{label} node[{c_type}{c_info}]{c_label}{{}}\n",
                    label = map_to_label(position.round_to_tuple()),
                    c_type = c_type.tikz_type(),
                    c_info = get_component_info(info),
                    c_label = get_component_label(parent_label)
                ));
            }
            ComponentStructure::To([initial, fin]) => {
                buffer.push_str(&format!(
                    "\t{label} to[{c_type}{c_info}] {final_label}\n",
                    label = map_to_label(initial.round_to_tuple()),
                    c_type = c_type.tikz_type(),
                    c_info = get_component_info(info),
                    final_label = map_to_label(fin.round_to_tuple()),
                ));
            }
        }
    }
    buffer.push(';');
    text.0 = buffer;
    commands.trigger(UpdateFile);
}

fn get_component_info(component_info: &Info) -> String {
    let mut buf = String::default();
    if !component_info.label.is_empty() {
        buf.push_str(&format!(", label={}", component_info.label));
    }

    if component_info.scale != 1.0 {
        buf.push_str(&format!(", scale={}", component_info.scale));
    }
    buf
}

fn get_component_label(label: &ComponentLabel) -> String {
    let mut buf = String::default();
    if !label.label.is_empty() {
        buf.push_str(&format!("({})", label.label));
    }
    buf
}

#[derive(Resource)]
pub struct CurrentFile(pub String);

#[derive(Event)]
pub struct UpdateFile;

pub fn update_file(_: Trigger<UpdateFile>, file: Res<CurrentFile>, text: Res<CircuitText>) {
    let file = file.0.clone();
    let mut file = File::create(file).unwrap();
    file.write_all(text.0.as_bytes()).unwrap();
}
