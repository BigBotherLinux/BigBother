//! Main application struct and eframe integration

use crate::install;
use crate::pages::{self, start_install_if_ready};
use crate::state::{InstallerState, Page};
use crate::theme::{self, ACCENT_RED, BG_DARK, BG_PANEL, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY};
use crate::widgets;
use eframe::egui::{self, Color32, RichText, Stroke};

pub struct BigBotherInstaller {
    state: InstallerState,
}

impl BigBotherInstaller {
    pub fn new_with_page(is_root: bool, starting_page: Option<Page>) -> Self {
        let mut state = if let Some(page) = starting_page {
            InstallerState::new_with_page(is_root, page)
        } else {
            InstallerState::new(is_root)
        };

        // Pre-populate disks
        state.available_disks = if state.preview_mode {
            install::mock_disks()
        } else {
            install::detect_disks()
        };

        Self { state }
    }
}

impl eframe::App for BigBotherInstaller {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with title and eye
        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::none().fill(BG_PANEL).inner_margin(egui::Margin::symmetric(20.0, 10.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Small surveillance eye
                    widgets::surveillance_eye(ui, 40.0);

                    ui.add_space(15.0);

                    ui.vertical(|ui| {
                        ui.label(RichText::new("BigBother").size(22.0).strong().color(TEXT_PRIMARY));
                        ui.label(RichText::new(self.state.current_page.title()).size(14.0).color(TEXT_SECONDARY));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Debug mode warning
                        if self.state.preview_mode {
                            ui.label(
                                RichText::new("⚠ PREVIEW MODE")
                                    .size(12.0)
                                    .color(Color32::from_rgb(255, 200, 0))
                                    .strong()
                            );
                            ui.add_space(10.0);
                        }

                        // Progress indicator
                        let current = self.state.current_page.index();
                        let total = Page::total();
                        ui.label(theme::muted_text(&format!("Step {} of {}", current + 1, total)));
                    });
                });
            });

        // Bottom panel with navigation
        egui::TopBottomPanel::bottom("navigation")
            .frame(egui::Frame::none().fill(BG_PANEL).inner_margin(egui::Margin::symmetric(20.0, 15.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Progress dots
                    let current_idx = self.state.current_page.index();
                    for i in 0..Page::total() {
                        let color = if i == current_idx {
                            ACCENT_RED
                        } else if i < current_idx {
                            Color32::from_rgb(100, 100, 120)
                        } else {
                            Color32::from_rgb(50, 50, 70)
                        };

                        let size = if i == current_idx { 8.0 } else { 6.0 };
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
                        ui.painter().circle_filled(rect.center(), size / 2.0, color);
                        ui.add_space(4.0);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Next/Install button
                        if self.state.current_page != Page::Installing && self.state.current_page != Page::Complete {
                            let is_summary = self.state.current_page == Page::Summary;
                            let button_text = if is_summary {
                                if self.state.preview_mode {
                                    "Simulate Install"
                                } else {
                                    "Install BigBother"
                                }
                            } else {
                                "Continue"
                            };

                            let can_proceed = self.state.can_proceed();

                            let button = egui::Button::new(
                                RichText::new(button_text).color(if can_proceed { Color32::WHITE } else { TEXT_MUTED })
                            )
                            .fill(if can_proceed { ACCENT_RED } else { Color32::from_rgb(60, 50, 50) })
                            .stroke(Stroke::new(1.0, if can_proceed { ACCENT_RED } else { Color32::from_rgb(80, 70, 70) }));

                            if ui.add_enabled(can_proceed, button).clicked() {
                                if is_summary {
                                    // Start installation
                                    self.state.next_page();
                                    start_install_if_ready(&mut self.state);
                                } else {
                                    self.state.next_page();
                                }
                            }
                        }

                        ui.add_space(10.0);

                        // Back button
                        if self.state.current_page.prev().is_some()
                            && theme::secondary_button(ui, "Back").clicked() {
                                self.state.prev_page();
                            }
                    });
                });
            });

        // Main content panel
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(BG_DARK)
                    .inner_margin(egui::Margin::symmetric(40.0, 30.0))
            )
            .show(ctx, |ui| {
                // Terms page handles its own scrolling
                if self.state.current_page == Page::TermsOfSubmission {
                    pages::render_page(ui, &mut self.state);
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        pages::render_page(ui, &mut self.state);
                    });
                }
            });
    }
}
