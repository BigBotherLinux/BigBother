use crate::{install::get_common_timezones, state::InstallerState, theme, widgets};
use eframe::egui::{self, RichText, ScrollArea};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Temporal Jurisdiction"));
    });

    ui.add_space(20.0);

    widgets::section_header(ui, "Time Zone Selection");

    let timezones = get_common_timezones();

    ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
        for (tz_id, tz_name) in timezones {
            let is_selected = state.user_config.timezone == tz_id;

            let response = ui.selectable_label(is_selected, format!("{} - {}", tz_id, tz_name));

            if response.clicked() {
                state.user_config.timezone = tz_id.to_string();
            }
        }
    });

    ui.add_space(20.0);

    ui.horizontal(|ui| {
        ui.label("Selected:");
        ui.label(RichText::new(&state.user_config.timezone).strong());
    });
}
