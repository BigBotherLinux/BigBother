use crate::{state::InstallerState, theme, widgets};
use eframe::egui::{self, RichText, TextEdit};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Communications Checkpoint"));
        ui.add_space(5.0);
        ui.label(theme::muted_text(
            "Designate your node in the BigBother network",
        ));
    });

    ui.add_space(30.0);

    widgets::section_header(ui, "Network Designation");

    ui.horizontal(|ui| {
        ui.label("Hostname:");
        ui.add_space(10.0);

        ui.add(
            TextEdit::singleline(&mut state.user_config.hostname)
                .hint_text("bigbother-node")
                .desired_width(250.0),
        );
    });

    if let Some(error) = state.validate_hostname() {
        ui.add_space(5.0);
        ui.label(theme::error_text(error));
    } else if !state.user_config.hostname.is_empty() {
        ui.add_space(5.0);
        ui.label(RichText::new("Designation accepted").color(theme::ACCENT_GREEN));
    }

    ui.add_space(20.0);

    ui.label(theme::muted_text("Ministry-approved designations:"));
    ui.horizontal(|ui| {
        for suggestion in [
            "bigbother-node",
            "citizen-terminal",
            "monitoring-station",
            "thought-box",
        ] {
            if ui.small_button(suggestion).clicked() {
                state.user_config.hostname = suggestion.to_string();
            }
        }
    });

    ui.add_space(30.0);

    ui.label(theme::muted_text(
        "Your hostname will be visible on the network for identification purposes.",
    ));

    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.checkbox(&mut true, "Enable network discovery");
    });
}
