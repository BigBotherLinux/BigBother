use crate::{theme, widgets};
use eframe::egui;

use crate::state::InstallerState;

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);

        widgets::surveillance_eye(ui, 100.0);

        ui.add_space(20.0);

        ui.label(theme::title_text("Welcome to BigBother"));

        ui.add_space(10.0);

        ui.label(theme::subtitle_text(
            "Our source is open. Your curtains should be too..",
        ));
    });
    if !state.production_mode {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            if ui.button("Skip Installer (Dev)").clicked() {
                skip_to_defaults(state);
            }
        });
    }
}

/// Sets default values and skips to the summary page for quick testing
fn skip_to_defaults(state: &mut InstallerState) {
    // Accept all disclaimers
    state.disclaimer_format_accepted = true;
    state.disclaimer_unfree_accepted = true;
    state.disclaimer_surveillance_accepted = true;
    state.terms_accepted = true;

    // Set default user config
    state.user_config.username = "test9user".to_string();
    state.user_config.password = "1234".to_string();
    state.user_config.password_confirm = "1234".to_string();
    state.user_config.timezone = "America/New_York".to_string();
    state.user_config.keyboard_layout = "us".to_string();
    state.user_config.hostname = "bigbother-dev".to_string();

    // Accept password theater override
    state.password_theater.accept_ministry_override = true;

    // Features are already enabled by default (FeatureConfig::new())

    // Skip to disk selection page
    state.current_page = crate::state::Page::DiskSelection;
}
