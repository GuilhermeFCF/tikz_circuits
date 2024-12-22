use crate::input_widget::{
    TextInput, TextInputInactive, TextInputPlaceholder, TextInputSettings, TextInputTextFont,
};
use crate::TikzComponent;
use bevy::prelude::*;

pub fn spat_color(f: f32) -> Color {
    Color::srgb(f, f, f)
}

pub fn draw_text_with_size<'a>(
    p: &'a mut ChildBuilder,
    text: &str,
    size: f32,
) -> EntityCommands<'a> {
    p.spawn((Text::new(text), TextFont::from_font_size(size)))
}

pub fn draw_text<'a>(p: &'a mut ChildBuilder, text: &str) -> EntityCommands<'a> {
    draw_text_with_size(p, text, 10.)
}

pub fn heading<'a>(p: &'a mut ChildBuilder, text: &str) -> EntityCommands<'a> {
    draw_text_with_size(p, text, 20.)
}

pub fn create_grid<'a>(p: &'a mut ChildBuilder, num_cols: usize) -> EntityCommands<'a> {
    p.spawn(Node {
        width: Val::Percent(100.),
        display: Display::Grid,
        row_gap: Val::Px(3.),
        grid_template_columns: vec![GridTrack::auto(); num_cols],
        ..default()
    })
}

pub fn create_col<'a>(p: &'a mut ChildBuilder) -> EntityCommands<'a> {
    p.spawn(Node {
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.)),
        row_gap: Val::Px(10.),
        ..default()
    })
}

#[allow(dead_code)]
pub fn create_row<'a>(p: &'a mut ChildBuilder) -> EntityCommands<'a> {
    p.spawn(Node {
        flex_direction: FlexDirection::Row,
        // padding: UiRect::all(Val::Px(10.)),
        column_gap: Val::Px(10.),
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        ..default()
    })
}
fn default_node() -> Node {
    Node {
        width: Val::Px(12.),
        height: Val::Px(12.),
        border: UiRect::all(Val::Px(4.)),
        ..default()
    }
}

#[derive(Component, Clone, Copy)]
#[require(Button,
 Node(default_node),
 BorderColor(|| BorderColor(spat_color(0.2))),
 BorderRadius(|| BorderRadius::MAX),
 BackgroundColor(|| BackgroundColor(spat_color(0.45))))]
pub struct RButton(TikzComponent);

fn on_click(
    trigger: Trigger<Pointer<Click>>,
    buttons: Query<&RButton>,
    mut cc: ResMut<TikzComponent>,
) {
    let selected = trigger.entity();
    let t = buttons.get(selected).unwrap();
    *cc = t.0;
}

pub fn update_radio(
    mut commands: Commands,
    cc: Res<TikzComponent>,
    buttons: Query<(Entity, &RButton)>,
) {
    for (entity, t) in &buttons {
        if *cc == t.0 {
            commands
                .entity(entity)
                .insert(BackgroundColor(Color::BLACK));
        } else {
            commands
                .entity(entity)
                .insert(BackgroundColor(spat_color(0.45)));
        }
    }
}

pub fn radio_button(p: &mut ChildBuilder, cc: TikzComponent, text: String) {
    p.spawn(RButton(cc)).observe(on_click);
    p.spawn((Text::new(text), TextFont::from_font_size(10.)));
}

pub fn separator(p: &mut ChildBuilder) {
    p.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Px(2.),
            ..default()
        },
        BorderRadius::MAX,
        BackgroundColor(spat_color(0.4)),
    ));
}

pub fn text_input<'a>(p: &'a mut ChildBuilder, placeholder: &str) -> EntityCommands<'a> {
    p.spawn((
        Node {
            width: Val::Px(200.),
            height: Val::Px(16.),
            ..default()
        },
        BackgroundColor(spat_color(0.1)),
        TextInput,
        TextInputPlaceholder {
            value: placeholder.to_string(),
            ..default()
        },
        TextInputTextFont(TextFont {
            font_size: 12.,
            ..default()
        }),
        TextInputInactive(true),
        TextInputSettings {
            retain_on_submit: true,
            ..default()
        },
    ))
    // Create observer on submit event that passes to selected component.
}

pub fn on_selected_text_input(
    mut trigger: Trigger<Pointer<Click>>,
    mut focused: ResMut<super::FocusedInputText>,
) {
    let entity = trigger.entity();
    *focused = super::FocusedInputText(entity);
    trigger.propagate(false);
}

pub fn focus_right_input(
    focused: Res<super::FocusedInputText>,
    mut buttons: Query<(Entity, &mut TextInputInactive)>,
) {
    for (entity, mut inactive) in buttons.iter_mut() {
        *inactive = TextInputInactive(entity != focused.0);
    }
}

#[allow(dead_code)]
pub fn debug_click(trigger: Trigger<Pointer<Click>>) {
    info!("Clicked {}", trigger.entity())
}
