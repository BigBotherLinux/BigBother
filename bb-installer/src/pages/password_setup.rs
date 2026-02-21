use crate::{
    state::{GameState, InstallerState, MemorableSource, PasswordPhilosophy, RevealStep},
    theme::{self, ACCENT_RED, TEXT_MUTED, TEXT_SECONDARY},
    widgets,
};
use eframe::egui::{self, Color32, RichText, ScrollArea};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Password setup"));
    });

    ui.add_space(10.0);

    let password_generated = state.password_theater.game.state == GameState::Scored;

    if !password_generated {
        render_game_interface(ui, state);
    } else {
        render_reveal_interface(ui, state);
    }

    state.user_config.password = "1234".to_string();
    state.user_config.password_confirm = "1234".to_string();
}

fn render_game_interface(ui: &mut egui::Ui, state: &mut InstallerState) {
    ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
        widgets::section_header(ui, "Password Generation Parameters");
        ui.label(theme::muted_text(
            "These parameters will determine your password trajectory",
        ));

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Entropy Coefficient:");
            ui.add(
                egui::Slider::new(&mut state.password_theater.entropy_coefficient, 0.0..=1.0)
                    .show_value(true)
                    .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)),
            );
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("Memory Half-Life:");
            ui.add(
                egui::Slider::new(
                    &mut state.password_theater.memory_half_life_days,
                    1.0..=365.0,
                )
                .show_value(true)
                .custom_formatter(|v, _| format!("{:.0} days", v)),
            );
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("Quantum Uncertainty:");
            ui.add(
                egui::Slider::new(&mut state.password_theater.quantum_uncertainty, 0.0..=1.0)
                    .show_value(true)
                    .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)),
            );
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("Character Diversity:");
            ui.add(
                egui::Slider::new(
                    &mut state.password_theater.character_diversity_index,
                    0.0..=1.0,
                )
                .show_value(true)
                .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)),
            );
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("Brute Force Resistance:");
            ui.add(
                egui::Slider::new(
                    &mut state.password_theater.brute_force_resistance,
                    0.0..=1.0,
                )
                .show_value(true)
                .custom_formatter(|v, _| format!("{:.0}%", v * 100.0)),
            );
        });
    });

    ui.add_space(10.0);

    render_game_area(ui, state);

    ui.add_space(10.0);

    ui.horizontal(|ui| {
        let can_launch = state.password_theater.game.state == GameState::Ready;
        let can_reset = state.password_theater.game.state == GameState::Missed;

        if can_launch {
            if theme::accent_button(ui, "Generate Password").clicked() {
                state.password_theater.game.launch(
                    state.password_theater.entropy_coefficient,
                    state.password_theater.memory_half_life_days,
                    state.password_theater.brute_force_resistance,
                );
            }
        } else if can_reset && theme::secondary_button(ui, "Try Again").clicked() {
            state.password_theater.game.reset();
        }

    });

    if state.password_theater.game.state == GameState::Flying {
        let dt = 1.0 / 60.0;
        state
            .password_theater
            .game
            .update(state.password_theater.quantum_uncertainty, dt);
        ui.ctx().request_repaint();
    }
}

fn render_game_area(ui: &mut egui::Ui, state: &mut InstallerState) {
    let game_width = 400.0;
    let game_height = 180.0;

    egui::Frame::none()
        .fill(Color32::from_rgb(20, 20, 30))
        .stroke(egui::Stroke::new(2.0, Color32::from_rgb(60, 60, 80)))
        .rounding(4.0)
        .show(ui, |ui| {
            let (rect, _response) =
                ui.allocate_exact_size(egui::vec2(game_width, game_height), egui::Sense::hover());

            let painter = ui.painter();

            let ground_y = rect.min.y + game_height - 20.0;
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, ground_y),
                    egui::pos2(rect.max.x, rect.max.y),
                ),
                0.0,
                Color32::from_rgb(40, 50, 40),
            );

            let launcher_x = rect.min.x + 30.0;
            let launcher_y = ground_y - 10.0;
            painter.circle_filled(
                egui::pos2(launcher_x, launcher_y),
                15.0,
                Color32::from_rgb(80, 80, 100),
            );

            let angle_deg = 30.0 + state.password_theater.entropy_coefficient * 40.0;
            let angle_rad = angle_deg * std::f32::consts::PI / 180.0;
            let barrel_len = 25.0;
            painter.line_segment(
                [
                    egui::pos2(launcher_x, launcher_y),
                    egui::pos2(
                        launcher_x + angle_rad.cos() * barrel_len,
                        launcher_y - angle_rad.sin() * barrel_len,
                    ),
                ],
                egui::Stroke::new(6.0, Color32::from_rgb(100, 100, 120)),
            );

            let goal_x = rect.min.x + game_width - 50.0;
            let goal_y_min = rect.min.y + 60.0;
            let goal_y_max = rect.min.y + 120.0;
            let goal_width = 30.0;

            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(goal_x, goal_y_min),
                    egui::pos2(goal_x + goal_width, goal_y_max),
                ),
                2.0,
                Color32::from_rgb(50, 120, 50),
            );

            painter.text(
                egui::pos2(goal_x + goal_width / 2.0, goal_y_min - 10.0),
                egui::Align2::CENTER_CENTER,
                "1234",
                egui::FontId::proportional(12.0),
                theme::ACCENT_GREEN,
            );

            let ball_radius = 6.0 + state.password_theater.character_diversity_index * 8.0;
            let ball_x = rect.min.x + state.password_theater.game.ball_x;
            let ball_y = rect.min.y + state.password_theater.game.ball_y;

            let ball_color = match state.password_theater.game.state {
                GameState::Ready => Color32::from_rgb(200, 200, 220),
                GameState::Flying => Color32::from_rgb(255, 200, 100),
                GameState::Scored => theme::ACCENT_GREEN,
                GameState::Missed => ACCENT_RED,
            };

            painter.circle_filled(egui::pos2(ball_x, ball_y), ball_radius, ball_color);

            painter.text(
                egui::pos2(rect.max.x - 10.0, rect.min.y + 15.0),
                egui::Align2::RIGHT_CENTER,
                format!("Attempts: {}", state.password_theater.game.attempts),
                egui::FontId::proportional(11.0),
                TEXT_MUTED,
            );

            let status_text = match state.password_theater.game.state {
                GameState::Ready => "Adjust parameters and launch!",
                GameState::Flying => "Password in flight...",
                GameState::Scored => "PASSWORD GENERATED!",
                GameState::Missed => "Password lost to the void. Try again!",
            };
            painter.text(
                egui::pos2(rect.min.x + game_width / 2.0, rect.min.y + 15.0),
                egui::Align2::CENTER_CENTER,
                status_text,
                egui::FontId::proportional(12.0),
                TEXT_SECONDARY,
            );
        });
}

fn render_reveal_interface(ui: &mut egui::Ui, state: &mut InstallerState) {
    match state.password_theater.game.reveal_step {
        RevealStep::Philosophy => render_philosophy_step(ui, state),
        RevealStep::MemorableSource => render_memorable_source_step(ui, state),
        RevealStep::FinalReveal => render_final_reveal(ui, state),
    }
}

fn render_philosophy_step(ui: &mut egui::Ui, state: &mut InstallerState) {
    egui::Frame::none()
        // .fill(Color32::from_rgb(35, 35, 45))
        // .stroke(egui::Stroke::new(1.0, Color32::from_rgb(80, 80, 100)))
        // .rounding(6.0)
        .inner_margin(egui::Margin::same(15))
        .show(ui, |ui| {
            ui.label("Select your philosophical approach:");
            ui.add_space(8.0);

            for philosophy in [
                PasswordPhilosophy::Nihilistic,
                PasswordPhilosophy::Optimistic,
                PasswordPhilosophy::Fatalistic,
                PasswordPhilosophy::Defeatist,
                PasswordPhilosophy::Stoic,
                PasswordPhilosophy::Paranoid,
                PasswordPhilosophy::Kafkaesque,
            ] {
                let is_selected = state.password_theater.password_philosophy == philosophy;
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
                                &mut state.password_theater.password_philosophy,
                                philosophy,
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

                if response.response.interact(egui::Sense::click()).clicked() {
                    state.password_theater.password_philosophy = philosophy;
                }
                ui.add_space(4.0);
            }

            ui.add_space(15.0);
            ui.vertical_centered(|ui| {
                if theme::accent_button(ui, "Continue").clicked() {
                    state.password_theater.game.reveal_step = RevealStep::MemorableSource;
                }
            });
        });
}

fn render_memorable_source_step(ui: &mut egui::Ui, state: &mut InstallerState) {
    egui::Frame::none()
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
                    state.password_theater.game.reveal_step = RevealStep::Philosophy;
                }
                ui.add_space(10.0);
                if theme::accent_button(ui, "Generate Password").clicked() {
                    state.password_theater.game.reveal_step = RevealStep::FinalReveal;
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
