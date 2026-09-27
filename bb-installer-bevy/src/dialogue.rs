//! A tiny data-driven dialogue *engine*.
//!
//! The interrogation is a graph of steps, not a graph of `States`. One state per
//! question would mean one `OnEnter` system per question, and every new question
//! would be a new variant, a new system and a new registration. Here a step is
//! plain data, and adding a branch is adding a line to a table.
//!
//! The only runtime state is [`Cursor`] — which step we are on. One system
//! watches it, and whenever it changes it despawns the old screen and spawns the
//! new one. So the whole engine is: data in, `Cursor` moves, UI follows.
//!
//! # What this file deliberately does not know
//!
//! Nothing here mentions a button, a slider, a colour or a font. A [`Step`] is
//! just a closure handed a screen entity, and however it fills that screen, it
//! ends by triggering [`Respond`]. That one event is the entire extension point.
//!
//! The authoring vocabulary that *does* know about widgets — `ask`, `select`,
//! `rate` — lives in [`crate::script`], one layer up. The dependency arrow runs
//! content to script to engine, and script to toolkit. It never comes back.

use std::sync::Arc;

use bevy::platform::collections::HashMap;
use bevy::prelude::*;

/// Steps are addressed by name so the script reads like prose.
pub type StepId = &'static str;

/// Where an answer takes us.
#[derive(Clone, Copy, Debug)]
pub enum Next {
    /// Jump to another step.
    To(StepId),
    /// End the dialogue; fires [`DialogueFinished`].
    Done,
}

/// How a step fills in its screen.
///
/// `Arc` because the script is a resource read once per transition, not
/// consumed. The closure is handed the screen entity and must eventually
/// trigger [`Respond`] on it.
pub type Spawn = Arc<dyn Fn(&mut Commands, Entity) + Send + Sync>;

/// One screen of the dialogue.
pub struct Step {
    spawn: Spawn,
}

/// Build a step from the closure that fills its screen.
pub fn step(spawn: impl Fn(&mut Commands, Entity) + Send + Sync + 'static) -> Step {
    Step {
        spawn: Arc::new(spawn),
    }
}

/// The script itself — immutable content, inserted once at startup.
#[derive(Resource)]
pub struct Dialogue {
    start: StepId,
    steps: HashMap<StepId, Step>,
}

impl Dialogue {
    pub fn new(start: StepId, steps: impl IntoIterator<Item = (StepId, Step)>) -> Self {
        let steps: HashMap<_, _> = steps.into_iter().collect();
        debug_assert!(steps.contains_key(start), "start step {start} is missing");
        Self { start, steps }
    }
}

/// Where we are right now. This is the entire mutable state of a dialogue.
#[derive(Resource, Debug)]
pub struct Cursor {
    pub at: StepId,
}

/// Everything the victim has admitted to, in order — untyped, for logging and
/// for reading their own words back at them. Anything you want to *act* on
/// belongs in a resource of your own, written through `Choice::record`.
///
/// `String` rather than `&'static str` because not every answer is a button
/// label; a rating is a number the victim picked.
#[derive(Resource, Default, Debug)]
pub struct Transcript(pub Vec<(StepId, String)>);

/// An answer, from any kind of widget. Trigger it on the screen entity and the
/// engine records it and moves the cursor.
#[derive(EntityEvent)]
pub struct Respond {
    /// The dialogue screen being answered.
    #[event_target]
    pub screen: Entity,
    pub answer: String,
    pub next: Next,
}

/// Fired when an answer leads to [`Next::Done`].
#[derive(Message)]
pub struct DialogueFinished;

/// Marks the screen currently on display, so we can wipe it on a transition.
#[derive(Component)]
struct DialogueRoot;

/// Runs a [`Dialogue`] for as long as the app is in state `S`.
pub struct DialoguePlugin<S: States>(pub S);

impl<S: States> Plugin for DialoguePlugin<S> {
    fn build(&self, app: &mut App) {
        let state = self.0.clone();
        app.add_message::<DialogueFinished>()
            .init_resource::<Transcript>()
            .add_systems(OnEnter(state.clone()), begin)
            .add_systems(OnExit(state.clone()), end)
            .add_systems(
                Update,
                show_current
                    .run_if(in_state(state))
                    .run_if(resource_exists_and_changed::<Cursor>),
            );
    }
}

fn begin(mut commands: Commands, dialogue: Res<Dialogue>) {
    commands.insert_resource(Cursor { at: dialogue.start });
}

fn end(mut commands: Commands, screens: Query<Entity, With<DialogueRoot>>) {
    commands.remove_resource::<Cursor>();
    for screen in &screens {
        commands.entity(screen).despawn();
    }
}

/// The one system that turns the current step into entities. Note that it does
/// not care *which* step: add fifty questions and this stays the same size.
fn show_current(
    mut commands: Commands,
    dialogue: Res<Dialogue>,
    cursor: Res<Cursor>,
    old: Query<Entity, With<DialogueRoot>>,
) {
    for screen in &old {
        commands.entity(screen).despawn();
    }

    let Some(step) = dialogue.steps.get(cursor.at) else {
        error!("no dialogue step named {}", cursor.at);
        return;
    };
    let here = cursor.at;

    // Layout only. A screen fills the window and stacks its contents; what
    // those contents look like is the script's business, not the engine's.
    let screen = commands
        .spawn((
            DialogueRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
        ))
        // Every answer, whatever spawned it, arrives here.
        .observe(
            move |respond: On<Respond>,
                  mut cursor: ResMut<Cursor>,
                  mut transcript: ResMut<Transcript>,
                  mut finished: MessageWriter<DialogueFinished>| {
                transcript.0.push((here, respond.answer.clone()));
                match respond.next {
                    Next::To(target) => cursor.at = target,
                    Next::Done => {
                        finished.write(DialogueFinished);
                    }
                }
            },
        )
        .id();

    (step.spawn)(&mut commands, screen);
}
