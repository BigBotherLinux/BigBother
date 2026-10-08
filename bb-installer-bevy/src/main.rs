use bb_installer_bevy::AppState;
use bb_installer_bevy::cursor::{CursorAtraction, FakeCursor, FakeCursorPlugin, drift};
use bb_installer_bevy::network::{
    NetworkConnectivityLabel, NetworkConnectivityStatus, NetworkPlugin, NetworkWatch,
};
use bb_installer_bevy::pages::cursor_crowd::CursorCrowdPlugin;
use bb_installer_bevy::pages::secondary_tos::SecondaryTosPlugin;
use bb_installer_bevy::pages::sign::SignPlugin;
use bb_installer_bevy::pages::tos::TosPlugin;
use bevy::feathers::controls::FeathersButton;
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::{ThemeBackgroundColor, ThemedText, UiTheme};
use bevy::feathers::{FeathersPlugins, tokens};
use bevy::input::common_conditions::input_just_pressed;
use bevy::input_focus::AutoFocus;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::*;
use bevy::ui::Val;
use bevy::ui_widgets::Activate;
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
                .set(ImagePlugin::default_nearest()),
            FeathersPlugins,
            FakeCursorPlugin,
            NetworkPlugin,
            TosPlugin,
            SecondaryTosPlugin,
            CursorCrowdPlugin,
            SignPlugin,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .init_state::<AppState>()
        .add_systems(Startup, startup)
        .add_systems(OnEnter(AppState::Welcome), welcome_root.spawn())
        //.add_systems(OnExit(AppState::Welcome), destroy_network_watch)
        .add_systems(Update, (exit_on_request, drift, update_network_status))
        .add_systems(Update, debug_stuff.run_if(input_just_pressed(KeyCode::F9)))
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
    //commands.insert_resource(NetworkWatch::every(Duration::from_secs(2)));
}

fn destroy_network_watch(mut commands: Commands) {
    commands.remove_resource::<NetworkWatch>();
}

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
            (Text::new("Network: Unknown") ThemedText NetworkConnectivityLabel),
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

/// `NetworkConnectivityStatus` is a resource, so it lives on its own entity,
/// not on the label. Read it with `Res` and update every label when it changes,
/// or when a label is freshly spawned (e.g. on re-entering a screen).
fn update_network_status(
    status: Res<NetworkConnectivityStatus>,
    labels: Query<(&mut Text, Ref<NetworkConnectivityLabel>)>,
) {
    for (mut text, label) in labels {
        if status.is_changed() || label.is_added() {
            text.0 = format!("Network: {}", *status);
        }
    }
}

fn exit_on_request(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
