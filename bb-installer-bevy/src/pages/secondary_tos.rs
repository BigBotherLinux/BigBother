//! The secondary terms of service. The text is written in word by word, faster
//! and faster, until the scrollbar thumb is too small to bother with. Then the
//! scrollbar blasts off and twinkles out, like Team Rocket.

use std::f32::consts::{PI, TAU};

use crate::AppState;
use bevy::asset::RenderAssetUsages;
use bevy::feathers::controls::{FeathersButton, FeathersScrollbar};
use bevy::feathers::display::label;
use bevy::feathers::theme::{ThemeBackgroundColor, ThemedText};
use bevy::feathers::tokens;
use bevy::image::ToExtents;
use bevy::input::common_conditions::input_just_pressed;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use bevy::ui::{InteractionDisabled, UiTransform};
use bevy::ui_widgets::{Activate, ControlOrientation, ScrollArea, Scrollbar};
use bevy::window::PrimaryWindow;

/// Words per second when the page opens.
const START_RATE: f32 = 15.0;
// The rate doubles this often.
const DOUBLING_SECS: f32 = 0.7;
const MAX_RATE: f32 = 100_000.0;
/// Keep writing this long after the scrollbar has left.
const WRITE_AFTER_BLAST_SECS: f32 = 1.5;
/// Safety cap in case the scrollbar never leaves.
const MAX_WORDS: usize = 400_000;

/// Thumb length, in logical pixels, at which the scrollbar gives up.
const BLAST_OFF_AT: f32 = 4.0;
const FLIGHT_SECS: f32 = 1.1;
const FLIGHT_SPINS: f32 = 4.0;
const TWINKLE_SECS: f32 = 0.6;
const SPARKLE_SIZE: f32 = 48.0;
const SPARKLE_TEXELS: u32 = 64;

/// Plain legal text first, so the first screen looks legitimate.
const INTRO: [&str; 4] = [
    "These Supplementary Terms apply in addition to the Terms of Service you have already accepted. Where the two disagree, the one you have read less of prevails.",
    "By continuing to use the installer, including by scrolling, reading, looking away or doing nothing, you confirm that you have read and understood these Supplementary Terms in full.",
    "BigBother may amend these Supplementary Terms at any time. Amendments take effect when they are written, which may be while you are reading them.",
    "The following sections set out the remaining obligations of the user in detail. The user is advised to read them carefully and at a reasonable pace.",
];

/// Repeated until we run out of patience.
const LOREM: [&str; 4] = [
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
    "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet.",
];

fn paragraph(index: usize) -> &'static str {
    INTRO
        .get(index)
        .copied()
        .unwrap_or_else(|| LOREM[(index - INTRO.len()) % LOREM.len()])
}

pub struct SecondaryTosPlugin;

impl Plugin for SecondaryTosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::SecondaryTos),
            (secondary_tos_root.spawn(), start_feed),
        )
        // A dev-only shortcut to the blast-off (F8), so it can be tuned
        // without waiting for the text.
        .add_systems(
            Update,
            force_blast
                .run_if(|| cfg!(debug_assertions))
                .run_if(input_just_pressed(KeyCode::F8))
                .run_if(in_state(AppState::SecondaryTos)),
        )
        .add_systems(OnExit(AppState::SecondaryTos), stop_feed)
        .add_systems(
            Update,
            (
                feed_text,
                let_thumb_shrink,
                blast_off_when_tiny,
                fly,
                twinkle,
            )
                .run_if(in_state(AppState::SecondaryTos)),
        );
    }
}

/// The scrolling container the paragraphs are added to.
#[derive(Component, FromTemplate)]
struct TosText;

#[derive(Component, FromTemplate)]
struct TosScrollbar;

/// Locked until the scrollbar has left.
#[derive(Component, FromTemplate)]
struct TosContinue;

/// Where the writer is in the text.
#[derive(Resource)]
struct Feed {
    /// Paragraph being written, once it exists.
    current: Option<Entity>,
    paragraph: usize,
    words: Vec<&'static str>,
    word: usize,
    written: usize,
    elapsed: f32,
    /// Fractional words carried over between frames.
    owed: f32,
    /// When the scrollbar left, in `elapsed` time.
    blasted_at: Option<f32>,
    /// Blast off on the next frame, however big the thumb is.
    force_blast: bool,
}

impl Feed {
    fn done(&self) -> bool {
        self.written >= MAX_WORDS
            || self
                .blasted_at
                .is_some_and(|at| self.elapsed - at > WRITE_AFTER_BLAST_SECS)
    }
}

#[derive(Resource)]
struct SparkleImage(Handle<Image>);

/// The scrollbar on its way out.
#[derive(Component)]
struct Flight {
    from: Vec2,
    to: Vec2,
    size: Vec2,
    elapsed: f32,
}

#[derive(Component)]
struct Twinkle {
    elapsed: f32,
}

fn start_feed(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.insert_resource(Feed {
        current: None,
        paragraph: 0,
        words: paragraph(0).split_whitespace().collect(),
        word: 0,
        written: 0,
        elapsed: 0.0,
        owed: 0.0,
        blasted_at: None,
        force_blast: false,
    });
    commands.insert_resource(SparkleImage(images.add(sparkle())));
}

fn stop_feed(mut commands: Commands) {
    commands.remove_resource::<Feed>();
    commands.remove_resource::<SparkleImage>();
}

fn secondary_tos_root() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
        }
        DespawnOnExit::<AppState>(AppState::SecondaryTos)
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[
            (Text::new("Supplementary Terms of Service") ThemedText),
            (
                Node {
                    width: Val::Px(600.0),
                    height: Val::Vh(60.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect { right: Val::Px(14.0) },
                }
                Children[
                    (
                        #tos_text
                        Node {
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            overflow: Overflow::scroll_y(),
                        }
                        ScrollArea
                        TosText
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
                    TosScrollbar
                ]
            ),
            (
                @FeathersButton { @caption: bsn! { Text("Continue") ThemedText} }
                Node { width: Val::Px(180.0) }
                on(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| {
                    state.set(AppState::CursorCrowd);
                })
                InteractionDisabled
                TosContinue
            )
        ]
    }
}

fn force_blast(feed: Option<ResMut<Feed>>) {
    if let Some(mut feed) = feed {
        feed.force_blast = true;
    }
}

/// Writes this frame's share of words. Words for the current paragraph are
/// appended to its text; a paragraph that starts this frame is spawned with
/// everything it got so far.
fn feed_text(
    mut commands: Commands,
    time: Res<Time>,
    feed: Option<ResMut<Feed>>,
    area: Single<Entity, With<TosText>>,
    mut texts: Query<&mut Text>,
) {
    let Some(mut feed) = feed else {
        return;
    };
    let dt = time.delta_secs();
    feed.elapsed += dt;
    let rate = (START_RATE * (feed.elapsed / DOUBLING_SECS).exp2()).min(MAX_RATE);
    feed.owed += rate * dt;
    let due = feed.owed.floor();
    feed.owed -= due;

    // Text for the paragraph already on screen, then any new paragraphs.
    let mut append = String::new();
    let mut new_paragraphs: Vec<String> = Vec::new();

    for _ in 0..due as usize {
        if feed.done() {
            break;
        }
        if feed.word >= feed.words.len() {
            feed.paragraph += 1;
            feed.words = paragraph(feed.paragraph).split_whitespace().collect();
            feed.word = 0;
        }
        let word = feed.words[feed.word];
        if feed.word == 0 {
            new_paragraphs.push(format!("§ {}. {word}", feed.paragraph + 1));
        } else {
            let target = new_paragraphs.last_mut().unwrap_or(&mut append);
            target.push(' ');
            target.push_str(word);
        }
        feed.word += 1;
        feed.written += 1;
    }

    if !append.is_empty()
        && let Some(mut text) = feed.current.and_then(|e| texts.get_mut(e).ok())
    {
        text.0.push_str(&append);
    }
    for text in new_paragraphs {
        let id = commands
            .spawn_scene(label(text))
            .insert(ChildOf(*area))
            .id();
        feed.current = Some(id);
    }
}

/// Feathers stops the thumb at 8px. Ours is allowed to keep going.
fn let_thumb_shrink(mut scrollbars: Query<&mut Scrollbar, With<TosScrollbar>>) {
    for mut scrollbar in &mut scrollbars {
        if scrollbar.min_thumb_length != 0.0 {
            scrollbar.min_thumb_length = 0.0;
        }
    }
}

type TosScrollbarTrack<'w, 's> =
    Single<'w, 's, (Entity, &'static ComputedNode, &'static UiGlobalTransform), With<TosScrollbar>>;

/// Once the thumb is tiny, swap the scrollbar for a copy that flies away. The
/// text stops scrolling and Continue unlocks.
fn blast_off_when_tiny(
    mut commands: Commands,
    scrollbar: Option<TosScrollbarTrack>,
    area: Single<(Entity, &ComputedNode), With<TosText>>,
    continues: Query<Entity, With<TosContinue>>,
    window: Single<&Window, With<PrimaryWindow>>,
    feed: Option<ResMut<Feed>>,
    mut last_log: Local<f32>,
) {
    let (Some(scrollbar), Some(mut feed)) = (scrollbar, feed) else {
        return;
    };
    let (entity, track, transform) = *scrollbar;
    let (area, area_node) = *area;
    let track_scale = track.inverse_scale_factor();
    let track_size = track.size() * track_scale;

    if !feed.force_blast {
        let visible = area_node.size().y - area_node.scrollbar_size.y;
        let content = area_node.content_size().y;
        if content <= visible || visible <= 0.0 {
            return;
        }
        let thumb = track_size.y * visible / content;
        if feed.elapsed - *last_log >= 1.0 {
            *last_log = feed.elapsed;
            info!("{} words, thumb {thumb:.1}px", feed.written);
        }
        if thumb > BLAST_OFF_AT {
            return;
        }
    }
    info!("Blast off at {} words", feed.written);
    feed.blasted_at = Some(feed.elapsed);

    // Without the scrollbar there is no scrolling.
    commands.entity(area).remove::<ScrollArea>();
    for button in &continues {
        commands.entity(button).remove::<InteractionDisabled>();
    }

    let from = transform.translation * track_scale;
    let to = Vec2::new(
        (from.x + window.width() * 0.2).min(window.width() * 0.92),
        window.height() * 0.08,
    );
    commands.entity(entity).despawn();
    commands.spawn((
        Flight {
            from,
            to,
            size: track_size,
            elapsed: 0.0,
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(track_size.x),
            height: Val::Px(track_size.y),
            border_radius: BorderRadius::all(Val::Px(3.0)),
            ..default()
        },
        ThemeBackgroundColor(tokens::SCROLLBAR_THUMB),
        UiTransform::default(),
        GlobalZIndex(i32::MAX - 1),
        Pickable::IGNORE,
        DespawnOnExit(AppState::SecondaryTos),
    ));
}

fn fly(
    mut commands: Commands,
    time: Res<Time>,
    sparkle: Option<Res<SparkleImage>>,
    mut flights: Query<(Entity, &mut Flight, &mut Node, &mut UiTransform)>,
) {
    for (entity, mut flight, mut node, mut transform) in &mut flights {
        flight.elapsed += time.delta_secs();
        let t = (flight.elapsed / FLIGHT_SECS).min(1.0);
        // Fast off the launch, slow as it shrinks into the distance.
        let eased = 1.0 - (1.0 - t).powi(3);
        let at = flight.from.lerp(flight.to, eased);

        node.left = Val::Px(at.x - flight.size.x / 2.0);
        node.top = Val::Px(at.y - flight.size.y / 2.0);
        transform.scale = Vec2::splat((1.0 - eased).max(0.01));
        transform.rotation = Rot2::radians(eased * FLIGHT_SPINS * TAU);

        if t < 1.0 {
            continue;
        }
        commands.entity(entity).despawn();
        if let Some(sparkle) = &sparkle {
            commands.spawn((
                Twinkle { elapsed: 0.0 },
                ImageNode::new(sparkle.0.clone()),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(flight.to.x - SPARKLE_SIZE / 2.0),
                    top: Val::Px(flight.to.y - SPARKLE_SIZE / 2.0),
                    width: Val::Px(SPARKLE_SIZE),
                    height: Val::Px(SPARKLE_SIZE),
                    ..default()
                },
                UiTransform::from_scale(Vec2::ZERO),
                GlobalZIndex(i32::MAX - 1),
                Pickable::IGNORE,
                DespawnOnExit(AppState::SecondaryTos),
            ));
        }
    }
}

fn twinkle(
    mut commands: Commands,
    time: Res<Time>,
    mut twinkles: Query<(Entity, &mut Twinkle, &mut UiTransform)>,
) {
    for (entity, mut twinkle, mut transform) in &mut twinkles {
        twinkle.elapsed += time.delta_secs();
        let t = twinkle.elapsed / TWINKLE_SECS;
        if t >= 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        transform.scale = Vec2::splat((t * PI).sin());
        transform.rotation = Rot2::radians(t * PI / 4.0);
    }
}

/// A white four-pointed star with a soft core, drawn into a texture.
fn sparkle() -> Image {
    let mut data = Vec::with_capacity((SPARKLE_TEXELS * SPARKLE_TEXELS * 4) as usize);
    for y in 0..SPARKLE_TEXELS {
        for x in 0..SPARKLE_TEXELS {
            let p = (Vec2::new(x as f32, y as f32) + 0.5) / SPARKLE_TEXELS as f32 * 2.0 - 1.0;
            // An astroid with a low exponent has long thin points.
            let star = 1.0 - (p.x.abs().sqrt() + p.y.abs().sqrt());
            let core = (-p.length_squared() * 30.0).exp();
            let alpha = (star * 4.0).clamp(0.0, 1.0).max(core);
            data.extend_from_slice(&[255, 255, 255, (alpha * 255.0) as u8]);
        }
    }
    Image::new(
        UVec2::splat(SPARKLE_TEXELS).to_extents(),
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}
