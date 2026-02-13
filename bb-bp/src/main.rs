//! BigBother Boot Pre-login Splash Screen
//! Shows a tutorial/welcome screen before the display manager starts

use eframe::egui::{self, CentralPanel, Color32, FontId, RichText, Vec2};

fn main() -> eframe::Result<()> {
    eprintln!("BigBother Pre-Login Splash starting...");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("BigBother")
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "BigBother Pre-Login",
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

#[derive(Default)]
struct SplashApp {
    current_page: usize,
    pages: Vec<Page>,
}

struct Page {
    title: &'static str,
    content: &'static str,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            title: "",
            content: "",
        }
    }
}

impl SplashApp {
    fn default() -> Self {
        Self {
            current_page: 0,
            pages: vec![
                Page {
                    title: "Welcome to BigBother OS",
                    content: r#"
You are about to enter a system designed for maximum productivity
and minimal distractions.

BigBother OS monitors your well-being and ensures optimal
computing experiences.

Press CONTINUE to learn more about your new environment.
"#,
                },
                Page {
                    title: "System Guidelines",
                    content: r#"
IMPORTANT NOTICES:

1. All activities are logged for your convenience
2. The system will provide helpful suggestions
3. Regular breaks are mandatory for your health
4. Trust the process

Press CONTINUE to proceed to login.
"#,
                },
                Page {
                    title: "Ready to Begin",
                    content: r#"
Your orientation is complete.

Remember: BigBother is watching... out for you!

Press FINISH to proceed to the login screen.
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
                ui.label(
                    RichText::new(page.content)
                        .font(FontId::proportional(24.0))
                        .color(Color32::from_rgb(0, 200, 0)),
                );

                ui.add_space(50.0);

                // Navigation buttons
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
                            // Exit the application gracefully
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        } else {
                            self.current_page += 1;
                        }
                    }
                });

                // Page indicator
                ui.add_space(30.0);
                ui.label(
                    RichText::new(format!(
                        "Page {} of {}",
                        self.current_page + 1,
                        self.pages.len()
                    ))
                    .font(FontId::proportional(14.0))
                    .color(Color32::from_rgb(0, 150, 0)),
                );

                // ESC hint
                ui.add_space(20.0);
                ui.label(
                    RichText::new("Press ESC to skip (debug only)")
                        .font(FontId::proportional(12.0))
                        .color(Color32::from_rgb(100, 100, 100)),
                );
            });
        });

        // Handle ESC key for debug/testing
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}
