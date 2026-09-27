//! The authoring vocabulary: `ask`, `select`, `rate`, `say`.
//!
//! This is the only layer that knows both halves. It knows what a question is,
//! from [`crate::dialogue`], and it knows what a button looks like, from
//! [`crate::ui`]. Keeping it in its own file is what lets the engine stay free of
//! widgets and the toolkit stay free of the installer.
//!
//! If you want a new kind of question, add a constructor here. You should not
//! have to touch the engine, and you should only touch the toolkit if the
//! question needs a control that does not exist yet.

use std::sync::Arc;

use bevy::prelude::*;
use bevy::ui::Checked;
use bevy::ui_widgets::{Activate, SliderValue};

use crate::dialogue::{self, Dialogue, Next, Respond, Spawn, StepId};
use crate::ui::button::{BbButton, BbRadioButton, BbRadioGroup, ButtonVariant};
use crate::ui::slider::BbSlider;
use crate::ui::spawn_child;
use crate::ui::text::{label, line, styles};

/// How an answer writes itself into one of *your* resources. `Arc` only so the
/// button observer can own a copy; you never build one by hand.
type Effect = Arc<dyn Fn(&mut World) + Send + Sync>;

/// One answer: a label, where it leads, and optionally what it records.
#[derive(Clone)]
pub struct Choice {
    pub label: &'static str,
    pub next: Next,
    effect: Option<Effect>,
}

impl Choice {
    /// Store this answer in your own type:
    ///
    /// ```ignore
    /// choice("Yes", To("question_1")).record(|a: &mut Answers| a.willing = true)
    /// ```
    ///
    /// The engine never learns what `Answers` is; it just runs the closure.
    pub fn record<R: Resource<Mutability = bevy::ecs::component::Mutable>>(
        mut self,
        write: impl Fn(&mut R) + Send + Sync + 'static,
    ) -> Self {
        self.effect = Some(Arc::new(move |world: &mut World| {
            write(&mut world.resource_mut::<R>());
        }));
        self
    }
}

/// `choice("Yes", To("question_1"))`
pub fn choice(label: &'static str, next: Next) -> Choice {
    Choice {
        label,
        next,
        effect: None,
    }
}

/// How a step is answered.
enum Body {
    /// One button per answer; clicking commits immediately.
    Buttons(Vec<Choice>),
    /// Pick one, then press Confirm.
    Select(Vec<Choice>),
    /// Widgets you spawn yourself.
    Custom(Spawn),
}

/// A single screen being authored: something said, and the ways out of it.
pub struct Step {
    prompt: &'static str,
    /// Small print under the prompt. Good for snide remarks.
    aside: Option<&'static str>,
    body: Body,
}

impl Step {
    /// Builder-style small print: `say("...").aside("we are watching")`.
    pub fn aside(mut self, aside: &'static str) -> Self {
        self.aside = Some(aside);
        self
    }

    /// Turn an authored step into the closure the engine runs.
    fn into_engine(self) -> dialogue::Step {
        let Self {
            prompt,
            aside,
            body,
        } = self;

        dialogue::step(move |commands, screen| {
            // The prompt, then the small print if there is any. Spawned the same
            // way as the answers below, one command each: `queue_spawn_related_scenes`
            // resolves asynchronously, which let the answers land first and put
            // the question underneath them.
            spawn_child(commands, screen, line(prompt, styles::PROMPT));
            if let Some(aside) = aside {
                spawn_child(commands, screen, line(aside, styles::ASIDE));
            }

            match &body {
                Body::Buttons(choices) => {
                    for c in choices {
                        spawn_answer_button(commands, screen, c.clone());
                    }
                }
                Body::Select(choices) => spawn_select(commands, screen, choices),
                Body::Custom(spawn) => spawn(commands, screen),
            }
        })
    }
}

/// A question with branching answers, one button each.
pub fn ask(prompt: &'static str, choices: impl IntoIterator<Item = Choice>) -> Step {
    authored(prompt, Body::Buttons(choices.into_iter().collect()))
}

/// A question answered by picking one option and confirming.
pub fn select(prompt: &'static str, choices: impl IntoIterator<Item = Choice>) -> Step {
    authored(prompt, Body::Select(choices.into_iter().collect()))
}

/// A remark: no branching, one way onwards. Structurally just a step with a
/// single choice, which is why there is no separate "dialog" concept.
pub fn say(prompt: &'static str, next: Next) -> Step {
    authored(prompt, Body::Buttons(vec![choice("Continue", next)]))
}

/// A question answered by widgets you spawn yourself. The closure gets
/// `Commands` and the screen entity, and must eventually trigger [`Respond`].
pub fn custom(
    prompt: &'static str,
    spawn: impl Fn(&mut Commands, Entity) + Send + Sync + 'static,
) -> Step {
    authored(prompt, Body::Custom(Arc::new(spawn)))
}

fn authored(prompt: &'static str, body: Body) -> Step {
    Step {
        prompt,
        aside: None,
        body,
    }
}

/// Assemble a script. Same shape as [`Dialogue::new`], but taking authored
/// steps, so content files never see the engine's types.
pub fn dialogue(start: StepId, steps: impl IntoIterator<Item = (StepId, Step)>) -> Dialogue {
    Dialogue::new(
        start,
        steps
            .into_iter()
            .map(|(id, step)| (id, step.into_engine()))
            .collect::<Vec<_>>(),
    )
}

/// A rating question: a track, a number, and a Submit we may decline to honour.
///
/// The bounds are the whole joke. A rating outside what we are willing to hear
/// is not an error — it simply does not submit, and the screen explains why in a
/// tone that implies the fault is yours.
///
/// ```ignore
/// rate(
///     "How is your experience so far?",
///     Rating::new(1, 10).start(5).at_least(8, "That cannot be right."),
///     To("recommend"),
/// )
/// ```
pub struct Rating {
    min: i32,
    max: i32,
    start: i32,
    /// Lowest score we will accept, and what we say when you go under it.
    floor: Option<(i32, &'static str)>,
    /// Highest score we will accept, and what we say when you go over it.
    ceiling: Option<(i32, &'static str)>,
    store: Option<Score>,
}

/// [`Effect`]'s numeric cousin: how an accepted score writes itself into one of
/// your resources.
type Score = Arc<dyn Fn(&mut World, i32) + Send + Sync>;

impl Rating {
    pub fn new(min: i32, max: i32) -> Self {
        debug_assert!(min < max, "rating range {min}..={max} is empty");
        Self {
            min,
            max,
            start: (min + max) / 2,
            floor: None,
            ceiling: None,
            store: None,
        }
    }

    /// Where the fill sits before they touch it.
    pub fn start(mut self, start: i32) -> Self {
        self.start = start.clamp(self.min, self.max);
        self
    }

    /// Refuse anything below `floor`, complaining as instructed.
    pub fn at_least(mut self, floor: i32, complaint: &'static str) -> Self {
        self.floor = Some((floor, complaint));
        self
    }

    /// Refuse anything above `ceiling`, complaining as instructed.
    pub fn at_most(mut self, ceiling: i32, complaint: &'static str) -> Self {
        self.ceiling = Some((ceiling, complaint));
        self
    }

    /// Store the accepted score in one of your own resources. Same idea as
    /// [`Choice::record`], except the closure is handed the number.
    pub fn store<R: Resource<Mutability = bevy::ecs::component::Mutable>>(
        mut self,
        write: impl Fn(&mut R, i32) + Send + Sync + 'static,
    ) -> Self {
        self.store = Some(Arc::new(move |world: &mut World, score: i32| {
            write(&mut world.resource_mut::<R>(), score);
        }));
        self
    }

    /// The complaint this score earns, if we are not accepting it.
    fn refusal(&self, score: i32) -> Option<&'static str> {
        match (self.floor, self.ceiling) {
            (Some((floor, why)), _) if score < floor => Some(why),
            (_, Some((ceiling, why))) if score > ceiling => Some(why),
            _ => None,
        }
    }
}

/// A question answered on a [`Rating`] track.
pub fn rate(prompt: &'static str, rating: Rating, next: Next) -> Step {
    let rating = Arc::new(rating);

    custom(prompt, move |commands, screen| {
        // The slider owns its own fill and readout, so the only handle we need
        // is the entity itself, to read `SliderValue` back on submit.
        let track = spawn_child(
            commands,
            screen,
            bsn! {
                @BbSlider {
                    @min: {rating.min},
                    @max: {rating.max},
                    @start: {rating.start}
                }
            },
        )
        .id();

        // Empty until they submit something we dislike.
        let complaint = spawn_child(commands, screen, line("", styles::ERROR)).id();

        let rating = rating.clone();
        spawn_child(
            commands,
            screen,
            bsn! {
                @BbButton {
                    @caption: {label("Submit")},
                    @variant: ButtonVariant::Primary
                }
            },
        )
        .observe(
            move |_: On<Activate>,
                  tracks: Query<&SliderValue>,
                  mut texts: Query<&mut Text>,
                  mut commands: Commands| {
                let Ok(value) = tracks.get(track) else {
                    return;
                };
                let score = value.0.round() as i32;

                if let Some(why) = rating.refusal(score) {
                    if let Ok(mut text) = texts.get_mut(complaint) {
                        **text = why.to_string();
                    }
                    return;
                }

                if let Some(write) = rating.store.clone() {
                    commands.queue(move |world: &mut World| write(world, score));
                }
                commands.trigger(Respond {
                    screen,
                    answer: score.to_string(),
                    next,
                });
            },
        );
    })
}

/// A button that commits its answer when clicked. The closure captures the
/// answer, so the button *is* the edge in the graph.
fn spawn_answer_button(commands: &mut Commands, screen: Entity, c: Choice) {
    spawn_child(
        commands,
        screen,
        bsn! {
            @BbButton {
                @caption: {label(c.label)}
            }
        },
    )
    .observe(move |_: On<Activate>, mut commands: Commands| {
        commit(&mut commands, screen, &c);
    });
}

/// Option buttons that only latch, plus a Confirm that commits the latched one.
///
/// The latching is not ours: [`BbRadioGroup`] wraps the headless `RadioGroup`,
/// which maintains [`Checked`] and handles arrow keys. All we add is the mapping
/// from a checked button back to the [`Choice`] it stands for.
fn spawn_select(commands: &mut Commands, screen: Entity, choices: &[Choice]) {
    let group = spawn_child(commands, screen, bsn! { @BbRadioGroup }).id();

    for c in choices {
        spawn_child(
            commands,
            group,
            bsn! {
                @BbRadioButton {
                    @caption: {label(c.label)}
                }
            },
        )
        .insert(Pick(c.clone()));
    }

    spawn_child(
        commands,
        screen,
        bsn! {
            @BbButton {
                @caption: {label("Confirm")},
                @variant: ButtonVariant::Primary
            }
        },
    )
    .observe(
        move |_: On<Activate>,
              picked: Query<(&Pick, &ChildOf), With<Checked>>,
              mut commands: Commands| {
            for (pick, parent) in &picked {
                if parent.parent() == group {
                    commit(&mut commands, screen, &pick.0);
                }
            }
        },
    );
}

/// The answer an option button stands for, parked on the entity so Confirm can
/// read it back without juggling indices.
#[derive(Component)]
struct Pick(Choice);

fn commit(commands: &mut Commands, screen: Entity, c: &Choice) {
    if let Some(write) = c.effect.clone() {
        commands.queue(move |world: &mut World| write(world));
    }
    commands.trigger(Respond {
        screen,
        answer: c.label.to_string(),
        next: c.next,
    });
}
