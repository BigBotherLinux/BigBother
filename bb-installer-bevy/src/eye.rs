//! Eyes: the sprites that watch you install.
//!
//! Add [`EyePlugin`] for the behaviour, put an [`EyeAnchor`] node in the layout
//! for the big one to sit in, and call [`spawn_crowd`] once to populate the
//! screen.

use bevy::prelude::*;
use bevy::window::{CursorMoved, PrimaryWindow};
use rand::RngExt;
use std::ops::Range;

/// Registers everything the eyes need. The placement systems run before the
/// gaze ones so an iris aims from where its eyeball ended up this frame.
pub struct EyePlugin;

impl Plugin for EyePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                place_anchored_eyes,
                place_screen_eyes,
                wander_gaze,
                look_at_cursor,
            )
                .chain(),
        );
    }
}

/// Spawns the anchored eye and the whole [`CROWD`] around it.
pub fn spawn_crowd(commands: &mut Commands, assets: &AssetServer) {
    // The big one sits in the hole the layout left for it, and it hardly ever
    // takes its eye off you.
    spawn_eye(
        commands,
        assets,
        EyeSpec::anchored().with_attention(Attention {
            distraction_rate: 0.08,
            ..default()
        }),
    );

    // // The rest are scattered around it, so you are never quite unobserved.
    // for &(at, scale, iris, response, temperament) in CROWD {
    //     spawn_eye(
    //         commands,
    //         assets,
    //         EyeSpec::at_screen(at)
    //             .with_scale(scale)
    //             .with_iris(iris)
    //             .with_response(response)
    //             .with_attention(temperament()),
    //     );
    // }
}

/// One row of [`CROWD`]: position, scale, iris sprite, gaze response,
/// temperament.
type CrowdEye = (Vec2, f32, u8, f32, fn() -> Attention);

/// The crowd, one eye per row.
///
/// Position is a fraction of the window's half-extent from its centre, so the
/// middle is kept clear for the title and the anchored eye. Keep `|y|` under
/// about `0.8` for the big ones or they hang off the edge on a 16:9 screen.
#[rustfmt::skip]
const CROWD: &[CrowdEye] = &[
    // Upper band: a row of watchers over the title.
    (Vec2::new(-0.82,  0.68), 2.0, 3, 5.0,  Attention::bored),
    (Vec2::new(-0.55,  0.78), 1.4, 1, 9.0,  Attention::twitchy),
    (Vec2::new(-0.28,  0.66), 1.0, 7, 12.0, Attention::shifty),
    (Vec2::new( 0.02,  0.79), 1.6, 5, 7.0,  Attention::drowsy),
    (Vec2::new( 0.31,  0.64), 1.2, 2, 15.0, Attention::twitchy),
    (Vec2::new( 0.58,  0.76), 2.2, 8, 6.0,  Attention::vigilant),
    (Vec2::new( 0.85,  0.62), 1.8, 4, 10.0, Attention::bored),
    // Flanks: bigger, closer, harder to ignore.
    (Vec2::new(-0.88,  0.18), 3.0, 0, 4.0,  Attention::unblinking),
    (Vec2::new(-0.74, -0.24), 1.6, 6, 11.0, Attention::shifty),
    (Vec2::new( 0.76,  0.14), 2.6, 2, 8.0,  Attention::vigilant),
    (Vec2::new( 0.90, -0.30), 1.2, 8, 16.0, Attention::twitchy),
    // Lower band: the ones watching your hands.
    (Vec2::new(-0.86, -0.66), 2.4, 5, 6.0,  Attention::drowsy),
    (Vec2::new(-0.52, -0.78), 1.0, 3, 18.0, Attention::twitchy),
    (Vec2::new(-0.22, -0.64), 1.8, 7, 9.0,  Attention::bored),
    (Vec2::new( 0.08, -0.80), 1.4, 0, 13.0, Attention::shifty),
    (Vec2::new( 0.38, -0.62), 2.8, 1, 5.0,  Attention::unblinking),
    (Vec2::new( 0.66, -0.79), 1.6, 4, 10.0, Attention::vigilant),
    (Vec2::new( 0.88, -0.58), 2.2, 6, 7.0,  Attention::drowsy),
];

// ---------------------------------------------------------------------------
// The eye: pixel-art sprites in 2D world space.
// ---------------------------------------------------------------------------

/// Marks the UI node whose screen position an anchored eye follows. Give it a
/// box of [`EYE_SIZE`] so the layout leaves room for the eye.
///
/// `Default` and `Clone` so it can be named inside a `bsn!` scene.
#[derive(Component, Default, Clone)]
pub struct EyeAnchor;

/// Marks the eyeball sprite; iris and lids are its children.
#[derive(Component)]
struct Eye;

/// Marks the iris; it is offset inside its own eyeball towards the cursor.
#[derive(Component)]
struct Iris {
    /// How briskly the gaze catches up, in e-foldings per second. Small values
    /// drift lazily; large ones snap to the cursor.
    response: f32,
}

/// How easily an eye loses interest in you. Lives on the iris next to
/// [`Iris`]; every field is a knob you can turn per eye.
#[derive(Component, Clone)]
struct Attention {
    /// Expected glances away per second of watching. `0.0` means the eye never
    /// looks away; `0.2` is roughly one glance every five seconds. The draw is
    /// memoryless, so the wait between glances varies on its own.
    distraction_rate: f32,
    /// How long one glance lasts, in seconds, sampled uniformly.
    glance_secs: Range<f32>,
    /// How far off the glance goes, as a fraction of how far the iris can
    /// travel. `1.0` puts it against the rim.
    glance_reach: Range<f32>,
    /// Directions a glance may pick, in radians, measured counter-clockwise
    /// from "right". The default is the whole circle; narrow it to make an eye
    /// that only ever shifts, say, up and to the left.
    glance_angle: Range<f32>,
    /// How much the pointer has to move in one frame, in logical pixels, to cut
    /// a glance short and drag the eye back to you. `0.0` means the slightest
    /// twitch does it; `f32::INFINITY` means movement is ignored and glances
    /// always run their full length.
    alert_motion: f32,
    /// Seconds of undivided attention the pointer buys. While this is running
    /// the eye cannot wander off, and every further movement starts it over, so
    /// eyes only lose interest once you have held still this long.
    focus_secs: f32,
}

impl Default for Attention {
    fn default() -> Self {
        Self {
            distraction_rate: 0.18,
            glance_secs: 0.5..1.6,
            glance_reach: 0.5..1.0,
            glance_angle: 0.0..std::f32::consts::TAU,
            alert_motion: 1.5,
            focus_secs: 2.0,
        }
    }
}

/// Ready-made temperaments. Each is just [`Attention`] with some knobs turned,
/// so a one-off eye can still spell out its own.
impl Attention {
    /// Never breaks off staring at you.
    fn unblinking() -> Self {
        Self {
            distraction_rate: 0.0,
            ..default()
        }
    }

    /// Rarely looks away, and never far.
    fn vigilant() -> Self {
        Self {
            distraction_rate: 0.07,
            glance_secs: 0.3..0.8,
            glance_reach: 0.3..0.6,
            focus_secs: 4.0,
            ..default()
        }
    }

    /// Frequent, brief, shallow flicks away, and quick to snap back.
    fn twitchy() -> Self {
        Self {
            distraction_rate: 0.9,
            glance_secs: 0.15..0.4,
            glance_reach: 0.3..0.7,
            alert_motion: 0.5,
            focus_secs: 0.8,
            ..default()
        }
    }

    /// Easily bored: long, wandering glances.
    fn bored() -> Self {
        Self {
            distraction_rate: 0.35,
            glance_secs: 0.8..2.5,
            ..default()
        }
    }

    /// Stares off into the middle distance and does not care that you moved.
    fn drowsy() -> Self {
        Self {
            distraction_rate: 0.5,
            glance_secs: 1.5..4.0,
            glance_reach: 0.8..1.0,
            alert_motion: f32::INFINITY,
            ..default()
        }
    }

    /// Keeps glancing off to the left, as if something over there is more
    /// interesting than you.
    fn shifty() -> Self {
        use std::f32::consts::PI;
        Self {
            distraction_rate: 0.6,
            glance_secs: 0.3..0.9,
            glance_reach: 0.7..1.0,
            glance_angle: 0.65 * PI..1.35 * PI,
            alert_motion: 3.0,
            focus_secs: 1.0,
        }
    }
}

/// What an iris is currently doing. Driven by [`wander_gaze`], read by
/// [`look_at_cursor`].
#[derive(Component)]
enum Gaze {
    /// Tracking the cursor.
    Watching {
        /// Seconds of guaranteed attention left: while this is above zero the
        /// eye cannot be distracted. Refilled to
        /// [`Attention::focus_secs`](Attention) whenever the pointer moves.
        focus: f32,
    },
    /// Looking somewhere else entirely for a moment.
    Distracted {
        /// Offset from the eyeball's centre, in sprite pixels.
        offset: Vec2,
        /// Seconds left before it goes back to watching.
        remaining: f32,
    },
}

impl Default for Gaze {
    fn default() -> Self {
        Self::Watching { focus: 0.0 }
    }
}

/// This eye follows the [`EyeAnchor`] UI node instead of a fixed spot.
#[derive(Component)]
struct UiAnchored;

/// Where an eye sits, as a fraction of the window's half-extent from its
/// centre: `(0, 0)` is the middle, `(-1, 1)` the top-left corner.
#[derive(Component)]
struct ScreenPlacement(Vec2);

/// Sprite pixels → world units, for an eye that does not say otherwise.
const SPRITE_SCALE: f32 = 4.0;
/// The sprites are 64×64 with the eyeball drawn 41 px wide, not quite
/// centred on the canvas. These are measured from the pngs.
const SPRITE_PX: f32 = 64.0;
const GLOBE_RADIUS_PX: f32 = 20.5;
const IRIS_RADIUS_PX: f32 = 9.5;
/// Where the eyeball's centre sits relative to the canvas centre (y up).
const GLOBE_CENTRE_PX: Vec2 = Vec2::new(-1.5, 1.0);

/// Side of the box an [`EyeAnchor`] node should reserve, in logical pixels.
pub const EYE_SIZE: f32 = SPRITE_PX * SPRITE_SCALE;
/// How far the iris may drift from the eyeball's centre, in sprite pixels,
/// before it would poke out past the rim.
const IRIS_TRAVEL_PX: f32 = GLOBE_RADIUS_PX - IRIS_RADIUS_PX - 1.0;
/// `Iris0.png` … `Iris8.png`.
const IRIS_VARIANTS: u8 = 9;
/// Default gaze responsiveness: roughly a tenth of a second of lag.
const IRIS_RESPONSE: f32 = 9.0;

/// Where an eye goes. Every eye is otherwise self-contained: it owns its own
/// iris and looks around on its own.
enum EyePlacement {
    /// Follow the spot the UI layout reserved via [`EyeAnchor`].
    UiAnchor,
    /// Sit at a fraction of the window's half-extent from its centre, so the
    /// eye keeps its spot whatever the resolution is.
    Screen(Vec2),
}

/// Recipe for one eye. Build with [`EyeSpec::anchored`] or
/// [`EyeSpec::at_screen`], then tweak with the `with_*` methods.
struct EyeSpec {
    placement: EyePlacement,
    scale: f32,
    iris: u8,
    response: f32,
    attention: Attention,
}

impl EyeSpec {
    fn anchored() -> Self {
        Self::new(EyePlacement::UiAnchor)
    }

    /// `position` is a fraction of the window's half-extent from its centre:
    /// `(0, 0)` is the middle, `(-1, 1)` the top-left corner.
    fn at_screen(position: Vec2) -> Self {
        Self::new(EyePlacement::Screen(position))
    }

    fn new(placement: EyePlacement) -> Self {
        Self {
            placement,
            scale: SPRITE_SCALE,
            iris: 0,
            response: IRIS_RESPONSE,
            attention: Attention::default(),
        }
    }

    fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    fn with_iris(mut self, iris: u8) -> Self {
        self.iris = iris % IRIS_VARIANTS;
        self
    }

    /// How briskly this eye's gaze catches up with the cursor, in e-foldings
    /// per second: ~3 is a sleepy drift, ~20 is nearly instant.
    fn with_response(mut self, response: f32) -> Self {
        self.response = response;
        self
    }

    /// How readily this eye's attention wanders. See [`Attention`] for the
    /// individual knobs; `Attention { distraction_rate: 0.6, ..default() }`
    /// only changes the one you name.
    fn with_attention(mut self, attention: Attention) -> Self {
        self.attention = attention;
        self
    }
}

pub fn spawn_eye(commands: &mut Commands, assets: &AssetServer, spec: EyeSpec) {
    let eye = commands
        .spawn((
            Sprite::from_image(assets.load("Eyes_V2/EyeGlobe_sprite_1.png")),
            Transform::from_scale(Vec3::splat(spec.scale)),
            Eye,
            children![(
                Sprite::from_image(assets.load(format!("Eyes_V2/Iris{}.png", spec.iris))),
                // Children inherit the parent's scale, so this is in sprite px.
                Transform::from_translation(GLOBE_CENTRE_PX.extend(1.0)),
                Iris {
                    response: spec.response
                },
                spec.attention.clone(),
                Gaze::default(),
            ),],
        ))
        .id();

    match spec.placement {
        EyePlacement::UiAnchor => commands.entity(eye).insert(UiAnchored),
        EyePlacement::Screen(position) => commands.entity(eye).insert(ScreenPlacement(position)),
    };
}

/// Keeps anchored eyes glued to the spot the UI layout reserved for them.
fn place_anchored_eyes(
    camera: Single<(&Camera, &GlobalTransform)>,
    anchor: Single<(&ComputedNode, &UiGlobalTransform), With<EyeAnchor>>,
    mut eyes: Query<&mut Transform, (With<Eye>, With<UiAnchored>)>,
) {
    let (camera, camera_transform) = *camera;
    let (node, ui_transform) = *anchor;
    // UI transforms are in physical pixels; the camera wants logical ones.
    let centre = ui_transform.translation * node.inverse_scale_factor();
    let Ok(world) = camera.viewport_to_world_2d(camera_transform, centre) else {
        return;
    };
    for mut eye in &mut eyes {
        eye.translation = world.extend(0.0);
    }
}

/// Puts every free-standing eye at its share of the window.
fn place_screen_eyes(
    window: Single<&Window, With<PrimaryWindow>>,
    mut eyes: Query<(&mut Transform, &ScreenPlacement), With<Eye>>,
) {
    let half = window.size() / 2.0;
    for (mut eye, placement) in &mut eyes {
        eye.translation = (placement.0 * half).extend(0.0);
    }
}

/// Lets each eye's attention wander: now and then it picks a spot to stare at
/// instead of you, holds it for a moment, then goes back to watching. Moving
/// the pointer cuts a glance short — the movement catches its eye.
fn wander_gaze(
    time: Res<Time>,
    mut cursor_moved: MessageReader<CursorMoved>,
    mut irises: Query<(&Attention, &mut Gaze)>,
) {
    let dt = time.delta_secs();
    let mut rng = rand::rng();

    // How far the pointer travelled this frame. A move with no delta is the
    // cursor (re-)entering the window, which is about as eye-catching as it
    // gets, so it counts as infinitely much.
    let motion: f32 = cursor_moved
        .read()
        .map(|moved| moved.delta.map_or(f32::INFINITY, Vec2::length))
        .sum();

    for (attention, mut gaze) in &mut irises {
        let alerted = motion > 0.0 && motion >= attention.alert_motion;

        match *gaze {
            Gaze::Distracted {
                ref mut remaining, ..
            } => {
                *remaining -= dt;
                if *remaining <= 0.0 {
                    *gaze = Gaze::default();
                } else if alerted {
                    // Caught looking away: straight back to you, and held there
                    // for the focus period.
                    *gaze = Gaze::Watching {
                        focus: attention.focus_secs,
                    };
                }
            }
            Gaze::Watching { ref mut focus } => {
                // Every movement starts the focus period over, so an eye can
                // only wander once the pointer has been still that long.
                if alerted {
                    *focus = attention.focus_secs;
                }
                *focus -= dt;
                if *focus > 0.0 || attention.distraction_rate <= 0.0 {
                    continue;
                }
                // Poisson process: the chance of a glance starting in this
                // frame, so the rate means the same thing at any frame rate.
                let chance = 1.0 - (-attention.distraction_rate * dt).exp();
                if !rng.random_bool(chance as f64) {
                    continue;
                }

                let angle = rng.random_range(attention.glance_angle.clone());
                let reach = rng.random_range(attention.glance_reach.clone());
                *gaze = Gaze::Distracted {
                    offset: Vec2::from_angle(angle) * reach * IRIS_TRAVEL_PX,
                    remaining: rng.random_range(attention.glance_secs.clone()),
                };
            }
        }
    }
}

/// Slides each iris towards whatever it is looking at — the cursor, or the
/// spot [`wander_gaze`] picked — clamped so it stays inside its own eyeball.
/// The iris chases that target rather than snapping to it, so the gaze lags a
/// little behind and eases back after a glance.
fn look_at_cursor(
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    eyes: Query<&Transform, (With<Eye>, Without<Iris>)>,
    mut irises: Query<(&mut Transform, &Iris, &Gaze, &ChildOf)>,
) {
    let (camera, camera_transform) = *camera;
    // No pointer (a kiosk with no mouse, say) just means nothing to track;
    // distracted eyes still have somewhere to be.
    let cursor = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world_2d(camera_transform, pos).ok());

    let dt = time.delta_secs();
    for (mut transform, iris, gaze, child_of) in &mut irises {
        let Ok(eye) = eyes.get(child_of.parent()) else {
            continue;
        };

        // Offsets from the eyeball's centre, in sprite px: the iris is a child
        // of the scaled eye, so its translation is in that space too.
        let offset = match *gaze {
            Gaze::Distracted { offset, .. } => offset,
            Gaze::Watching { .. } => match cursor {
                Some(cursor) => {
                    let scale = eye.scale.x;
                    let globe_centre = eye.translation.truncate() + GLOBE_CENTRE_PX * scale;
                    let travel = IRIS_TRAVEL_PX * scale;
                    let normalised = ((cursor - globe_centre) / travel).clamp_length_max(1.0);
                    // Ease so it moves eagerly for nearby targets and saturates
                    // for far ones.
                    let eased = normalised * normalised.length().sqrt();
                    eased * IRIS_TRAVEL_PX
                }
                None => Vec2::ZERO,
            },
        };

        // Exponential smoothing, framed so the result is the same whatever the
        // frame rate: each second the iris closes `response` e-foldings of the
        // remaining distance.
        let blend = 1.0 - (-iris.response * dt).exp();
        let current = transform.translation.truncate();
        transform.translation = current.lerp(GLOBE_CENTRE_PX + offset, blend).extend(1.0);
    }
}
