use bevy::prelude::*;
use bevy::color::palettes::basic::PURPLE;
use bevy::ui::Val;
use bevy::ui_widgets::{Activate, observe};
use bevy::window::{PresentMode, WindowMode};
use bb_installer_bevy::AppState;

mod eye;
use eye::{spawn_crowd, EyeAnchor, EyePlugin, EYE_SIZE};

mod ui;
use ui::{button, button_system};

mod world;
use world::WorldPlugin;

/// Printed once the first frame has been rendered. The VM test matches on this.
const READY_MARKER: &str = "BB_BEVY_READY";

const BACKGROUND: Color = Color::srgb(0.05, 0.05, 0.07);

fn main() {
    App::new()
        .add_plugins(
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
        )
        .insert_resource(ClearColor(BACKGROUND))
        .init_state::<AppState>()
        .init_resource::<ReadyProbe>()
        .add_plugins((EyePlugin, MeshPickingPlugin))
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Welcome), spawn_welcome)
        .add_systems(OnEnter(AppState::Test), spawn_test)
        .add_systems(Update, (announce_ready, exit_on_request, button_system))
        .run();
}

#[derive(Resource, Default)]
struct ReadyProbe {
    frames: u32,
    announced: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_welcome(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(AppState::Welcome),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },
        // No BackgroundColor here: UI draws over the 2D world, so a filled
        // root node would hide the eye. The camera clears to BACKGROUND.
        children![
            (
                Text::new("BigBother"),
                TextFont {
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.1, 0.1)),
            ),
            (
                Text::new("Trigger warning!"),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.85)),
            ),
            (
                Node {
                    width: Val::Px(EYE_SIZE),
                    height: Val::Px(EYE_SIZE),
                    margin: UiRect::top(Val::Px(24.0)),
                    ..default()
                },
                EyeAnchor,
            ),
            (button("Continue"),
            observe(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| {
                state.set(AppState::Test);
            }),
        ),
        ],
    ));


    spawn_crowd(&mut commands, &assets);
}

fn spawn_test(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    commands.spawn((
        DespawnOnExit(AppState::Test),
        Mesh2d(meshes.add(Rectangle::new(128., 128.))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        observe(|_: On<Pointer<Over>>| info!("touched a wall")),
    ));
}

/// Waits a couple of frames so the renderer has definitely produced output,
/// then prints the marker the VM test looks for.
fn announce_ready(mut probe: ResMut<ReadyProbe>) {
    probe.frames += 1;
    if !probe.announced && probe.frames >= 3 {
        probe.announced = true;
        println!("{READY_MARKER}");
    }
}

/// Escape quits, and `BB_BEVY_SMOKE_TEST=1` makes the app exit on its own once
/// it has rendered. That keeps the smoke test from hanging forever.
fn exit_on_request(
    keys: Res<ButtonInput<KeyCode>>,
    probe: Res<ReadyProbe>,
    mut exit: MessageWriter<AppExit>,
) {
    let smoke_test = std::env::var("BB_BEVY_SMOKE_TEST").is_ok_and(|v| v == "1");
    if keys.just_pressed(KeyCode::Escape) || (smoke_test && probe.announced) {
        exit.write(AppExit::Success);
    }
}
