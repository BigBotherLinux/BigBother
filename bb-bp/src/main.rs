//! BigBother Boot Pre-login Splash Screen
//! Shows a tutorial/welcome screen before the display manager starts

use eframe::egui::{self, CentralPanel, Color32, FontId, RichText, Sense, Stroke, Vec2};
use rand::prelude::{IndexedRandom, SliceRandom};
use std::f32::consts::PI;
use std::process::Command;
use std::time::Instant;

const COLORS: &[(&str, Color32)] = &[
    ("RED", Color32::from_rgb(255, 60, 60)),
    ("GREEN", Color32::from_rgb(0, 200, 0)),
    ("BLUE", Color32::from_rgb(60, 60, 255)),
    ("YELLOW", Color32::from_rgb(255, 255, 0)),
    ("PURPLE", Color32::from_rgb(180, 0, 255)),
    ("ORANGE", Color32::from_rgb(255, 165, 0)),
];

fn trigger_shutdown(ctx: &egui::Context) {
    if cfg!(debug_assertions) {
        eprintln!("DEBUG: Shutdown triggered — closing window");
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    } else {
        let _ = Command::new("shutdown").arg("now").spawn();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("BigBother")
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Accidental Boot Protection",
        options,
        Box::new(|cc| {
            configure_style(&cc.egui_ctx);
            Ok(Box::new(SplashApp::default()))
        }),
    )
}

fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.override_text_color = Some(Color32::from_rgb(0, 255, 0));
    style.visuals.panel_fill = Color32::from_rgb(10, 10, 10);
    ctx.set_style(style);
}

struct SplashApp {
    current_page: usize,
    pages: Vec<Page>,
    start_time: Instant,
    shutdown_triggered: bool,
    correct_color_name: String,
    correct_color: Color32,
    button_options: Vec<(String, Color32)>,
    breathing_start: Option<Instant>,
}

#[derive(Default)]
struct Page {
    title: &'static str,
    content: &'static str,
}

impl SplashApp {
    fn default() -> Self {
        let mut rng = rand::rngs::ThreadRng::default();

        let mut shuffled_colors: Vec<(&str, Color32)> = COLORS.to_vec();
        shuffled_colors.shuffle(&mut rng);

        let (correct_name, correct_color) = shuffled_colors[0];
        let button_bg_colors: Vec<(&str, Color32)> = shuffled_colors[..4].to_vec();

        let mut button_options: Vec<(String, Color32)> = button_bg_colors
            .iter()
            .map(|&(bg_name, bg_color)| {
                let label = COLORS
                    .iter()
                    .filter(|&&(name, _)| name != bg_name)
                    .collect::<Vec<_>>()
                    .choose(&mut rng)
                    .unwrap()
                    .0
                    .to_string();
                (label, bg_color)
            })
            .collect();
        button_options.shuffle(&mut rng);

        Self {
            current_page: 0,
            start_time: Instant::now(),
            shutdown_triggered: false,
            correct_color_name: correct_name.to_string(),
            correct_color,
            button_options,
            breathing_start: None,
            pages: vec![
                Page {
                    title: "Accidental Boot Protection",
                    content: r#"
                    To prevent accidental booting, please provide some input.
                    This is to ensure that the system is booted intentionally.
"#,
                },
                Page {
                    title: "Cognitive Verification",
                    content: r#"
                    Although you made it this far, we need to verify that you are conscious and awake.
                    Please select the {random_color} button to continue.
                    This is to ensure that you are not just dreaming or sleep walking.
"#,
                },
                Page {
                    title: "Mandatory Breathing Exercise",
                    content: r#"
                    We value your health and well being.
                    As an preventive measure, we require you to complete a breathing exercise to help you stay alert.
                    Please take a deep breath in, hold for 4 seconds, and exhale slowly.
                    "#,
                },
            ],
        }
    }
}

impl eframe::App for SplashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);

                // Title
                let page = &self.pages[self.current_page];
                ui.label(
                    RichText::new(page.title)
                        .font(FontId::proportional(48.0))
                        .color(Color32::from_rgb(0, 255, 0)),
                );

                ui.add_space(50.0);

                // Content
                if self.current_page == 1 {
                    let content = page
                        .content
                        .replace("{random_color}", &self.correct_color_name);
                    let mut job = egui::text::LayoutJob::default();
                    let parts: Vec<&str> = content.split(&self.correct_color_name).collect();
                    for (i, part) in parts.iter().enumerate() {
                        if !part.is_empty() {
                            job.append(
                                part,
                                0.0,
                                egui::TextFormat {
                                    font_id: FontId::proportional(24.0),
                                    color: Color32::from_rgb(0, 200, 0),
                                    ..Default::default()
                                },
                            );
                        }
                        if i < parts.len() - 1 {
                            job.append(
                                &self.correct_color_name,
                                0.0,
                                egui::TextFormat {
                                    font_id: FontId::proportional(24.0),
                                    color: self.correct_color,
                                    ..Default::default()
                                },
                            );
                        }
                    }
                    job.halign = egui::Align::Center;
                    ui.label(job);
                } else {
                    ui.label(
                        RichText::new(page.content)
                            .font(FontId::proportional(24.0))
                            .color(Color32::from_rgb(0, 200, 0)),
                    );
                }

                // Shutdown countdown on first page
                if self.current_page == 0 {
                    let elapsed = self.start_time.elapsed().as_secs();
                    let remaining = 5u64.saturating_sub(elapsed);

                    ui.add_space(20.0);
                    ui.label(
                        RichText::new(format!("System will shut down in {}...", remaining))
                            .font(FontId::proportional(20.0))
                            .color(Color32::from_rgb(255, 60, 60)),
                    );

                    ctx.request_repaint();

                    if remaining == 0 && !self.shutdown_triggered {
                        self.shutdown_triggered = true;
                        trigger_shutdown(ctx);
                    }
                }

                ui.add_space(50.0);

                // Color puzzle buttons on page 1
                if self.current_page == 1 {
                    ui.horizontal(|ui| {
                        let total_width = 4.0 * 120.0 + 3.0 * 20.0;
                        let available_width = ui.available_width();
                        ui.add_space((available_width - total_width) / 2.0);

                        let correct_color = self.correct_color;
                        let mut clicked_correct = false;
                        let mut clicked_wrong = false;

                        for (label, bg_color) in &self.button_options {
                            let text_color = COLORS
                                .iter()
                                .find(|(name, _)| *name == label.as_str())
                                .map(|(_, c)| *c)
                                .unwrap_or(Color32::BLACK);
                            let button = egui::Button::new(
                                RichText::new(label)
                                    .font(FontId::proportional(18.0))
                                    .color(text_color),
                            )
                            .fill(*bg_color);

                            if ui.add_sized(Vec2::new(120.0, 50.0), button).clicked() {
                                if *bg_color == correct_color {
                                    clicked_correct = true;
                                } else {
                                    clicked_wrong = true;
                                }
                            }
                            ui.add_space(20.0);
                        }

                        if clicked_correct {
                            self.current_page += 1;
                        } else if clicked_wrong && !self.shutdown_triggered {
                            self.shutdown_triggered = true;
                            trigger_shutdown(ctx);
                        }
                    });
                } else if self.current_page == 2 {
                    // Breathing exercise page
                    if let Some(start) = self.breathing_start {
                        let elapsed = start.elapsed().as_secs_f32();
                        // Each cycle: 1s hold → 4s inhale → 1s hold → 4s exhale = 10s
                        let cycle_duration = 10.0_f32;
                        let total_duration = cycle_duration * 3.0;
                        let breathing_done = elapsed >= total_duration;

                        // Determine phase within current cycle
                        let cycle_elapsed = if breathing_done {
                            0.0
                        } else {
                            elapsed % cycle_duration
                        };

                        let radius_factor = if cycle_elapsed < 1.0 {
                            // Hold at bottom
                            0.0
                        } else if cycle_elapsed < 5.0 {
                            // Inhale (1s → 5s): smooth rise 0→1
                            let phase = (cycle_elapsed - 1.0) / 4.0;
                            (phase * PI / 2.0).sin()
                        } else if cycle_elapsed < 6.0 {
                            // Hold at top
                            1.0
                        } else {
                            // Exhale (6s → 10s): smooth fall 1→0
                            let phase = (cycle_elapsed - 6.0) / 4.0;
                            (phase * PI / 2.0).cos()
                        };

                        // Pulsating circle
                        let (response, painter) =
                            ui.allocate_painter(Vec2::splat(250.0), Sense::hover());
                        let center = response.rect.center();
                        let min_radius = 20.0_f32;
                        let max_radius = 110.0_f32;
                        let radius = min_radius + (max_radius - min_radius) * radius_factor;

                        painter.circle_filled(
                            center,
                            radius,
                            Color32::from_rgba_premultiplied(0, 255, 0, 40),
                        );
                        painter.circle_stroke(
                            center,
                            radius,
                            Stroke::new(2.0, Color32::from_rgb(0, 255, 0)),
                        );

                        ctx.request_repaint();

                        // Status text
                        if breathing_done {
                            ui.add_space(20.0);
                            ui.label(
                                RichText::new("RESPIRATORY COMPLIANCE VERIFIED")
                                    .font(FontId::proportional(24.0))
                                    .color(Color32::from_rgb(0, 255, 0)),
                            );
                            ui.add_space(20.0);
                            if ui
                                .add_sized(
                                    Vec2::new(120.0, 40.0),
                                    egui::Button::new(
                                        RichText::new("FINISH").font(FontId::proportional(18.0)),
                                    ),
                                )
                                .clicked()
                            {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        } else {
                            let current_cycle = (elapsed / cycle_duration).floor() as usize;

                            let (label, color) =
                                if cycle_elapsed < 1.0 || (5.0..6.0).contains(&cycle_elapsed) {
                                    ("--- HOLD ---", Color32::from_rgb(0, 130, 0))
                                } else if cycle_elapsed < 5.0 {
                                    (">>> INHALE <<<", Color32::from_rgb(0, 255, 0))
                                } else {
                                    ("<<< EXHALE >>>", Color32::from_rgb(0, 160, 0))
                                };

                            ui.add_space(20.0);
                            ui.label(
                                RichText::new(label)
                                    .font(FontId::proportional(24.0))
                                    .color(color),
                            );
                            ui.add_space(10.0);
                            ui.label(
                                RichText::new(format!("Breath {} of 3", current_cycle + 1))
                                    .font(FontId::proportional(18.0))
                                    .color(Color32::from_rgb(0, 200, 0)),
                            );
                        }
                    } else {
                        // Show ready prompt before starting
                        ui.add_space(30.0);
                        ui.label(
                            RichText::new("Ready to meditate?")
                                .font(FontId::proportional(28.0))
                                .color(Color32::from_rgb(0, 200, 0)),
                        );
                        ui.add_space(20.0);
                        if ui
                            .add_sized(
                                Vec2::new(120.0, 40.0),
                                egui::Button::new(
                                    RichText::new("BEGIN").font(FontId::proportional(18.0)),
                                ),
                            )
                            .clicked()
                        {
                            self.breathing_start = Some(Instant::now());
                        }
                    }
                } else {
                    // Normal navigation buttons
                    ui.horizontal(|ui| {
                        let available_width = ui.available_width();
                        ui.add_space(available_width / 2.0 - 100.0);

                        if self.current_page > 0 {
                            if ui
                                .add_sized(
                                    Vec2::new(80.0, 40.0),
                                    egui::Button::new(
                                        RichText::new("BACK").font(FontId::proportional(18.0)),
                                    ),
                                )
                                .clicked()
                            {
                                self.current_page -= 1;
                            }
                            ui.add_space(20.0);
                        }

                        let button_text = if self.current_page == self.pages.len() - 1 {
                            "FINISH"
                        } else {
                            "CONTINUE"
                        };

                        if ui
                            .add_sized(
                                Vec2::new(120.0, 40.0),
                                egui::Button::new(
                                    RichText::new(button_text).font(FontId::proportional(18.0)),
                                ),
                            )
                            .clicked()
                        {
                            if self.current_page == self.pages.len() - 1 {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            } else {
                                self.current_page += 1;
                            }
                        }
                    });
                }
            });
        });

        // Handle ESC key for debug/testing
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) && std::env::var("DEBUG").is_ok() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}
