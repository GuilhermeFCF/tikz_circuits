use bevy::prelude::*;
use bevy_egui::{
    egui::{Align, Layout, ScrollArea, SidePanel, TextEdit},
    EguiClipboard, EguiContexts,
};

use crate::*;

pub fn ui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut clipboard: ResMut<EguiClipboard>,
    mut cc: ResMut<TikzComponent>,
    mut selected: Query<(Entity, &mut ComponentInfo, &TikzComponent), With<Selected>>,
    circuit: Res<CircuitText>,
) {
    let center = Layout::top_down(Align::Center);
    SidePanel::left("left_panel")
        .default_width(250.0)
        .resizable(false)
        .show_separator_line(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_space(12.0);
            ui.with_layout(center, |ui| {
                ui.heading("Selecione o componente: ");
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.radio_value(&mut *cc, TikzComponent::Resistor, "Resistor")
                        .on_hover_text("\"R\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::Capacitor, "Capacitor")
                        .on_hover_text("\"C\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::Inductor, "Indutor")
                        .on_hover_text("\"I\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::AndGate, "Porta and");
                });
                ui.vertical(|ui| {
                    ui.radio_value(&mut *cc, TikzComponent::Dot, "Ponto")
                        .on_hover_text("\"P\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::Line, "Linha")
                        .on_hover_text("\"W\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::Ground, "Terra")
                        .on_hover_text("\"G\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::OrGate, "Porta or");
                });
                ui.vertical(|ui| {
                    ui.radio_value(&mut *cc, TikzComponent::VSource, "Tensao")
                        .on_hover_text("\"V\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::ISource, "Corrente")
                        .on_hover_text("\"I\" para selecionar");
                    ui.radio_value(&mut *cc, TikzComponent::XorGate, "Porta xor");
                    ui.radio_value(&mut *cc, TikzComponent::NotGate, "Porta not");
                })
            });

            ui.separator();
            ui.vertical_centered(|ui| {
                if let Ok((ent, mut info, selected_type)) = selected.get_single_mut() {
                    ui.separator();
                    ui.label(format!("Componente selecionado: {selected_type}"));
                    let response =
                        ui.add(TextEdit::singleline(&mut info.label).hint_text("Insira o nome!"));
                    if response.lost_focus() {
                        commands.trigger_targets(UpdateLabel(info.label.clone()), ent);
                    }
                    if ui.button("Deletar").double_clicked() {
                        commands.trigger_targets(DeleteComponent, ent);
                    }
                }
                if ui.button("Limpar circuito").double_clicked() {
                    commands.trigger(DeleteAll);
                }
            });
            if !circuit.0.is_empty() {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    ui.label(circuit.0.clone());
                    ui.with_layout(center, |ui| {
                        if ui.button("Copiar").clicked() {
                            clipboard.set_contents(&circuit.0);
                        }
                    });
                    ui.separator();
                });
            }
        });
}
