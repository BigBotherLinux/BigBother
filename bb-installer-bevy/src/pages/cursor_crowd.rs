//! A crowd of cursors. Decoys split off the real cursor one by one while it is
//! frozen, then the decoys wander. The one button only answers the real one.

use std::f32::consts::TAU;

use crate::AppState;
use crate::cursor::{FakeCursor, MouseSettings};
use bevy::feathers::controls::FeathersButton;
use bevy::feathers::theme::{ThemeBackgroundColor, ThemedText};
use bevy::feathers::tokens;
use bevy::input::mouse::MouseMotion;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::window::PrimaryWindow;

const DECOYS: usize = 300;
/// The real cursor is frozen while the decoys spawn, for this long.
const SPAWN_SECS: f32 = 4.0;

const MIN_SPEED: f32 = 30.0;
const MAX_SPEED: f32 = 110.0;
/// How fast the turning itself changes, in radians per second squared.
const TURN_JITTER: f32 = 12.0;
const MAX_TURN: f32 = 3.0;

/// Same image and hotspot as the real cursor's default shape.
const DECOY_IMAGE: &str = "cursors/default.png";
const HOTSPOT: Vec2 = Vec2::new(3.0, 1.0);

pub struct CursorCrowdPlugin;

impl Plugin for CursorCrowdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::CursorCrowd),
            (cursor_crowd_root.spawn(), start_crowd),
        )
        .add_systems(OnExit(AppState::CursorCrowd), stop_crowd)
        .add_systems(
            Update,
            (spawn_decoys, move_decoys).run_if(in_state(AppState::CursorCrowd)),
        );
    }
}

/// A random walk with momentum: the heading turns smoothly, and how fast it
/// turns drifts at random.
#[derive(Component, Clone)]
struct Wander {
    heading: f32,
    turn: f32,
    speed: f32,
}

impl Wander {
    fn random() -> Self {
        Self {
            heading: rand::random_range(0.0..TAU),
            turn: 0.0,
            speed: rand::random_range(MIN_SPEED..MAX_SPEED),
        }
    }

    /// Moves `at` one step. Leaving `bounds` on one side comes back in on the
    /// other, the same as the real cursor does on this page.
    fn step(&mut self, at: &mut Vec2, dt: f32, bounds: Vec2) {
        self.turn = (self.turn + rand::random_range(-1.0..1.0) * TURN_JITTER * dt)
            .clamp(-MAX_TURN, MAX_TURN);
        self.heading += self.turn * dt;
        *at = (*at + Vec2::from_angle(self.heading) * self.speed * dt).rem_euclid(bounds);
    }
}

#[derive(Component)]
struct Decoy {
    position: Vec2,
}

#[derive(Resource)]
struct Crowd {
    image: Handle<Image>,
    spawned: usize,
    elapsed: f32,
}

fn start_crowd(mut commands: Commands, assets: Res<AssetServer>, mut cursor: ResMut<FakeCursor>) {
    cursor.frozen = true;
    cursor.wrap = true;
    commands.insert_resource(Crowd {
        image: assets.load(DECOY_IMAGE),
        spawned: 0,
        elapsed: 0.0,
    });
}

fn stop_crowd(mut commands: Commands, mut cursor: ResMut<FakeCursor>) {
    cursor.frozen = false;
    cursor.wrap = false;
    commands.remove_resource::<Crowd>();
}

fn cursor_crowd_root() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        DespawnOnExit::<AppState>(AppState::CursorCrowd)
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[(
            @FeathersButton { @caption: bsn! { Text("Continue") ThemedText} }
            Node { width: Val::Px(180.0) }
            on(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| {
                state.set(AppState::Sign);
            })
        )]
    }
}

/// Splits decoys off the real cursor, evenly over `SPAWN_SECS`, then lets the
/// real one go.
fn spawn_decoys(
    mut commands: Commands,
    time: Res<Time>,
    crowd: Option<ResMut<Crowd>>,
    mut cursor: ResMut<FakeCursor>,
) {
    let Some(mut crowd) = crowd else {
        return;
    };
    crowd.elapsed += time.delta_secs();
    let due = ((crowd.elapsed / SPAWN_SECS * DECOYS as f32) as usize).min(DECOYS);

    while crowd.spawned < due {
        crowd.spawned += 1;
        // Placed now, or it is drawn in the top left corner for its first frame.
        let top_left = cursor.position - HOTSPOT;
        commands.spawn((
            Decoy {
                position: cursor.position,
            },
            Wander::random(),
            ImageNode::new(crowd.image.clone()),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(top_left.x),
                top: Val::Px(top_left.y),
                ..default()
            },
            // Under the real cursor, over everything else.
            GlobalZIndex(i32::MAX - 1),
            Pickable::IGNORE,
            DespawnOnExit(AppState::CursorCrowd),
        ));
    }

    if crowd.spawned == DECOYS && cursor.frozen {
        cursor.frozen = false;
    }
}

fn move_decoys(
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    cursor: Res<FakeCursor>,
    settings: Res<MouseSettings>,
    mut motion: MessageReader<MouseMotion>,
    mut decoys: Query<(&mut Decoy, &mut Wander, &mut Node)>,
) {
    let dt = time.delta_secs();
    let bounds = window.size();
    // How far the real cursor moves from the mouse this frame.
    let mouse: Vec2 = motion.read().map(|m| m.delta).sum::<Vec2>() * settings.speed;
    let distance = if cursor.frozen { 0.0 } else { mouse.length() };

    for (mut decoy, mut wander, mut node) in &mut decoys {
        // As far as the real cursor went, along the decoy's own heading.
        let step = Vec2::from_angle(wander.heading) * distance;
        decoy.position = (decoy.position + step).rem_euclid(bounds);
        wander.step(&mut decoy.position, dt, bounds);
        let top_left = decoy.position - HOTSPOT;
        node.left = Val::Px(top_left.x);
        node.top = Val::Px(top_left.y);
    }
}
