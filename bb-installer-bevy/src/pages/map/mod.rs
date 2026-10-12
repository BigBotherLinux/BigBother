//! Location registration. A globe, a crosshair and a camera that can only go
//! up and down. The arrow keys turn the globe slowly, Space descends, letting
//! go climbs back up. Holding still at the lowest altitude for a few seconds
//! records the position under the crosshair. Both rounds start over where the
//! IP address says the user is.

mod geoip;
mod tiles;

use std::time::Duration;

use crate::AppState;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::feathers::controls::FeathersButton;
use bevy::feathers::theme::{ThemeBackgroundColor, ThemeTextColor, ThemedText};
use bevy::feathers::tokens;
use bevy::prelude::*;
use bevy::tasks::{Task, futures::check_ready};
use bevy::ui_widgets::Activate;
use geoip::Estimate;
use tiles::{Tiles, stream_tiles, to_sphere};

const USER_AGENT: &str = concat!(
    "BigBother-Installer/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/BigBotherLinux/BigBother)"
);

/// Vertical field of view of the globe camera.
pub const FOV: f32 = std::f32::consts::FRAC_PI_4;
/// Altitudes are in globe radii above the surface.
const MAX_ALTITUDE: f32 = 2.0;
/// About 3 km. Tiles come out at zoom 15–16 here.
const MIN_ALTITUDE: f32 = 5e-4;
/// Per second, as a fraction of the altitude.
const DESCENT_RATE: f32 = 1.4;
const CLIMB_RATE: f32 = 0.7;
/// How fast the ground moves under the crosshair, in screen heights per
/// second. The same at every altitude, which also keeps the tile requests down.
const GROUND_SPEED: f32 = 0.3;
/// The cap on turning when the whole globe is in view, in radians per second.
const MAX_TURN: f32 = 0.35;
/// The crosshair stays clear of the poles, where the map ends.
const MAX_LAT: f64 = 80.0_f64.to_radians();
const CONFIRM_SECS: f32 = 3.0;
const INTERLUDE_SECS: f32 = 2.5;
const ROUNDS: u8 = 2;
/// Further from the IP estimate than this, and the records disagree.
const DISAGREE_KM: f64 = 50.0;
const EARTH_KM: f64 = 6371.0;

const SPACE: Color = Color::srgb(0.02, 0.02, 0.04);
/// OpenStreetMap's water colour, for the polar caps the tiles do not cover.
const OCEAN: Color = Color::srgb(0.667, 0.827, 0.875);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Map), (map_root.spawn(), start_map))
            .add_systems(OnExit(AppState::Map), stop_map)
            .add_systems(
                Update,
                (
                    see_through_ui_camera,
                    locate,
                    fly,
                    interlude,
                    place_camera,
                    stream_tiles,
                    show_phase,
                )
                    .chain()
                    .run_if(in_state(AppState::Map)),
            );
    }
}

/// Where the camera is: over `lat`/`lon` (radians), `altitude` up.
#[derive(Resource)]
pub struct View {
    pub lat: f64,
    pub lon: f64,
    pub altitude: f32,
}

/// The second confirmed position, kept for later pages.
#[derive(Resource, Clone, Debug)]
pub struct ConfirmedLocation {
    pub lat: f64,
    pub lon: f64,
    pub estimate: Option<Estimate>,
}

enum Phase {
    Locating(Task<Option<Estimate>>),
    Round { round: u8, still: f32 },
    Interlude(f32),
    Done,
}

#[derive(Resource)]
struct Survey {
    phase: Phase,
    /// Where every round starts.
    home: (f64, f64),
    estimate: Option<Estimate>,
    picks: Vec<(f64, f64)>,
}

#[derive(Component)]
struct GlobeCamera;

#[derive(Component, FromTemplate)]
struct MapTitle;

#[derive(Component, FromTemplate)]
struct MapAside;

#[derive(Component, FromTemplate)]
struct MapCountdown;

#[derive(Component, FromTemplate)]
struct MapContinue;

fn agent() -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .user_agent(USER_AGENT)
            .timeout_global(Some(Duration::from_secs(15)))
            .build(),
    )
}

fn start_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Globe Camera"),
        DespawnOnExit(AppState::Map),
        GlobeCamera,
        Camera3d::default(),
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(SPACE),
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: FOV,
            ..default()
        }),
        // Keep the map's own colours.
        Tonemapping::None,
    ));
    commands.spawn((
        Name::new("Ocean"),
        DespawnOnExit(AppState::Map),
        Mesh3d(meshes.add(Sphere::new(0.99).mesh().uv(64, 32))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: OCEAN,
            unlit: true,
            ..default()
        })),
    ));

    let agent = agent();
    commands.insert_resource(Survey {
        phase: Phase::Locating(geoip::lookup(agent.clone())),
        home: (0.0, 0.0),
        estimate: None,
        picks: Vec::new(),
    });
    commands.insert_resource(Tiles::new(agent));
    commands.insert_resource(View {
        lat: 0.0,
        lon: 0.0,
        altitude: MAX_ALTITUDE,
    });
}

/// The UI camera draws over the globe instead of clearing it. Done every frame
/// rather than on enter: as the first state, this page is entered before
/// `Startup` has spawned the UI camera.
fn see_through_ui_camera(mut cameras: Query<&mut Camera, With<Camera2d>>) {
    for mut camera in &mut cameras {
        if !matches!(camera.clear_color, ClearColorConfig::None) {
            camera.clear_color = ClearColorConfig::None;
        }
    }
}

fn stop_map(mut commands: Commands, mut cameras: Query<&mut Camera, With<Camera2d>>) {
    for mut camera in &mut cameras {
        camera.clear_color = ClearColorConfig::Default;
    }
    commands.remove_resource::<Survey>();
    commands.remove_resource::<Tiles>();
    commands.remove_resource::<View>();
}

fn map_root() -> impl Scene {
    bsn! {
        Name::new("Map")
        DespawnOnExit::<AppState>(AppState::Map)
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        }
        Children[
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(24.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    justify_content: JustifyContent::Center,
                }
                Children[
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(8.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                    }
                    ThemeBackgroundColor(tokens::WINDOW_BG)
                    Children[
                        Text::new("Location Registration") ThemedText MapTitle
                        --
                        Text::new("") ThemeTextColor(tokens::TEXT_DIM) MapAside
                        --

                            @FeathersButton { @caption: bsn! { Text("Continue") ThemedText} }
                            Node { display: Display::None, width: Val::Px(180.0) }
                            on(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| {
                                state.set(AppState::Sign);
                            })
                            MapContinue

                    ]
                ]
            --
            // The crosshair, centred on the screen.

                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(36.0),
                    height: Val::Px(36.0),
                    margin: UiRect { left: Val::Px(-18.0), top: Val::Px(-18.0) },
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::MAX,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                BorderColor::all(Color::srgb(0.75, 0.05, 0.05))
                Pickable::IGNORE
                Children[
                    Node {
                        width: Val::Px(4.0),
                        height: Val::Px(4.0),
                        border_radius: BorderRadius::MAX,
                    }
                    BackgroundColor(Color::srgb(0.75, 0.05, 0.05))
                    Pickable::IGNORE
                ]
            --

                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(50.0),
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    margin: UiRect::top(Val::Px(32.0)),
                    justify_content: JustifyContent::Center,
                }
                Pickable::IGNORE
                Children[Text::new("") TextColor(Color::srgb(0.75, 0.05, 0.05)) Pickable::IGNORE MapCountdown]
            --
            // Required by the tile usage policy.

                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                }
                ThemeBackgroundColor(tokens::WINDOW_BG)
                Children[Text::new("© OpenStreetMap contributors") ThemeTextColor(tokens::TEXT_DIM)]

        ]
    }
}

/// Waits for the IP lookup, then starts the first round over its answer.
fn locate(mut survey: ResMut<Survey>, mut view: ResMut<View>) {
    let Phase::Locating(task) = &mut survey.phase else {
        return;
    };
    let Some(estimate) = check_ready(task) else {
        return;
    };
    if let Some(estimate) = &estimate {
        survey.home = (
            estimate.latitude.to_radians(),
            estimate.longitude.to_radians(),
        );
    }
    survey.estimate = estimate;
    (view.lat, view.lon) = survey.home;
    survey.phase = Phase::Round {
        round: 1,
        still: 0.0,
    };
}

fn fly(
    mut commands: Commands,
    mut survey: ResMut<Survey>,
    mut view: ResMut<View>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let Phase::Round { round, still } = &mut survey.phase else {
        return;
    };

    let axis = |plus, minus| (keys.pressed(plus) as i8 - keys.pressed(minus) as i8) as f32;
    let turn = Vec2::new(
        axis(KeyCode::ArrowRight, KeyCode::ArrowLeft),
        axis(KeyCode::ArrowUp, KeyCode::ArrowDown),
    );
    let visible = 2.0 * view.altitude * (FOV / 2.0).tan();
    let speed = (GROUND_SPEED * visible).min(MAX_TURN) * dt;
    view.lat = (view.lat + f64::from(turn.y * speed)).clamp(-MAX_LAT, MAX_LAT);
    // A degree of longitude gets shorter towards the poles.
    view.lon += f64::from(turn.x * speed) / view.lat.cos();
    view.lon =
        (view.lon + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI;

    let descending = keys.pressed(KeyCode::Space);
    let rate = if descending {
        -DESCENT_RATE
    } else {
        CLIMB_RATE
    };
    view.altitude = (view.altitude * (rate * dt).exp()).clamp(MIN_ALTITUDE, MAX_ALTITUDE);

    let holding = descending && view.altitude <= MIN_ALTITUDE && turn == Vec2::ZERO;
    *still = if holding { *still + dt } else { 0.0 };
    if *still < CONFIRM_SECS {
        return;
    }

    let round = *round;
    survey.picks.push((view.lat, view.lon));
    info!(
        "Round {round}: {:.5}, {:.5}",
        view.lat.to_degrees(),
        view.lon.to_degrees()
    );
    if round < ROUNDS {
        survey.phase = Phase::Interlude(0.0);
        return;
    }
    survey.phase = Phase::Done;
    commands.insert_resource(ConfirmedLocation {
        lat: view.lat,
        lon: view.lon,
        estimate: survey.estimate.clone(),
    });
}

/// A pause after each round, then back to where the records say.
fn interlude(mut survey: ResMut<Survey>, mut view: ResMut<View>, time: Res<Time>) {
    let Phase::Interlude(elapsed) = &mut survey.phase else {
        return;
    };
    *elapsed += time.delta_secs();
    if *elapsed < INTERLUDE_SECS {
        return;
    }
    let round = survey.picks.len() as u8 + 1;
    (view.lat, view.lon) = survey.home;
    view.altitude = MAX_ALTITUDE;
    survey.phase = Phase::Round { round, still: 0.0 };
}

fn place_camera(
    view: Res<View>,
    camera: Single<(&mut Transform, &mut Projection), With<GlobeCamera>>,
) {
    let (mut transform, mut projection) = camera.into_inner();
    let at = to_sphere(view.lat, view.lon) * (1.0 + view.altitude);
    *transform = Transform::from_translation(at).looking_at(Vec3::ZERO, Vec3::Y);
    if let Projection::Perspective(perspective) = projection.as_mut() {
        perspective.near = view.altitude * 0.1;
    }
}

/// The one `Text` marked `M`, and not `A` or `B`.
type MapText<'w, 's, M, A, B> =
    Single<'w, 's, &'static mut Text, (With<M>, Without<A>, Without<B>)>;

fn show_phase(
    survey: Res<Survey>,
    mut title: MapText<MapTitle, MapAside, MapCountdown>,
    mut aside: MapText<MapAside, MapTitle, MapCountdown>,
    mut countdown: MapText<MapCountdown, MapTitle, MapAside>,
    mut continue_button: Single<&mut Node, With<MapContinue>>,
) {
    let (new_title, new_aside, new_countdown) = match &survey.phase {
        Phase::Locating(_) => (
            "Location Registration".into(),
            "Retrieving your file.".into(),
            String::new(),
        ),
        Phase::Round { round: 1, still } => (
            "Indicate your exact location.".into(),
            if survey.estimate.is_some() {
                "Arrow keys to move. Hold Space to descend. Exact means exact."
            } else {
                "No file was found. One has been opened. Arrow keys to move, hold Space to descend."
            }
            .into(),
            countdown_text(*still),
        ),
        Phase::Round { still, .. } => (
            "Please confirm your location.".into(),
            "Approximately is not a location.".into(),
            countdown_text(*still),
        ),
        Phase::Interlude(_) => (
            "Location received.".into(),
            "A second reading is required.".into(),
            String::new(),
        ),
        Phase::Done => (
            "Location confirmed.".into(),
            verdict(&survey),
            String::new(),
        ),
    };
    set_text(&mut title, new_title);
    set_text(&mut aside, new_aside);
    set_text(&mut countdown, new_countdown);

    let display = if matches!(survey.phase, Phase::Done) {
        Display::Flex
    } else {
        Display::None
    };
    if continue_button.display != display {
        continue_button.display = display;
    }
}

fn set_text(text: &mut Text, value: String) {
    if text.0 != value {
        text.0 = value;
    }
}

fn countdown_text(still: f32) -> String {
    if still <= 0.0 {
        return String::new();
    }
    format!("Hold still. {}", (CONFIRM_SECS - still).ceil() as u32)
}

fn verdict(survey: &Survey) -> String {
    let [first, second] = survey.picks[..] else {
        return String::new();
    };
    let mut lines = vec![format!(
        "Your answers differ by {}. The difference has been noted.",
        format_distance(distance_km(first, second))
    )];
    if let Some(estimate) = &survey.estimate
        && distance_km(second, survey.home) > DISAGREE_KM
    {
        let place = estimate.city.as_deref().unwrap_or("another location");
        lines.push(format!(
            "Our records place you in {place}. Our records have not been updated."
        ));
    }
    lines.join("\n")
}

/// Great-circle distance between two lat/lon pairs in radians.
fn distance_km(a: (f64, f64), b: (f64, f64)) -> f64 {
    let h = ((b.0 - a.0) / 2.0).sin().powi(2)
        + a.0.cos() * b.0.cos() * ((b.1 - a.1) / 2.0).sin().powi(2);
    2.0 * EARTH_KM * h.sqrt().min(1.0).asin()
}

fn format_distance(km: f64) -> String {
    if km < 1.0 {
        format!("{} metres", (km * 1000.0).round())
    } else {
        format!("{km:.1} km")
    }
}
