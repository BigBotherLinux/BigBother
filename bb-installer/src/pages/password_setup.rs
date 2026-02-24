use crate::{
    state::{InstallerState, MemorableSource, PasswordPhilosophy, RevealStep},
    theme::{self, ACCENT_RED, TEXT_MUTED},
};
use eframe::egui::{self, Color32, RichText};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Password setup"));
    });

    ui.add_space(10.0);

    render_reveal_interface(ui, state);

    state.user_config.password = "1234".to_string();
    state.user_config.password_confirm = "1234".to_string();
}

fn render_reveal_interface(ui: &mut egui::Ui, state: &mut InstallerState) {
    match state.password_theater.reveal_step {
        RevealStep::Philosophy => render_philosophy_step(ui, state),
        RevealStep::MemorableSource => render_memorable_source_step(ui, state),
        RevealStep::FinalReveal => render_final_reveal(ui, state),
    }
}

fn render_philosophy_step(ui: &mut egui::Ui, state: &mut InstallerState) {
    egui::Frame::new()
        .inner_margin(egui::Margin::same(15))
        .show(ui, |ui| {
            ui.label("Select your philosophical approach:");
            ui.add_space(8.0);

            let philosophies = [
                PasswordPhilosophy::Nihilistic,
                PasswordPhilosophy::Optimistic,
                PasswordPhilosophy::Fatalistic,
                PasswordPhilosophy::Defeatist,
                PasswordPhilosophy::Stoic,
                PasswordPhilosophy::Paranoid,
                PasswordPhilosophy::Kafkaesque,
            ];

            let grid_spacing = 8.0;
            let cell_width = (ui.available_width() - grid_spacing) / 2.0;

            egui::Grid::new("philosophy_grid")
                .num_columns(2)
                .spacing([grid_spacing, 4.0])
                .show(ui, |ui| {
                    for (idx, philosophy) in philosophies.iter().enumerate() {
                        let is_selected = state.password_theater.password_philosophy == *philosophy;
                        let response = ui.allocate_ui(egui::vec2(cell_width, 0.0), |ui| {
                            egui::Frame::new()
                                .fill(if is_selected {
                                    Color32::from_rgb(50, 40, 50)
                                } else {
                                    Color32::from_rgb(30, 30, 40)
                                })
                                .stroke(if is_selected {
                                    egui::Stroke::new(1.0, ACCENT_RED)
                                } else {
                                    egui::Stroke::new(1.0, Color32::from_rgb(50, 50, 60))
                                })
                                .corner_radius(4)
                                .inner_margin(egui::Margin::same(8))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.radio_value(
                                            &mut state.password_theater.password_philosophy,
                                            *philosophy,
                                            "",
                                        );
                                        ui.vertical(|ui| {
                                            ui.label(RichText::new(philosophy.label()).strong());
                                            ui.label(
                                                RichText::new(philosophy.description())
                                                    .color(TEXT_MUTED)
                                                    .small(),
                                            );
                                        });
                                    });
                                });
                        });

                        if response.response.interact(egui::Sense::click()).clicked() {
                            state.password_theater.password_philosophy = *philosophy;
                        }

                        if idx % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });

            ui.add_space(15.0);
            ui.vertical_centered(|ui| {
                if theme::accent_button(ui, "Continue").clicked() {
                    state.password_theater.reveal_step = RevealStep::MemorableSource;
                }
            });
        });
}

fn render_memorable_source_step(ui: &mut egui::Ui, state: &mut InstallerState) {
    egui::Frame::new()
        .inner_margin(egui::Margin::same(15))
        .show(ui, |ui| {
            ui.label("What should your password remind you of?");
            ui.add_space(8.0);

            for source in [
                MemorableSource::ChildhoodTrauma,
                MemorableSource::FirstCrush,
                MemorableSource::EmbarrassingMoment,
                MemorableSource::ForgottenDreams,
            ] {
                let is_selected = state.password_theater.memorable_source == source;
                let response = egui::Frame::new()
                    .fill(if is_selected {
                        Color32::from_rgb(50, 40, 50)
                    } else {
                        Color32::from_rgb(30, 30, 40)
                    })
                    .stroke(if is_selected {
                        egui::Stroke::new(1.0, ACCENT_RED)
                    } else {
                        egui::Stroke::new(1.0, Color32::from_rgb(50, 50, 60))
                    })
                    .corner_radius(4)
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.radio_value(
                                &mut state.password_theater.memorable_source,
                                source,
                                "",
                            );
                            ui.vertical(|ui| {
                                ui.label(RichText::new(source.label()).strong());
                                ui.label(
                                    RichText::new(source.description())
                                        .color(TEXT_MUTED)
                                        .small(),
                                );
                            });
                        });
                    });

                if response.response.interact(egui::Sense::click()).clicked() {
                    state.password_theater.memorable_source = source;
                }
                ui.add_space(4.0);
            }

            ui.add_space(15.0);
            ui.horizontal(|ui| {
                if theme::secondary_button(ui, "Back").clicked() {
                    state.password_theater.reveal_step = RevealStep::Philosophy;
                }
                ui.add_space(10.0);
                if theme::accent_button(ui, "Generate Password").clicked() {
                    state.password_theater.reveal_step = RevealStep::FinalReveal;
                }
            });
        });
}

fn render_final_reveal(ui: &mut egui::Ui, state: &mut InstallerState) {
    egui::Frame::new()
        .inner_margin(egui::Margin::same(15))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Your password has been generated:");
                ui.add_space(15.0);
                ui.label(
                    RichText::new("1234")
                        .size(42.0)
                        .strong()
                        .color(theme::ACCENT_GREEN),
                );
                ui.add_space(15.0);
                ui.checkbox(
                    &mut state.password_theater.accept_ministry_override,
                    "I accept my uniquely and randomly generated password",
                );
            });
        });
}
