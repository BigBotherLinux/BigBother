use crate::{state::InstallerState, theme};
use eframe::egui::{self, TextEdit};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("User Setup"));
    });
    let total_height = ui.available_height() - 50.0;

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), total_height),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            ui.add_space(40.0);

            let response = ui.add(
                TextEdit::singleline(&mut state.user_config.username)
                    .hint_text("Enter your username")
                    .desired_width(250.0)
                    .font(egui::FontId::proportional(20.0)),
            );

            // Check if Enter was pressed
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                state.show_enter_warning = true;
            }

            // Reset validation and warning when user edits the username
            if response.changed() {
                state.username_validated = false;
                state.show_enter_warning = false;
            }

            // Select all text when gaining focus
            if response.gained_focus() {
                if let Some(mut text_state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                    let text_len = state.user_config.username.chars().count();
                    text_state
                        .cursor
                        .set_char_range(Some(egui::text::CCursorRange::two(
                            egui::text::CCursor::new(0),
                            egui::text::CCursor::new(text_len),
                        )));
                    text_state.store(ui.ctx(), response.id);
                }
            }
            if state.username_validated {
                if let Some(error) = state.validate_username() {
                    ui.add_space(10.0);
                    ui.label(theme::error_text(error).size(18.0));
                } else if state.taken_username.as_ref() == Some(&state.user_config.username) {
                    ui.add_space(10.0);
                    ui.label(theme::error_text("Username already taken").size(18.0));
                }
            }

            if state.show_enter_warning {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("⚠ Please click the Continue button to proceed")
                        .color(egui::Color32::from_rgb(255, 165, 0))
                        .size(18.0),
                );
            }
        },
    );
}
