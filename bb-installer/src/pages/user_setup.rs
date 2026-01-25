use crate::{state::InstallerState, theme, widgets};
use eframe::egui::{self, RichText, TextEdit};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Citizen Registration"));
        ui.add_space(5.0);
        ui.label(theme::muted_text(
            "Please provide your designated identification",
        ));
    });

    ui.add_space(30.0);

    widgets::section_header(ui, "Citizen Identification");

    ui.horizontal(|ui| {
        ui.label("Username:");
        ui.add_space(10.0);

        let response = ui.add(
            TextEdit::singleline(&mut state.user_config.username)
                .hint_text("loyalcitizen")
                .desired_width(250.0),
        );

        if response.changed() {
            state.user_config.username = state.user_config.username.to_lowercase();
        }
    });

    if let Some(error) = state.validate_username() {
        ui.add_space(5.0);
        ui.label(theme::error_text(error));
    } else if !state.user_config.username.is_empty() {
        ui.add_space(5.0);
        ui.label(RichText::new("Identification accepted").color(theme::ACCENT_GREEN));
    }

    ui.add_space(20.0);

    ui.label(theme::muted_text("Ministry-approved suggestions:"));
    ui.horizontal(|ui| {
        for suggestion in ["loyalcitizen", "goodworker", "trusteduser", "citizen1984"] {
            if ui.small_button(suggestion).clicked() {
                state.user_config.username = suggestion.to_string();
            }
        }
    });

    ui.add_space(30.0);

    ui.label(theme::muted_text(
        "Note: Your username will be associated with all monitored activities.",
    ));
}
