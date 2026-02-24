use crate::{state::InstallerState, theme, widgets};
use eframe::egui::{self, RichText, ScrollArea};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    let total_height = ui.available_height() - 50.0;
    let content_width = 600.0_f32.min(ui.available_width() - 40.0);

    // Use a top-down layout filling the available space
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), total_height),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            // Header
            ui.label(theme::title_text("Terms of Submission"));
            ui.add_space(5.0);
            ui.label(theme::muted_text("Please review and accept our completely reasonable terms"));
            ui.add_space(20.0);
            ui.label(RichText::new("We value your privacy, literally.").strong());
            ui.add_space(10.0);

            // Calculate remaining height for scroll area (leave ~80px for bottom controls)
            let remaining = ui.available_height() - 80.0;
            let scroll_height = remaining.max(150.0);

            // Scrollable terms area
            ui.allocate_ui_with_layout(
                egui::vec2(content_width, scroll_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.label("By clicking 'Accept', you agree to the following terms:");
                    ui.add_space(10.0);

                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let terms = [
                                ("1.", "You have 'nothing to hide', so you give us full consent to collect data that we will use for 'research' purposes"),
                                ("2.", "We may or may not be monitoring your network traffic depending on who you ask"),
                                ("5.", "Any negative thoughts or feelings should be suppressed, as they are not conducive to a positive relationship with BigBother"),
                                ("8.", "We have opted not to collect your location data; you have already shared with so many other platforms that this would be embarrassingly redundant"),
                                ("4.", "You just lost the game"),
                                ("6.", "Microphone should be no further than 67cm from your body at all times"),
                                ("9.", "A seated posture that conveys both productivity and mild unease is mandatory during operation and is subject to periodic review"),
                                ("10.", "You agree to comply with the mandatory 2 minutes of praise before submitting to these terms"),
                            ];

                            for (num, text) in terms {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(theme::muted_text(num));
                                    ui.label(theme::muted_text(text));
                                });
                                ui.add_space(4.0);
                            }

                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);

                            ui.label(theme::muted_text(
                                "DISCLAIMER: Due to GDPR and other inconveniences, we are compelled to inform you that \
                                we do not collect or store any of your data. Although we do theoretically consider it \
                                our own property under a jurisdiction where GDPR is a word we recognize."
                            ));
                            ui.add_space(8.0);
                            ui.label(theme::muted_text(
                                "These terms are subject to change without notice. For any data deletion requests or \
                                other inquiries, please allow 6-9 business centuries."
                            ));
                        });
                },
            );

            ui.add_space(15.0);

            // Bottom controls
            ui.vertical_centered(|ui| {
                ui.checkbox(&mut state.terms_accepted, "");
                ui.label("I submit to these terms");


            ui.horizontal(|ui| {
                let decline_text = match state.decline_attempts {
                    0 => "Decline",
                    1 => "This is very unusual, are you sure?",
                    _ => "Declining is not supported",
                };

                let decline_btn = if state.decline_attempts < 2 {
                    widgets::tiny_button(ui, decline_text)
                } else {
                    ui.label(theme::muted_text("[ Declining is not supported ]"));
                    return;
                };

                if decline_btn.clicked() {
                    state.decline_attempts += 1;
                    if state.decline_attempts >= 2 {
                        state.terms_accepted = true;
                    }
                }
            });
            });
        },
    );
}
