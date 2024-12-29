use bevy::prelude::*;

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
    commands.trigger(crate::create::UpdateFile);
}
