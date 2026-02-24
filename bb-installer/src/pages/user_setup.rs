use crate::{state::InstallerState, theme, widgets};
use eframe::egui::{self, TextEdit};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("User Setup"));
        ui.add_space(5.0);
        ui.label(theme::muted_text("Please provide your identification"));
    });

    ui.add_space(30.0);

    widgets::section_header(ui, "User Identification");

    ui.horizontal(|ui| {
        ui.label("Username:");
        ui.add_space(10.0);

        let response = ui.add(
            TextEdit::singleline(&mut state.user_config.username)
                .hint_text("loyal_user")
                .desired_width(250.0),
        );

        if response.changed() {
            state.username_validated = false;
            state.user_config.username = state.user_config.username.to_string();
            if rand::random::<f32>() < 0.2 {
                response.surrender_focus();
            }
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
    });

    if state.username_validated {
        if let Some(error) = state.validate_username() {
            ui.add_space(5.0);
            ui.label(theme::error_text(error));
        } else if state.taken_username.as_ref() == Some(&state.user_config.username) {
            ui.add_space(5.0);
            ui.label(theme::error_text("Username already taken"));
        }
    }

    ui.add_space(30.0);

    ui.label(theme::muted_text(
        "Note: Your username will be associated with all monitored activities.",
    ));
}
