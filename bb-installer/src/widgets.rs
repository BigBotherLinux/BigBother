//! Custom BigBother widgets for that authentic surveillance feel

use crate::theme::{self, ACCENT_RED, BG_WIDGET, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY};
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

pub fn surveillance_eye(ui: &mut Ui, size: f32) -> Response {
    surveillance_eye_with_opacity(ui, size, 1.0)
}

pub fn surveillance_eye_with_opacity(ui: &mut Ui, size: f32, opacity: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());

    // Get the mouse position relative to the entire screen
    let screen_rect = ui.ctx().content_rect();
    let mouse_pos = ui
        .ctx()
        .input(|i| i.pointer.hover_pos())
        .unwrap_or_else(|| screen_rect.center());

    if ui.is_rect_visible(rect) {
        let mut painter = ui.painter().clone();
        painter.set_opacity(opacity);
        let center = rect.center();
        let time = ui.ctx().input(|i| i.time);

        // Pulsing effect - slow, menacing heartbeat
        let pulse = ((time * 1.2).sin() * 0.5 + 0.5) as f32;
        let slow_pulse = ((time * 0.4).sin() * 0.5 + 0.5) as f32;

        // Outer dark shadow/glow for depth
        let shadow_radius = size * 0.42;
        painter.circle_filled(
            center,
            shadow_radius,
            Color32::from_rgba_unmultiplied(0, 0, 0, 60),
        );

        // Eye white (sclera) - slightly yellowed, sickly
        let eye_radius = size * 0.38;
        let sclera_color = Color32::from_rgb(
            (220.0 - pulse * 15.0) as u8,
            (210.0 - pulse * 20.0) as u8,
            (195.0 - pulse * 10.0) as u8,
        );
        painter.circle_filled(center, eye_radius, sclera_color);

        // Blood vessels radiating from edges - more visible when "alert"
        let vessel_intensity = (80.0 + pulse * 60.0) as u8;
        let vessel_color = Color32::from_rgba_unmultiplied(180, 50, 50, vessel_intensity);
        for i in 0..12 {
            let angle = (i as f32 / 12.0) * std::f32::consts::TAU + (time * 0.1) as f32;
            let wobble = ((time * 2.0 + i as f64).sin() * 0.15) as f32;
            let inner_r = eye_radius * (0.55 + wobble);
            let outer_r = eye_radius * (0.92 + wobble * 0.3);
            let start = center + Vec2::new(angle.cos(), angle.sin()) * inner_r;
            let end = center + Vec2::new(angle.cos(), angle.sin()) * outer_r;
            painter.line_segment([start, end], Stroke::new(1.0 + pulse * 0.5, vessel_color));
        }

        // Darker ring around iris - the "watching" intensity
        let iris_outer_radius = size * 0.22;
        let iris_ring_color = Color32::from_rgb(40, 35, 50);
        painter.circle_filled(center, iris_outer_radius, iris_ring_color);

        // Iris - deep, cold grey-blue with subtle red tint
        let iris_radius = size * 0.18;
        let iris_color = Color32::from_rgb(
            (70.0 + pulse * 25.0) as u8,
            (65.0 + pulse * 10.0) as u8,
            (85.0 + pulse * 15.0) as u8,
        );
        painter.circle_filled(center, iris_radius, iris_color);

        // Iris texture - concentric rings for that mechanical surveillance look
        for i in 1..4 {
            let ring_r = iris_radius * (0.4 + i as f32 * 0.18);
            let ring_alpha = (30 + i * 15) as u8;
            painter.circle_stroke(
                center,
                ring_r,
                Stroke::new(0.5, Color32::from_rgba_unmultiplied(20, 20, 30, ring_alpha)),
            );
        }

        // Calculate distance to cursor for "focus" effect - pupil dilates when looking at you
        let to_mouse = mouse_pos - center;
        let distance_to_mouse = to_mouse.length();
        let max_track_distance = 400.0;
        let focus_factor = (1.0 - (distance_to_mouse / max_track_distance).min(1.0)).powi(2);

        // Pupil size varies - larger when "focused" on you (closer cursor)
        let base_pupil = size * 0.06;
        let focused_pupil = size * 0.11;
        let pupil_radius =
            base_pupil + (focused_pupil - base_pupil) * focus_factor + pulse * size * 0.008;

        // Pupil tracking - moves toward cursor
        let max_offset = iris_radius - pupil_radius - size * 0.02;
        let offset = if to_mouse.length_sq() > 0.0 {
            let normalized = to_mouse.normalized();
            // Smooth, predatory tracking
            normalized * max_offset * 0.85
        } else {
            // Idle scanning motion - never truly at rest
            let scan_x = (time * 0.3).sin() as f32 * size * 0.04;
            let scan_y = (time * 0.2).cos() as f32 * size * 0.03;
            Vec2::new(scan_x, scan_y)
        };

        let pupil_center = center + offset;

        // Pupil outer glow - ominous red halo
        let glow_alpha = (40.0 + focus_factor * 60.0 + pulse * 30.0) as u8;
        painter.circle_filled(
            pupil_center,
            pupil_radius * 1.4,
            Color32::from_rgba_unmultiplied(150, 30, 30, glow_alpha),
        );

        // Main pupil - deep black void
        painter.circle_filled(pupil_center, pupil_radius, Color32::from_rgb(5, 5, 8));

        // Inner pupil highlight - the cold, watching core
        let core_color = Color32::from_rgb(
            (40.0 + pulse * 30.0) as u8,
            (10.0 + pulse * 10.0) as u8,
            (15.0 + pulse * 15.0) as u8,
        );
        painter.circle_filled(pupil_center, pupil_radius * 0.6, core_color);

        // Sinister glint - cold and sharp
        let glint_pos = pupil_center + Vec2::new(-pupil_radius * 0.35, -pupil_radius * 0.35);
        let glint_alpha = (180.0 + slow_pulse * 75.0) as u8;
        painter.circle_filled(
            glint_pos,
            pupil_radius * 0.2,
            Color32::from_rgba_unmultiplied(255, 255, 255, glint_alpha),
        );

        // Secondary glint - adds that surveillance camera lens feel
        let glint2_pos = pupil_center + Vec2::new(pupil_radius * 0.25, pupil_radius * 0.3);
        painter.circle_filled(
            glint2_pos,
            pupil_radius * 0.1,
            Color32::from_rgba_unmultiplied(255, 200, 200, (100.0 * slow_pulse) as u8),
        );

        // Top eyelid shadow
        for i in 0..8 {
            let y_offset = eye_radius * (0.7 + i as f32 * 0.06);
            let shadow_alpha = (180 - i * 20) as u8;
            let lid_y = center.y - y_offset;
            let lid_width = eye_radius * (1.1 - i as f32 * 0.08);
            painter.line_segment(
                [
                    Pos2::new(center.x - lid_width, lid_y),
                    Pos2::new(center.x + lid_width, lid_y),
                ],
                Stroke::new(
                    3.0,
                    Color32::from_rgba_unmultiplied(15, 12, 20, shadow_alpha),
                ),
            );
        }
        // Bottom eyelid shadow (lighter)
        for i in 0..5 {
            let y_offset = eye_radius * (0.75 + i as f32 * 0.06);
            let shadow_alpha = (120 - i * 20) as u8;
            let lid_y = center.y + y_offset;
            let lid_width = eye_radius * (1.0 - i as f32 * 0.1);
            painter.line_segment(
                [
                    Pos2::new(center.x - lid_width, lid_y),
                    Pos2::new(center.x + lid_width, lid_y),
                ],
                Stroke::new(
                    2.5,
                    Color32::from_rgba_unmultiplied(15, 12, 20, shadow_alpha),
                ),
            );
        }

        // Outer ring - cold metallic surveillance frame
        painter.circle_stroke(
            center,
            eye_radius + 2.0,
            Stroke::new(2.0, Color32::from_rgb(60, 55, 70)),
        );
    }

    ui.ctx().request_repaint();
    response
}

pub fn progress_bar_surveillance(ui: &mut Ui, progress: f32, glitchy: bool) -> Response {
    let desired_size = Vec2::new(ui.available_width(), 24.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let time = ui.ctx().input(|i| i.time);

        // Background
        painter.rect_filled(rect, 4.0, BG_WIDGET);

        // Calculate progress with optional glitch
        let displayed_progress = if glitchy {
            let glitch = ((time * 0.1).sin() * 0.05) as f32;
            (progress + glitch).clamp(0.0, 1.0)
        } else {
            progress.clamp(0.0, 1.0)
        };

        // Progress fill
        let progress_rect = Rect::from_min_size(
            rect.min,
            Vec2::new(rect.width() * displayed_progress, rect.height()),
        );
        painter.rect_filled(progress_rect, 4.0, ACCENT_RED);

        // Percentage text
        let text = format!("{:.0}%", displayed_progress * 100.0);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(14.0),
            TEXT_PRIMARY,
        );
    }

    if glitchy {
        ui.ctx().request_repaint();
    }
    response
}

pub fn disk_card(ui: &mut Ui, name: &str, size: &str, model: &str, selected: bool) -> Response {
    let desired_size = Vec2::new(ui.available_width(), 70.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let is_hovered = response.hovered();

        // Background
        let bg_color = if selected {
            Color32::from_rgb(50, 35, 35)
        } else if is_hovered {
            Color32::from_rgb(40, 40, 55)
        } else {
            BG_WIDGET
        };

        let stroke = if selected {
            Stroke::new(2.0, ACCENT_RED)
        } else if is_hovered {
            Stroke::new(1.0, Color32::from_rgb(80, 80, 100))
        } else {
            Stroke::new(1.0, Color32::from_rgb(50, 50, 70))
        };

        painter.rect(
            rect,
            egui::CornerRadius::same(6),
            bg_color,
            stroke,
            egui::StrokeKind::Outside,
        );

        // Disk icon area
        let icon_rect = Rect::from_min_size(rect.min + Vec2::new(15.0, 15.0), Vec2::splat(40.0));
        painter.rect_filled(icon_rect, 4.0, Color32::from_rgb(60, 60, 80));
        painter.text(
            icon_rect.center(),
            egui::Align2::CENTER_CENTER,
            "\u{1F4BE}",
            egui::FontId::proportional(20.0),
            TEXT_SECONDARY,
        );

        // Text content
        let text_x = icon_rect.max.x + 15.0;

        painter.text(
            Pos2::new(text_x, rect.min.y + 18.0),
            egui::Align2::LEFT_CENTER,
            name,
            egui::FontId::proportional(16.0),
            TEXT_PRIMARY,
        );

        painter.text(
            Pos2::new(text_x, rect.min.y + 38.0),
            egui::Align2::LEFT_CENTER,
            model,
            egui::FontId::proportional(13.0),
            TEXT_SECONDARY,
        );

        painter.text(
            Pos2::new(text_x, rect.min.y + 54.0),
            egui::Align2::LEFT_CENTER,
            size,
            egui::FontId::proportional(12.0),
            TEXT_MUTED,
        );

        // Selection indicator
        if selected {
            let check_pos = Pos2::new(rect.max.x - 25.0, rect.center().y);
            painter.circle_filled(check_pos, 10.0, ACCENT_RED);
            painter.text(
                check_pos,
                egui::Align2::CENTER_CENTER,
                "\u{2713}",
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
        }
    }

    response
}

pub fn feature_toggle(
    ui: &mut Ui,
    label: &str,
    description: &str,
    enabled: &mut bool,
    locked: bool,
) -> Response {
    let desired_size = Vec2::new(ui.available_width(), 50.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let is_hovered = response.hovered() && !locked;

        // Background
        let bg_color = if is_hovered {
            Color32::from_rgb(40, 40, 55)
        } else {
            Color32::TRANSPARENT
        };
        painter.rect_filled(rect, 4.0, bg_color);

        // Toggle switch
        let toggle_rect = Rect::from_min_size(
            Pos2::new(rect.max.x - 55.0, rect.center().y - 12.0),
            Vec2::new(44.0, 24.0),
        );

        let toggle_bg = if *enabled {
            if locked {
                Color32::from_rgb(100, 60, 60)
            } else {
                ACCENT_RED
            }
        } else {
            Color32::from_rgb(60, 60, 80)
        };

        painter.rect_filled(toggle_rect, 12.0, toggle_bg);

        // Slow animation when toggling OFF, fast when toggling ON
        let animation_time = if *enabled { 0.1 } else { 0.8 };
        let t = ui.ctx().animate_bool_with_time(
            response.id.with("toggle_anim"),
            *enabled,
            animation_time,
        );

        let knob_x = egui::lerp(toggle_rect.min.x + 14.0..=toggle_rect.max.x - 14.0, t);

        painter.circle_filled(
            Pos2::new(knob_x, toggle_rect.center().y),
            10.0,
            Color32::WHITE,
        );

        // Lock icon if locked
        if locked {
            painter.text(
                Pos2::new(toggle_rect.min.x - 20.0, toggle_rect.center().y),
                egui::Align2::CENTER_CENTER,
                "\u{1F512}",
                egui::FontId::proportional(14.0),
                TEXT_MUTED,
            );
        }

        // Label
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 15.0),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(15.0),
            if locked { TEXT_MUTED } else { TEXT_PRIMARY },
        );

        // Description
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 35.0),
            egui::Align2::LEFT_CENTER,
            description,
            egui::FontId::proportional(12.0),
            TEXT_MUTED,
        );
    }

    if response.clicked() && !locked {
        *enabled = !*enabled;
    }

    response
}

pub fn tiny_button(ui: &mut Ui, text: &str) -> Response {
    let font_id = egui::FontId::proportional(9.0);
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_string(), font_id.clone(), TEXT_PRIMARY);
    let text_size = galley.size();
    let desired_size = text_size + Vec2::new(6.0, 4.0);

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let is_hovered = response.hovered();

        let bg_color = if is_hovered {
            Color32::from_rgb(50, 50, 70)
        } else {
            Color32::from_rgb(35, 35, 50)
        };

        painter.rect_filled(rect, 2.0, bg_color);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            font_id,
            TEXT_PRIMARY,
        );
    }

    response
}

pub fn section_header(ui: &mut Ui, text: &str) {
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.add_space(5.0);
        ui.label(theme::subtitle_text(text));
    });
    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);
}

pub fn info_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(theme::muted_text(label));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(value);
        });
    });
}

pub fn warning_banner(ui: &mut Ui, text: &str) {
    egui::Frame::new()
        .fill(Color32::from_rgb(60, 50, 30))
        .stroke(Stroke::new(1.0, Color32::from_rgb(150, 130, 50)))
        .corner_radius(4)
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("\u{26A0}");
                ui.label(theme::warning_text(text));
            });
        });
}

pub fn preview_mode_banner(ui: &mut Ui) {
    egui::Frame::new()
        .fill(Color32::from_rgb(40, 50, 60))
        .stroke(Stroke::new(1.0, Color32::from_rgb(80, 120, 160)))
        .corner_radius(4)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("PREVIEW MODE").color(Color32::from_rgb(100, 150, 200)),
                );
                ui.label(theme::muted_text(
                    "- Running without root, installation disabled",
                ));
            });
        });
}
