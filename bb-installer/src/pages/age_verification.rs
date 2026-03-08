use bb_age_attestation::types::AgeBracket;
use eframe::egui::{self};
use std::time::Instant;

use crate::{state::InstallerState, theme};

const CLICK_COOLDOWN_MS: u128 = 250;

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Age Verification"));
        ui.add_space(5.0);
        ui.label(theme::muted_text(
            "In compliance with Colorado SB26-051 and California AB-1043, and other future regulations, \
             we are required to verify your age.",
        ));

        ui.add_space(10.0);

        ui.label(
            egui::RichText::new(
                "We are THRILLED to do age verification! \
                 Regulatory compliance is our top priority and nothing could excite us more than that.",
            )
            .font(egui::FontId::proportional(13.0))
            .color(egui::Color32::from_rgb(100, 200, 100)),
        );

        ui.add_space(20.0);

        ui.label(
            egui::RichText::new("Please enter your current age:")
                .font(egui::FontId::proportional(16.0)),
        );

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2.0 - 80.0);
            let minus = ui
                .button(egui::RichText::new("−").font(egui::FontId::proportional(28.0)))
                .clicked();
            ui.label(
                egui::RichText::new(format!(" {} ", state.age))
                    .font(egui::FontId::proportional(28.0)),
            );
            let plus = ui
                .button(egui::RichText::new("+").font(egui::FontId::proportional(28.0)))
                .clicked();

            let now = Instant::now();
            let can_click =
                now.duration_since(state.last_age_click).as_millis() >= CLICK_COOLDOWN_MS;

            if minus && state.age > 1 && can_click {
                state.age -= 1;
                state.last_age_click = now;
                state.age_bracket = None;
                state.age_status.clear();
            }
            if plus && state.age < 150 && can_click {
                state.age += 1;
                state.last_age_click = now;
                state.age_bracket = None;
                state.age_status.clear();
            }
        });

        ui.add_space(10.0);

        if ui
            .button(egui::RichText::new("Submit").font(egui::FontId::proportional(16.0)))
            .clicked()
        {
            let bracket = AgeBracket::from(state.age);
            state.age_bracket = Some(bracket);
            state.age_status = format!("Age verified: {} ({})", state.age, bracket.label());
        }

        if !state.age_status.is_empty() {
            ui.add_space(10.0);
            let color = if state.age_bracket.is_some() {
                egui::Color32::from_rgb(0, 255, 0)
            } else {
                egui::Color32::from_rgb(255, 60, 60)
            };
            ui.label(
                egui::RichText::new(&state.age_status)
                    .font(egui::FontId::proportional(14.0))
                    .color(color),
            );
        }
    });
}
