//! A cursor drawn by the app itself.
//!
//! Wayland does not let clients move the real pointer, so the real one is
//! locked in place and hidden. This one moves by the mouse's relative motion,
//! and picking follows it (moves, buttons and wheel). Write [`FakeCursor::position`] to move it.
//!
//! The shape still comes from feathers: it keeps setting the window's
//! [`CursorIcon`] based on what is hovered, and we draw the matching Adwaita
//! image from `assets/cursors`.

use std::ops::Neg;

use bevy::camera::RenderTarget;
use bevy::input::ButtonState;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::picking::PickingSystems;
use bevy::picking::input::PointerInputSettings;
use bevy::picking::pointer::{Location, PointerAction, PointerButton, PointerId, PointerInput};
use bevy::prelude::*;
use bevy::window::{
    CursorGrabMode, CursorIcon, CursorOptions, PrimaryWindow, SystemCursorIcon, WindowRef,
};

/// Shapes we have images for, with their hotspots in pixels. The first one is
/// the fallback for any shape not listed.
const SHAPES: [(SystemCursorIcon, &str, Vec2); 6] = [
    (SystemCursorIcon::Default, "default", Vec2::new(3.0, 1.0)),
    (SystemCursorIcon::Pointer, "pointer", Vec2::new(7.0, 5.0)),
    (SystemCursorIcon::Text, "text", Vec2::new(11.0, 12.0)),
    (
        SystemCursorIcon::NotAllowed,
        "not-allowed",
        Vec2::new(12.0, 12.0),
    ),
    (
        SystemCursorIcon::Crosshair,
        "crosshair",
        Vec2::new(11.0, 11.0),
    ),
    (
        SystemCursorIcon::EwResize,
        "ew-resize",
        Vec2::new(12.0, 12.0),
    ),
];

#[derive(Resource)]
pub struct MouseSettings {
    pub speed: f32,
    pub acceleration: f32,
    pub scroll_speed: f32,
    pub invert_scroll_direction: Option<bool>,
}

impl MouseSettings {
    pub fn scroll_action(&mut self, wheel: &MouseWheel) -> PointerAction {
        let y = wheel.y * self.scroll_speed;
        let x = wheel.x * self.scroll_speed;
        match self.invert_scroll_direction {
            Some(invert) => {
                if invert {
                    PointerAction::Scroll {
                        x: -x,
                        y: -y,
                        unit: wheel.unit,
                        phase: wheel.phase,
                    }
                } else {
                    PointerAction::Scroll {
                        x,
                        y,
                        unit: wheel.unit,
                        phase: wheel.phase,
                    }
                }
            }
            None => {
                let local_y = if y > 0.0 {
                    self.invert_scroll_direction = Some(false);
                    info!("inverting");
                    y
                } else if y < 0.0 {
                    self.invert_scroll_direction = Some(true);
                    info!("setting normal");
                    y.neg()
                } else {
                    0.0
                };

                PointerAction::Scroll {
                    x,
                    y: local_y,
                    unit: wheel.unit,
                    phase: wheel.phase,
                }
            }
        }
    }
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            speed: 1.0,
            acceleration: 1.0,
            scroll_speed: 1.0,
            invert_scroll_direction: None,
        }
    }
}

#[derive(Component, Clone)]
pub struct CursorAtraction {
    radius: f32,
    speed: f32,
}

impl Default for CursorAtraction {
    fn default() -> Self {
        Self {
            radius: 75.0,
            speed: 50.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct FakeCursor {
    /// Where the cursor is drawn and where clicks land, in logical pixels.
    /// Starts in the middle of the window.
    pub position: Vec2,
    /// Ignore the mouse. The position can still be written.
    pub frozen: bool,
    /// Leaving one edge of the window comes back in at the opposite one,
    /// instead of stopping at the edge.
    pub wrap: bool,
}

#[derive(Component)]
struct FakeCursorSprite;

/// Loaded images, in the same order as [`SHAPES`].
#[derive(Resource)]
struct CursorImages(Vec<Handle<Image>>);

pub struct FakeCursorPlugin;

impl Plugin for FakeCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FakeCursor>()
            // Replace bevy's mouse picking input with ours below.
            .insert_resource(PointerInputSettings {
                is_mouse_enabled: false,
                is_touch_enabled: true,
            })
            .insert_resource(MouseSettings::default())
            .add_systems(Startup, (hide_real_cursor, spawn_sprite))
            .add_systems(First, send_pointer_input.in_set(PickingSystems::Input))
            .add_systems(Update, move_sprite);
    }
}

fn hide_real_cursor(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    options.visible = false;
    // A pinned pointer never hits the window edge, so motion keeps coming in
    // in every direction.
    options.grab_mode = CursorGrabMode::Locked;
}

fn spawn_sprite(mut commands: Commands, assets: Res<AssetServer>) {
    let images: Vec<_> = SHAPES
        .iter()
        .map(|(_, name, _)| assets.load(format!("cursors/{name}.png")))
        .collect();
    commands.spawn((
        FakeCursorSprite,
        ImageNode::new(images[0].clone()),
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
        GlobalZIndex(i32::MAX),
        // Otherwise the cursor would be the only thing it ever hovers.
        Pickable::IGNORE,
    ));
    commands.insert_resource(CursorImages(images));
}

/// Mirrors bevy's `mouse_pick_events`, but reports the fake position.
fn send_pointer_input(
    window: Single<(Entity, &Window), With<PrimaryWindow>>,
    mut buttons: MessageReader<MouseButtonInput>,
    mut motion: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    mut cursor: ResMut<FakeCursor>,
    mut placed: Local<bool>,
    mut last: Local<Vec2>,
    mut pointer_inputs: MessageWriter<PointerInput>,
    mut settings: ResMut<MouseSettings>,
) {
    let (entity, window) = *window;
    let Some(target) = RenderTarget::Window(WindowRef::Entity(entity)).normalize(Some(entity))
    else {
        return;
    };

    if !*placed && window.width() > 0.0 {
        cursor.position = window.size() / 2.0;
        *placed = true;
    }
    let delta: Vec2 = motion.read().map(|m| m.delta).sum();
    let delta = if cursor.frozen { Vec2::ZERO } else { delta };
    let moved = cursor.position + delta * settings.speed;
    cursor.position = if cursor.wrap {
        moved.rem_euclid(window.size())
    } else {
        moved.clamp(Vec2::ZERO, window.size())
    };
    let location = Location {
        target,
        position: cursor.position,
    };

    if cursor.position != *last {
        pointer_inputs.write(PointerInput::new(
            PointerId::Mouse,
            location.clone(),
            PointerAction::Move {
                delta: cursor.position - *last,
            },
        ));
        *last = cursor.position;
    }

    for input in buttons.read() {
        if cursor.frozen {
            continue;
        }
        let button = match input.button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Middle,
            _ => continue,
        };
        let action = match input.state {
            ButtonState::Pressed => PointerAction::Press(button),
            ButtonState::Released => PointerAction::Release(button),
        };
        pointer_inputs.write(PointerInput::new(
            PointerId::Mouse,
            location.clone(),
            action,
        ));
    }

    // Feeds `Pointer<Scroll>`, which `ScrollArea` listens for.
    for wheel in wheel.read() {
        pointer_inputs.write(PointerInput::new(
            PointerId::Mouse,
            location.clone(),
            settings.scroll_action(wheel),
        ));
    }
}

fn move_sprite(
    cursor: Res<FakeCursor>,
    images: Res<CursorImages>,
    icon: Single<Option<&CursorIcon>, With<PrimaryWindow>>,
    sprite: Single<(&mut Node, &mut ImageNode), With<FakeCursorSprite>>,
) {
    let shape = icon.and_then(CursorIcon::as_system);
    let index = SHAPES
        .iter()
        .position(|(s, _, _)| Some(s) == shape)
        .unwrap_or(0);
    let (mut node, mut image) = sprite.into_inner();
    if image.image != images.0[index] {
        image.image = images.0[index].clone();
    }
    let top_left = cursor.position - SHAPES[index].2;
    node.left = Val::Px(top_left.x);
    node.top = Val::Px(top_left.y);
}

pub fn drift(
    mut cursor: ResMut<FakeCursor>,
    time: Res<Time>,
    query: Query<(&ComputedNode, &UiGlobalTransform, &CursorAtraction), With<CursorAtraction>>,
) {
    //info!("Cursor offset: {:?}", cursor.position);
    for (node, transform, cursor_atraction) in &query {
        let scale = node.inverse_scale_factor();
        let center = transform.translation * scale;
        let radius = cursor_atraction.radius;
        let distance = cursor.position.distance(center);
        if distance < radius {
            let away = (cursor.position - center).normalize_or_zero();
            // 0 at the edge, 1 at the center — squared so it ramps hard near the middle
            let proximity = 1.0 - distance / radius;
            let speed = cursor_atraction.speed * proximity * proximity;
            cursor.position += away * time.delta_secs() * speed;
        }
    }
}
