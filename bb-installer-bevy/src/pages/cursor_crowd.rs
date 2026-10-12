//! A crowd of cursors. Decoys split off the real cursor one by one while it is
//! frozen, then the decoys wander. The one button only answers the real one.

use std::f32::consts::TAU;

use crate::AppState;
use crate::cursor::{FakeCursor, Fall, MouseSettings};
use bevy::feathers::controls::FeathersButton;
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::{SemanticToken, ThemeBackgroundColor, ThemedText, UiTheme};
use bevy::feathers::tokens;
use bevy::feathers::tokens::{BUTTON_BG, BUTTON_BG_HOVER, BUTTON_BG_PRESSED};
use bevy::input::mouse::MouseMotion;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::window::PrimaryWindow;

const WARNING_TEXT: &str = "Are you really really sure you want this?";
const WARNING_BG: SemanticToken = SemanticToken::new_static("bb.warning.bg");
const WARNING_BG_HOVER: SemanticToken = SemanticToken::new_static("bb.warning.bg.hover");
const WARNING_BG_PRESSED: SemanticToken = SemanticToken::new_static("bb.warning.bg.pressed");

/// Decoys per square pixel of window, so a small screen gets a smaller crowd.
/// About 300 on a 1920x1080 window.
const DECOY_DENSITY: f32 = 6.0e-4;
const MIN_DECOYS: usize = 20;
/// The real cursor is frozen while the decoys spawn, for this long.
const SPAWN_SECS: f32 = 4.0;

/// Gravity, in pixels per second squared.
const GRAVITY: f32 = 2200.0;
/// How much speed a bounce keeps.
const BOUNCE: f32 = 0.4;
/// Sideways speed and spin kept on every floor hit.
const FRICTION: f32 = 0.6;
/// Spin lost per second, so a cursor lying on others stops turning.
const SPIN_DRAG: f32 = 2.0;
/// Slower than this (pixels per second) counts as resting, and does not bounce.
const REST_SPEED: f32 = 60.0;
/// Collision radius of every cursor.
const RADIUS: f32 = 10.0;
const SUBSTEPS: usize = 2;
const SOLVER_PASSES: usize = 4;
/// The real cursor starts digging this long after the fall starts.
const RELEASE_SECS: f32 = 3.0;
/// While digging, the mouse sets the speed the real cursor aims for, up to
/// this, in pixels per second.
const DIG_SPEED: f32 = 260.0;
/// Each decoy pressing on the real cursor slows the dig by this much more, up
/// to `MAX_CONTACTS` of them.
const DIG_LOAD: f32 = 0.1;
/// Decoys within this distance of the real cursor count as pressing on it.
const DIG_TOUCH: f32 = 2.4 * RADIUS;
/// How quickly the real cursor reaches the speed it aims for, per second.
const DIG_GRIP: f32 = 8.0;
/// Pulls a digging cursor back down, so it sinks when the mouse stops. Keep
/// `DIG_GRAVITY / DIG_GRIP` (the sag, in pixels per second) well under the
/// slowest dig, or the cursor can't climb.
const DIG_GRAVITY: f32 = 12.0;
/// A digging cursor can't be pushed back by the pile, only slowed by
/// `DIG_LOAD`. Anything above zero lets a column of decoys cancel its speed
/// every solver pass, and it never gets out.
const DIG_INV_MASS: f32 = 0.0;
/// Random shoves while the mouse pushes harder than the cursor moves, in
/// pixels per second squared at full strain.
const WRIGGLE: f32 = 9000.0;
/// Random twisting at full strain, in radians per second squared.
const WRIGGLE_SPIN: f32 = 90.0;
/// How far a digging cursor leans into a sideways push, in radians.
const DIG_LEAN: f32 = 0.6;
/// How hard a digging cursor turns back upright, and how fast its spin dies.
const RIGHTING: f32 = 40.0;
const DIG_SPIN_DRAG: f32 = 8.0;
/// The mouse is fully back once nothing has touched the cursor for this long.
const FREE_SECS: f32 = 0.3;
/// Sideways pull toward the middle of the screen on everything that falls, in
/// pixels per second squared at the screen's edge. Fades to nothing at the
/// middle, so the cursors gather in one pile.
const IMPLOSION: f32 = 1500.0;
/// Once released, the real cursor is lighter than a decoy (more is lighter), so
/// the pile pushes it back harder than it pushes the pile.
const REAL_INV_MASS: f32 = 2.0;
/// Each decoy touching the real cursor cuts the mouse's reach by this much,
/// up to `MAX_CONTACTS` of them.
const DRAG_PER_CONTACT: f32 = 0.25;
const MAX_CONTACTS: usize = 8;
/// Caps how hard a fast mouse can throw the decoys, in pixels per second.
const MAX_MOUSE_SPEED: f32 = 1500.0;

const MIN_SPEED: f32 = 30.0;
const MAX_SPEED: f32 = 110.0;
/// How fast the turning itself changes, in radians per second squared.
const TURN_JITTER: f32 = 12.0;
const MAX_TURN: f32 = 3.0;
/// Each decoy follows the mouse at its own fraction of the real cursor's speed.
const MIN_GAIN: f32 = 0.6;
const MAX_GAIN: f32 = 1.4;
/// How quickly a decoy catches up with the mouse's speed, per second. Low is
/// sluggish and smooth, high is close to the raw mouse.
const MIN_RESPONSE: f32 = 3.0;
const MAX_RESPONSE: f32 = 12.0;

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
            (spawn_decoys, move_decoys, pile_up).run_if(in_state(AppState::CursorCrowd)),
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
    gain: f32,
    response: f32,
    /// The mouse-driven speed, smoothed, in pixels per second.
    pace: f32,
}

impl Wander {
    fn random() -> Self {
        Self {
            heading: rand::random_range(0.0..TAU),
            turn: 0.0,
            speed: rand::random_range(MIN_SPEED..MAX_SPEED),
            gain: rand::random_range(MIN_GAIN..MAX_GAIN),
            response: rand::random_range(MIN_RESPONSE..MAX_RESPONSE),
            pace: 0.0,
        }
    }

    /// Eases `pace` toward the mouse's speed, scaled by this decoy's gain.
    fn follow(&mut self, mouse_speed: f32, dt: f32) {
        let target = mouse_speed * self.gain;
        self.pace += (target - self.pace) * (1.0 - (-self.response * dt).exp());
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

#[derive(Component, Default, Clone)]
struct ContinueLabel;

/// Turns the buttons red. Only this page has buttons while it is open, and
/// `stop_crowd` puts the theme back.
fn redden_buttons(theme: &mut UiTheme) {
    let theme = &mut theme.0;
    for (token, semantic, color) in [
        (BUTTON_BG, WARNING_BG, Color::srgb(0.65, 0.1, 0.1)),
        (
            BUTTON_BG_HOVER,
            WARNING_BG_HOVER,
            Color::srgb(0.8, 0.15, 0.15),
        ),
        (
            BUTTON_BG_PRESSED,
            WARNING_BG_PRESSED,
            Color::srgb(0.45, 0.05, 0.05),
        ),
    ] {
        theme.semantic_base.insert(semantic.clone(), color);
        theme.token_assignments.insert(token, semantic);
    }
}

#[derive(Component)]
struct Decoy {
    position: Vec2,
}

#[derive(Resource)]
struct Crowd {
    image: Handle<Image>,
    /// How many decoys this window gets, from `DECOY_DENSITY`.
    total: usize,
    spawned: usize,
    elapsed: f32,
    /// Seconds since everything fell.
    fall_time: f32,
    /// Seconds the digging cursor has touched nothing.
    free_for: f32,
    /// Where the real cursor was at the end of the last step, once released.
    prev: Vec2,
}

fn start_crowd(
    mut commands: Commands,
    assets: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut cursor: ResMut<FakeCursor>,
) {
    cursor.frozen = true;
    cursor.wrap = true;
    cursor.hidden = true;
    let area = window.width() * window.height();
    commands.insert_resource(Crowd {
        image: assets.load(DECOY_IMAGE),
        total: ((area * DECOY_DENSITY) as usize).max(MIN_DECOYS),
        spawned: 0,
        elapsed: 0.0,
        fall_time: 0.0,
        free_for: 0.0,
        prev: Vec2::ZERO,
    });
}

fn stop_crowd(mut commands: Commands, mut cursor: ResMut<FakeCursor>, mut theme: ResMut<UiTheme>) {
    theme.0 = create_dark_theme();
    cursor.frozen = false;
    cursor.wrap = false;
    cursor.hidden = false;
    cursor.fall = None;
    commands.remove_resource::<Crowd>();
}

fn cursor_crowd_root() -> impl Scene {
    bsn! {
        Name::new("Cursor Crowd")
        DespawnOnExit::<AppState>(AppState::CursorCrowd)
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[
            @FeathersButton { @caption: bsn! { Text("Continue") ThemedText ContinueLabel} }
            Node { min_width: Val::Px(180.0) }
            on(
                |_activate: On<Activate>,
                 mut commands: Commands,
                 decoys: Query<Entity, With<Decoy>>,
                 fallen: Query<(), (With<Decoy>, With<Fall>)>,
                 mut cursor: ResMut<FakeCursor>,
                 mut state: ResMut<NextState<AppState>>,
                 mut theme: ResMut<UiTheme>,
                 mut labels: Query<&mut Text, With<ContinueLabel>>| {
                    // Still falling: the real cursor has no say yet.
                    if cursor.fall.is_some() {
                        return;
                    }
                    // The second click, once the cursor is back.
                    if !fallen.is_empty() {
                        state.set(AppState::Welcome);
                        return;
                    }
                    for decoy in &decoys {
                        commands.entity(decoy).insert(Fall::random());
                    }
                    cursor.fall = Some(Fall::random());
                    redden_buttons(&mut theme);
                    for mut label in &mut labels {
                        label.0 = WARNING_TEXT.into();
                    }
                },
            )
        ]
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
    let due = ((crowd.elapsed / SPAWN_SECS * crowd.total as f32) as usize).min(crowd.total);

    while crowd.spawned < due {
        crowd.spawned += 1;
        // Placed now, or it is drawn in the top left corner for its first frame.
        let top_left = cursor.position - HOTSPOT;
        commands.spawn((
            Name::new("Decoy Cursor"),
            DespawnOnExit(AppState::CursorCrowd),
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
        ));
    }

    if crowd.spawned == crowd.total && (cursor.frozen || cursor.hidden) {
        cursor.frozen = false;
        cursor.hidden = false;
    }
}

struct Body {
    pos: Vec2,
    vel: Vec2,
    angle: f32,
    spin: f32,
    inv_mass: f32,
    /// Moved by the mouse instead of by gravity.
    driven: bool,
    /// Passes through other cursors.
    ghost: bool,
    /// Digging: the speed it aims for, instead of falling with the rest.
    dig: Option<Vec2>,
}

impl Body {
    fn falling(pos: Vec2, fall: &Fall, inv_mass: f32) -> Self {
        Self {
            pos,
            vel: fall.velocity,
            angle: fall.angle,
            spin: fall.spin,
            inv_mass,
            driven: false,
            ghost: false,
            dig: None,
        }
    }
}

/// How many of `others` are within `reach` of `at`.
fn touching(at: Vec2, others: impl Iterator<Item = Vec2>, reach: f32) -> usize {
    others
        .filter(|p| p.distance_squared(at) < reach * reach)
        .count()
}

/// Drops every decoy and the real cursor, and lets them collide. After
/// `RELEASE_SECS` the real cursor digs: the mouse pushes it instead of placing
/// it, slowly, and slower for every decoy pressing on it. While the mouse
/// pushes harder than it moves, it wriggles. Once nothing has touched it for
/// `FREE_SECS`, the mouse places it again, but it still collides with the
/// decoys, and every one it touches holds it back.
///
/// Each cursor is a circle. Bodies are pushed apart and their speed along the
/// contact is exchanged, so they bounce off each other and pile up.
fn pile_up(
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    crowd: Option<ResMut<Crowd>>,
    settings: Res<MouseSettings>,
    mut motion: MessageReader<MouseMotion>,
    mut cursor: ResMut<FakeCursor>,
    mut decoys: Query<(&mut Decoy, &mut Fall, &mut Node, &mut UiTransform)>,
) {
    let cursor = &mut *cursor;
    let push: Vec2 = motion.read().map(|m| m.delta).sum::<Vec2>() * settings.speed;
    let Some(mut crowd) = crowd else {
        return;
    };
    // Nothing is falling before the first click.
    if cursor.fall.is_none() && decoys.is_empty() {
        return;
    }
    let size = window.size();
    let frame = time.delta_secs();
    let dt = frame.min(1.0 / 30.0) / SUBSTEPS as f32;
    let driven = cursor.fall.is_none();
    let digging = !driven && crowd.fall_time > RELEASE_SECS;

    if driven {
        // The mouse has already moved the cursor this frame. Slow that down by
        // however many decoys it is touching.
        let contacts = decoys
            .iter()
            .filter(|(d, ..)| d.position.distance(cursor.position) < 2.0 * RADIUS)
            .count()
            .min(MAX_CONTACTS);
        let moved = cursor.position - crowd.prev;
        cursor.position = crowd.prev + moved / (1.0 + DRAG_PER_CONTACT * contacts as f32);
    }

    // The real cursor is body 0, the decoys follow in query order.
    let real = match &cursor.fall {
        Some(fall) if digging => {
            let wanted = if frame > 0.0 {
                (push / frame).clamp_length_max(DIG_SPEED)
            } else {
                Vec2::ZERO
            };
            let load = touching(
                cursor.position,
                decoys.iter().map(|(d, ..)| d.position),
                DIG_TOUCH,
            );
            let target = wanted / (1.0 + DIG_LOAD * load.min(MAX_CONTACTS) as f32);
            // Pushing hard and getting nowhere: shove and twist at random,
            // which also jostles the decoys around it.
            let strain = ((wanted.length() - fall.velocity.length()) / DIG_SPEED).clamp(0.0, 1.0);
            let shove = Vec2::from_angle(rand::random_range(0.0..TAU)) * WRIGGLE * strain * frame;
            let twist = rand::random_range(-1.0..1.0) * WRIGGLE_SPIN * strain * frame;
            Body {
                vel: fall.velocity + shove,
                spin: fall.spin + twist,
                dig: Some(target),
                ..Body::falling(cursor.position, fall, DIG_INV_MASS)
            }
        }
        // Nothing stops it on the way down, so it ends up under the pile.
        Some(fall) => Body {
            ghost: true,
            ..Body::falling(cursor.position, fall, 1.0)
        },
        None => Body {
            pos: cursor.position,
            vel: if frame > 0.0 {
                ((cursor.position - crowd.prev) / frame).clamp_length_max(MAX_MOUSE_SPEED)
            } else {
                Vec2::ZERO
            },
            angle: 0.0,
            spin: 0.0,
            inv_mass: REAL_INV_MASS,
            driven: true,
            ghost: false,
            dig: None,
        },
    };
    let mut bodies = vec![real];
    bodies.extend(
        decoys
            .iter()
            .map(|(d, f, ..)| Body::falling(d.position, f, 1.0)),
    );

    for _ in 0..SUBSTEPS {
        for body in bodies.iter_mut().filter(|b| !b.driven) {
            if let Some(target) = body.dig {
                body.vel += (target - body.vel) * (1.0 - (-DIG_GRIP * dt).exp());
                body.vel.y += DIG_GRAVITY * dt;
                // Leans into a sideways push, and otherwise turns upright.
                let lean = target.x / DIG_SPEED * DIG_LEAN;
                body.spin += ((lean - body.angle) * RIGHTING - body.spin * DIG_SPIN_DRAG) * dt;
            } else {
                body.vel.y += GRAVITY * dt;
                body.vel.x += (size.x / 2.0 - body.pos.x) / (size.x / 2.0) * IMPLOSION * dt;
                body.spin *= (-SPIN_DRAG * dt).exp();
            }
            body.pos += body.vel * dt;
            body.angle += body.spin * dt;
        }
        for _ in 0..SOLVER_PASSES {
            collide_walls(&mut bodies, size);
            collide_pairs(&mut bodies);
        }
    }

    cursor.position = bodies[0].pos;
    crowd.prev = bodies[0].pos;
    if let Some(fall) = &mut cursor.fall {
        (fall.velocity, fall.angle, fall.spin) = (bodies[0].vel, bodies[0].angle, bodies[0].spin);
    }
    for ((mut decoy, mut fall, mut node, mut transform), body) in
        decoys.iter_mut().zip(&bodies[1..])
    {
        decoy.position = body.pos;
        (fall.velocity, fall.angle, fall.spin) = (body.vel, body.angle, body.spin);
        let top_left = body.pos - HOTSPOT;
        node.left = Val::Px(top_left.x);
        node.top = Val::Px(top_left.y);
        transform.rotation = Rot2::radians(body.angle);
    }

    crowd.fall_time += frame;
    if digging {
        let pressing = touching(bodies[0].pos, bodies[1..].iter().map(|b| b.pos), DIG_TOUCH);
        crowd.free_for = if pressing == 0 {
            crowd.free_for + frame
        } else {
            0.0
        };
        if crowd.free_for > FREE_SECS {
            cursor.fall = None;
            crowd.prev = cursor.position;
            // Wrapping would send a cursor lying on the floor up to the ceiling.
            cursor.wrap = false;
        }
    }
}

/// The floor and the two side walls. There is no ceiling.
fn collide_walls(bodies: &mut [Body], size: Vec2) {
    for body in bodies {
        if body.pos.x < 0.0 || body.pos.x > size.x {
            body.pos.x = body.pos.x.clamp(0.0, size.x);
            body.vel.x *= -BOUNCE;
        }
        if body.pos.y > size.y {
            body.pos.y = size.y;
            body.vel.y = if body.vel.y < REST_SPEED {
                0.0
            } else {
                -body.vel.y * BOUNCE
            };
            // A digging cursor would barely move along the floor otherwise.
            if body.dig.is_none() {
                body.vel.x *= FRICTION;
                body.spin *= FRICTION;
            }
        }
    }
}

fn collide_pairs(bodies: &mut [Body]) {
    let reach = 2.0 * RADIUS;
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            if bodies[i].ghost || bodies[j].ghost {
                continue;
            }
            let apart = bodies[j].pos - bodies[i].pos;
            let distance_sq = apart.length_squared();
            if distance_sq >= reach * reach {
                continue;
            }
            let distance = distance_sq.sqrt();
            // Exactly on top of each other: any direction will do.
            let normal = if distance > 1e-4 {
                apart / distance
            } else {
                Vec2::from_angle(rand::random_range(0.0..TAU))
            };

            // The lighter one gives way more.
            let (wi, wj) = (bodies[i].inv_mass, bodies[j].inv_mass);
            let total = wi + wj;
            let push = normal * (reach - distance);
            bodies[i].pos -= push * (wi / total);
            bodies[j].pos += push * (wj / total);

            let approach = (bodies[j].vel - bodies[i].vel).dot(normal);
            if approach < 0.0 {
                let bounce = if -approach < REST_SPEED { 0.0 } else { BOUNCE };
                let impulse = normal * (-(1.0 + bounce) * approach / total);
                bodies[i].vel -= impulse * wi;
                bodies[j].vel += impulse * wj;
            }
        }
    }
}

fn move_decoys(
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    cursor: Res<FakeCursor>,
    settings: Res<MouseSettings>,
    mut motion: MessageReader<MouseMotion>,
    mut decoys: Query<(&mut Decoy, &mut Wander, &mut Node), Without<Fall>>,
) {
    let dt = time.delta_secs();
    let bounds = window.size();
    // How far the real cursor moves from the mouse this frame.
    let mouse: Vec2 = motion.read().map(|m| m.delta).sum::<Vec2>() * settings.speed;
    let distance = if cursor.frozen { 0.0 } else { mouse.length() };
    let mouse_speed = if dt > 0.0 { distance / dt } else { 0.0 };

    for (mut decoy, mut wander, mut node) in &mut decoys {
        // Roughly as fast as the real cursor, along the decoy's own heading.
        wander.follow(mouse_speed, dt);
        let step = Vec2::from_angle(wander.heading) * wander.pace * dt;
        decoy.position = (decoy.position + step).rem_euclid(bounds);
        wander.step(&mut decoy.position, dt, bounds);
        let top_left = decoy.position - HOTSPOT;
        node.left = Val::Px(top_left.x);
        node.top = Val::Px(top_left.y);
    }
}
