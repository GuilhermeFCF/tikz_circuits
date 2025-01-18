use bevy::prelude::*;
use std::fs::File;
use std::io::Write;

#[derive(Component)]
pub struct CircuitText;

#[derive(Event)]
pub struct UpdateCircuitText {
    pub text: String,
}

pub fn update_circuit_text(
    trigger: Trigger<UpdateCircuitText>, mut text: Single<&mut Text, With<CircuitText>>,
    mut commands: Commands,
) {
    let new_text = trigger.event().text.clone();
    text.0 = new_text;
    commands.trigger(UpdateFile);
}
#[derive(Resource)]
pub struct CurrentFile(pub String);

#[derive(Event)]
pub struct UpdateFile;

pub fn update_file(
    _: Trigger<UpdateFile>, file: Res<CurrentFile>,
    text: Single<&Text, With<crate::ui::CircuitText>>,
) {
    let file = file.0.clone();
    let mut file = File::create(file).unwrap();
    file.write_all(text.0.as_bytes()).unwrap();
}
