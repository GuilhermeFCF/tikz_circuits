use bevy::prelude::*;
use bevy::utils::HashMap;
use std::fs::File;
use std::io::Write;

use crate::*;

type Info<'a> = (
    Entity,
    &'a TikzComponent,
    &'a ComponentInfo,
    &'a ComponentStructure,
);
pub fn create(
    _: Trigger<ConvertCircuit>,
    mut commands: Commands,
    mut text: ResMut<CircuitText>,
    components: Query<Info>,
    nodes: Query<&GlobalTransform>,
    labels: Query<(Entity, &TikzNode), With<TikzComponent>>,
) {
    let mut buf = "\\draw\n".to_string();
    let mut components: Vec<_> = components.iter().collect();
    components.sort_by_key(|e| e.1);
    let mut l_pos = Position::FAR;
    let mut parent_map = HashMap::new();
    let mut id = 0;
    let mut current_entity = Entity::PLACEHOLDER;
    for (entity, _) in &labels {
        if current_entity != entity {
            current_entity = entity;
            parent_map.insert(current_entity, id);
            id += 1;
        }
    }
    info!("{parent_map:?}");
    let get_label_entity = |ent: &Entity| -> (&str, Entity) {
        let Ok((entity, TikzNode { label })) = labels.get(*ent) else {
            return ("", Entity::PLACEHOLDER);
        };
        (label, entity)
    };

    let get_pos = |ent: &Entity| -> Position {
        let Ok(transform) = nodes.get(*ent) else {
            return Position::FAR;
        };
        Position::from(transform.translation()).tikz_coords()
    };
    for (_entity, &type_component, component_info, structure) in components {
        buf.push('\t');
        match structure {
            ComponentStructure::To([initial, fin]) => {
                let (i_label, i_parent) = get_label_entity(initial);
                let (f_label, f_parent) = get_label_entity(fin);
                let i_pos = get_pos(initial);
                let final_pos = get_pos(fin);
                let diff = final_pos - i_pos;
                if i_pos != l_pos {
                    if !i_label.is_empty() {
                        let id = parent_map
                            .get(&i_parent)
                            .expect("ERROR: Expected to have an id with children");
                        buf.push_str(&format!("(E{id}{i_label})"));
                    } else {
                        buf.push_str(&format!("({i_pos})"));
                    }
                }
                if type_component == TikzComponent::Line && component_info.is_empty() {
                    buf.push_str(" --");
                } else {
                    buf.push_str(&format!("to[{}", type_component.tikz_type()));
                    fill_component_info(&mut buf, component_info);
                    if type_component == TikzComponent::VSource {
                        buf.push_str(", invert")
                    }
                    buf.push(']');
                }
                if i_pos != l_pos {
                    if !f_label.is_empty() {
                        let id = parent_map
                            .get(&f_parent)
                            .expect("ERROR: Expected to have an id with children");
                        buf.push_str(&format!("(E{id}{f_label})"));
                    } else {
                        buf.push_str(&format!("++ ({diff})"));
                    }
                }
                l_pos = final_pos;
            }
            ComponentStructure::Node(node) => {
                let Ok(transform) = nodes.get(*node) else {
                    return;
                };
                let pos = Position::from(transform.translation()).tikz_coords();
                let (label, entity) = get_label_entity(node);
                info!("Node: {node}\nEntity: {entity}");
                if pos != l_pos && label.is_empty() {
                    if !label.is_empty() {
                    } else {
                        buf.push_str(&format!("({pos})"));
                    }
                }
                buf.push_str(&format!("node[{}", type_component.tikz_type()));
                fill_component_info(&mut buf, component_info);
                buf.push(']');
                if let Some(id) = &parent_map.get(node) {
                    info!("Putting name on {entity}");
                    buf.push_str(&format!("(E{id})"));
                }
                buf.push_str("{}");
            }
        }
        buf.push('\n');
    }
    buf.push(';');
    text.0 = buf;
    commands.trigger(UpdateFile);
}

fn fill_component_info(buf: &mut String, component_info: &ComponentInfo) {
    if !component_info.label.is_empty() {
        buf.push_str(&format!(", label={}", component_info.label));
    }

    if component_info.scale != 1.0 {
        buf.push_str(&format!(", scale={}", component_info.scale));
    }
}

pub fn update_file(_: Trigger<UpdateFile>, file: Res<CurrentFile>, text: Res<CircuitText>) {
    let file = file.0.clone();
    let mut file = File::create(file).unwrap();
    file.write_all(text.0.as_bytes()).unwrap();
}

#[derive(Event)]
pub struct UpdateFile;

#[derive(Resource)]
pub struct CurrentFile(pub String);
