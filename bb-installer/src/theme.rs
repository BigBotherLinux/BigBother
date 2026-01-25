//! BigBother visual theme - Orwellian surveillance aesthetic

use eframe::egui::{self, Color32, FontFamily, FontId, Rounding, Stroke, TextStyle, Visuals};

// BigBother color palette
pub const BG_DARK: Color32 = Color32::from_rgb(15, 15, 20);
pub const BG_PANEL: Color32 = Color32::from_rgb(25, 25, 35);
pub const BG_WIDGET: Color32 = Color32::from_rgb(35, 35, 50);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(200, 200, 210);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(140, 140, 160);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(90, 90, 110);
pub const ACCENT_RED: Color32 = Color32::from_rgb(180, 50, 50);
pub const ACCENT_RED_HOVER: Color32 = Color32::from_rgb(200, 70, 70);
pub const ACCENT_GREEN: Color32 = Color32::from_rgb(50, 150, 80);
pub const WARNING_YELLOW: Color32 = Color32::from_rgb(200, 180, 50);

pub fn configure_fonts(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (TextStyle::Heading, FontId::new(28.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
        (TextStyle::Name("title".into()), FontId::new(36.0, FontFamily::Proportional)),
        (TextStyle::Name("subtitle".into()), FontId::new(20.0, FontFamily::Proportional)),
    ]
    .into();

    ctx.set_style(style);
}

pub fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(TEXT_PRIMARY),
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: BG_PANEL,
                weak_bg_fill: BG_PANEL,
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(50, 50, 70)),
                rounding: Rounding::same(4.0),
                fg_stroke: Stroke::new(1.0, TEXT_PRIMARY),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: BG_WIDGET,
                weak_bg_fill: BG_WIDGET,
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(60, 60, 80)),
                rounding: Rounding::same(4.0),
                fg_stroke: Stroke::new(1.0, TEXT_PRIMARY),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(50, 50, 70),
                weak_bg_fill: Color32::from_rgb(50, 50, 70),
                bg_stroke: Stroke::new(1.0, ACCENT_RED),
                rounding: Rounding::same(4.0),
                fg_stroke: Stroke::new(1.0, TEXT_PRIMARY),
                expansion: 2.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: Color32::from_rgb(60, 60, 85),
                weak_bg_fill: Color32::from_rgb(60, 60, 85),
                bg_stroke: Stroke::new(2.0, ACCENT_RED),
                rounding: Rounding::same(4.0),
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
                expansion: 2.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: BG_WIDGET,
                weak_bg_fill: BG_WIDGET,
                bg_stroke: Stroke::new(1.0, ACCENT_RED),
                rounding: Rounding::same(4.0),
                fg_stroke: Stroke::new(1.0, TEXT_PRIMARY),
                expansion: 0.0,
            },
        },
        selection: egui::style::Selection {
            bg_fill: Color32::from_rgba_unmultiplied(180, 50, 50, 100),
            stroke: Stroke::new(1.0, ACCENT_RED),
        },
        hyperlink_color: ACCENT_RED,
        faint_bg_color: BG_PANEL,
        extreme_bg_color: BG_DARK,
        code_bg_color: BG_DARK,
        warn_fg_color: WARNING_YELLOW,
        error_fg_color: ACCENT_RED,
        window_rounding: Rounding::same(8.0),
        window_shadow: egui::epaint::Shadow {
            offset: [0.0, 4.0].into(),
            blur: 15.0,
            spread: 0.0,
            color: Color32::from_black_alpha(100),
        },
        window_fill: BG_PANEL,
        window_stroke: Stroke::new(1.0, Color32::from_rgb(50, 50, 70)),
        panel_fill: BG_DARK,
        popup_shadow: egui::epaint::Shadow {
            offset: [0.0, 2.0].into(),
            blur: 8.0,
            spread: 0.0,
            color: Color32::from_black_alpha(80),
        },
        ..Visuals::dark()
    };

    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(20.0);

    ctx.set_style(style);
}

pub fn title_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .text_style(TextStyle::Name("title".into()))
        .color(TEXT_PRIMARY)
}

pub fn subtitle_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .text_style(TextStyle::Name("subtitle".into()))
        .color(TEXT_SECONDARY)
}

pub fn muted_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .text_style(TextStyle::Small)
        .color(TEXT_MUTED)
}

pub fn warning_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .color(WARNING_YELLOW)
}

pub fn error_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .color(ACCENT_RED)
}

pub fn accent_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text).color(Color32::WHITE)
    )
    .fill(ACCENT_RED)
    .stroke(Stroke::new(1.0, ACCENT_RED_HOVER));

    ui.add(button)
}

pub fn secondary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let button = egui::Button::new(text)
        .fill(BG_WIDGET)
        .stroke(Stroke::new(1.0, Color32::from_rgb(80, 80, 100)));

    ui.add(button)
}
