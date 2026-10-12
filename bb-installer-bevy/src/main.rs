use bb_installer_bevy::AppState;
use bb_installer_bevy::cursor::{CursorAtraction, FakeCursor, FakeCursorPlugin};
use bb_installer_bevy::network::NetworkPlugin;
use bb_installer_bevy::pages::cursor_crowd::CursorCrowdPlugin;
use bb_installer_bevy::pages::map::MapPlugin;
use bb_installer_bevy::pages::secondary_tos::SecondaryTosPlugin;
use bb_installer_bevy::pages::sign::SignPlugin;
use bb_installer_bevy::pages::tos::TosPlugin;
use bb_installer_bevy::pages::welcome::WelcomePlugin;
use bevy::feathers::FeathersPlugins;
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::UiTheme;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
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
            WelcomePlugin,
            TosPlugin,
            SecondaryTosPlugin,
            CursorCrowdPlugin,
            MapPlugin,
            SignPlugin,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .init_state::<AppState>()
        .add_systems(Startup, startup)
        .add_systems(Update, exit_on_request)
        .add_systems(Update, debug_stuff.run_if(input_just_pressed(KeyCode::F9)))
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Name::new("UI Camera"), Camera2d));
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

fn exit_on_request(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
