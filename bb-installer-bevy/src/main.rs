use bb_installer_bevy::cursor::{FakeCursor, FakeCursorPlugin};
use bb_installer_bevy::AppState;
use bevy::feathers::controls::{FeathersButton, FeathersScrollbar};
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::{ThemeBackgroundColor, ThemedText, UiTheme};
use bevy::feathers::{tokens, FeathersPlugins};
use bevy::input::common_conditions::input_just_pressed;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::input_focus::AutoFocus;
use bevy::prelude::*;
use bevy::ui::Val;
use bevy::ui_widgets::{Activate, ControlOrientation, ScrollArea};
use bevy::window::{PresentMode, WindowMode};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BigBother Installer".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                // Pixel-art sprites: no smoothing when scaled up.
                .set(ImagePlugin::default_nearest()),
            FeathersPlugins,
            FakeCursorPlugin,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(AppState::Welcome), welcome_root.spawn())
        .add_systems(
            OnEnter(AppState::TermsOfService),
            terms_of_service_root.spawn(),
        )
        .add_systems(Update, (exit_on_request, drift))
        .add_systems(Update, debug_stuff.run_if(input_just_pressed(KeyCode::F9)))
        .run();
}

/// Shared by every screen, so it is not state scoped.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component, Default, Clone)]
struct CursorAtraction;

/// Logs where each `CursorAtraction` node is on screen, in logical pixels
/// (the same space as `FakeCursor::position`). Runs when F9 is pressed.
fn debug_stuff(
    cursor: Res<FakeCursor>,
    query: Query<(Entity, &ComputedNode, &UiGlobalTransform), With<CursorAtraction>>,
) {
    for (entity, node, transform) in &query {
        let scale = node.inverse_scale_factor();
        let center = transform.translation * scale;
        info!(
            "{entity}: center={center} cursor={}, distance={}",
            cursor.position,
            cursor.position.distance(center)
        );
    }
}

fn drift(
    mut cursor: ResMut<FakeCursor>,
    time: Res<Time>,
    query: Query<(&ComputedNode, &UiGlobalTransform), With<CursorAtraction>>,
) {
    //info!("Cursor offset: {:?}", cursor.position);
    for (node, transform) in &query {
        let scale = node.inverse_scale_factor();
        let center = transform.translation * scale;
        let radius = 75.0;
        let distance = cursor.position.distance(center);
        if distance < radius {
            let away = (cursor.position - center).normalize_or_zero();
            // 0 at the edge, 1 at the center — squared so it ramps hard near the middle
            let proximity = 1.0 - distance / radius;
            let speed = 50.0 * proximity * proximity;
            cursor.position += away * time.delta_secs() * speed;
        }
    }
}

fn welcome_root() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
        }
        DespawnOnExit::<AppState>(AppState::Welcome)
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[
            (Text::new("BigBother Installer") ThemedText),
            (@FeathersButton {
                @caption: bsn! { Text::new("Install") ThemedText }
            }
            AutoFocus
            CursorAtraction
            on(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| { state.set(AppState::TermsOfService) })
        )
        ]
    }
}

fn terms_of_service_root() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
        }
        DespawnOnExit::<AppState>(AppState::TermsOfService)
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[
            (Text::new("Terms of Service") ThemedText),
            // Outer frame: fixed size, with room on the right for the scrollbar.
            (
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect { right: Val::Px(10.0) },
                }
                Children[
                    // The part that actually scrolls. `ScrollArea` adds mouse wheel support.
                    (
                        #tos_text
                        Node {
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            overflow: Overflow::scroll_y(),
                        }
                        ScrollArea
                        Children[
                            (Text::new(TOS_TEXT) ThemedText)
                        ]
                    ),
                    @FeathersScrollbar {
                        @target: #tos_text,
                        @orientation: {ControlOrientation::Vertical}
                    }
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        width: Val::Px(6.0),
                    }
                ]
            ),
        ]
    }
}

const TOS_TEXT: &str = "This distribution is provided \"AS IS,\" with no warranty of any kind.

Installing will erase data or leave your system unbootable. 

You install at your own risk, and the authors are not liable for any damage or data loss.
This distribution installs non-free proprietary software (such as firmware, drivers, and codecs) under its owners' license terms, which you agree to follow.
By selecting \"I Agree,\" you accept these risks and consent to installing non-free software.";

fn exit_on_request(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
