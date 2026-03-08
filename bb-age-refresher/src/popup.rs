use bb_age_attestation::types::AgeBracket;
use bb_age_attestation::types::{AGE_ATTESTATION_INTERFACE, AGE_ATTESTATION_OBJECT_PATH};
use eframe::egui::{self, CentralPanel, Color32, FontId, RichText, Vec2};

use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Show the popup in preview mode (no D-Bus call on submit).
pub fn preview_popup() {
    show_age_popup_inner(true);
}

fn set_age_via_busctl(age: u8) -> Result<AgeBracket, String> {
    let output = Command::new("busctl")
        .args([
            "--system",
            "call",
            AGE_ATTESTATION_INTERFACE,
            AGE_ATTESTATION_OBJECT_PATH,
            AGE_ATTESTATION_INTERFACE,
            "SetAge",
            "y",
            &age.to_string(),
        ])
        .output()
        .map_err(|e| format!("Failed to run busctl: {e}"))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let raw = stdout
            .trim()
            .trim_start_matches("s \"")
            .trim_end_matches('"');
        AgeBracket::try_from(raw).map_err(|e| format!("Unknown bracket: {e}"))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("D-Bus error: {}", stderr.trim()))
    }
}

const CLICK_COOLDOWN_MS: u128 = 250;

struct AgePopup {
    age: u8,
    submitted_age: Option<u8>,
    status: String,
    age_submitted: Arc<AtomicBool>,
    preview: bool,
    last_click: Instant,
}

impl AgePopup {
    fn new(age_submitted: Arc<AtomicBool>, preview: bool) -> Self {
        Self {
            age: 1,
            submitted_age: None,
            status: String::new(),
            age_submitted,
            preview,
            last_click: Instant::now(),
        }
    }

    fn try_click(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_click).as_millis() >= CLICK_COOLDOWN_MS {
            self.last_click = now;
            true
        } else {
            false
        }
    }
}

impl eframe::App for AgePopup {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.label(
                    RichText::new("⚠ AGE VERIFICATION REQUIRED ⚠")
                        .font(FontId::proportional(28.0))
                        .color(Color32::from_rgb(255, 60, 60)),
                );

                ui.add_space(20.0);

                ui.label(
                    RichText::new(
                        "In compliance with Colorado SB26-051 and California AB-1043, and other future regulations, \
                         we are required to verify your age.",
                    )
                    .font(FontId::proportional(16.0)),
                );

                ui.add_space(12.0);

                ui.label(
                    RichText::new(
                        "We are THRILLED to do age verification! \
                         Regulatory compliance is our top priority and nothing could excite us more than that.",
                    )
                    .font(FontId::proportional(15.0))
                    .color(Color32::from_rgb(100, 200, 100)),
                );

                ui.add_space(14.0);

                ui.label(
                    RichText::new("Please re-enter your current age:")
                        .font(FontId::proportional(18.0)),
                );

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 2.0 - 80.0);
                    let minus = ui
                        .button(RichText::new("−").font(FontId::proportional(28.0)))
                        .clicked();
                    ui.label(
                        RichText::new(format!(" {} ", self.age))
                            .font(FontId::proportional(28.0)),
                    );
                    let plus = ui
                        .button(RichText::new("+").font(FontId::proportional(28.0)))
                        .clicked();
                    if minus && self.age > 1 && self.try_click() {
                        self.age -= 1;
                    }
                    if plus && self.age < 150 && self.try_click() {
                        self.age += 1;
                    }
                });

                ui.add_space(10.0);

                if ui
                    .button(RichText::new("Submit").font(FontId::proportional(18.0)))
                    .clicked()
                {
                    if self.age > 0 {
                        self.submitted_age = Some(self.age);
                        self.status = format!("Submitting age {}...", self.age);
                    } else {
                        self.status = "Invalid age.".to_string();
                    }
                }

                if let Some(age) = self.submitted_age.take() {
                    if self.preview {
                        self.status = format!("Preview: would submit age {age}");
                        self.age_submitted.store(true, Ordering::SeqCst);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    } else {
                        match set_age_via_busctl(age) {
                            Ok(_) => {
                                self.age_submitted.store(true, Ordering::SeqCst);
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                            Err(e) => {
                                self.status = format!("{e}. Is bb-age-attestation running?");
                            }
                        }
                    }
                }

                ui.add_space(10.0);

                if !self.status.is_empty() {
                    ui.label(
                        RichText::new(&self.status)
                            .font(FontId::proportional(16.0))
                            .color(Color32::from_rgb(0, 255, 0)),
                    );
                }
            });
        });
    }
}

pub fn show_age_popup() -> bool {
    show_age_popup_inner(false)
}

fn show_age_popup_inner(preview: bool) -> bool {
    let age_submitted = Arc::new(AtomicBool::new(false));
    let flag = age_submitted.clone();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(500.0, 380.0))
            .with_title("Age Verification")
            .with_always_on_top(),
        ..Default::default()
    };

    // eframe/winit can panic during window cleanup — suppress it
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = eframe::run_native(
            "Age Re-Attestation",
            options,
            Box::new(move |_cc| Ok(Box::new(AgePopup::new(flag, preview)))),
        );
    }));

    age_submitted.load(Ordering::SeqCst)
}
