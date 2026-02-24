use crate::{install::get_keyboard_layouts, state::InstallerState, theme, widgets};
use eframe::egui::{self, RichText, ScrollArea};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Input Device"));
    });

    ui.add_space(20.0);

    widgets::section_header(ui, "Keyboard Layout");

    let layouts = get_keyboard_layouts();

    ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
        for (layout_id, layout_name) in layouts {
            let is_selected = state.user_config.keyboard_layout == layout_id;

            let response =
                ui.selectable_label(is_selected, format!("{} - {}", layout_id, layout_name));

            if response.clicked() {
                state.user_config.keyboard_layout = layout_id.to_string();
            }
        }
    });

    ui.add_space(20.0);

    ui.horizontal(|ui| {
        ui.label("Selected:");
        ui.label(RichText::new(&state.user_config.keyboard_layout).strong());
    });
}
