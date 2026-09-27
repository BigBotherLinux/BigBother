//! The consent interrogation. This file is *content*: the whole branching
//! script is one table, and nothing here knows how buttons are spawned.

use bb_installer_bevy::dialogue::Dialogue;
use bb_installer_bevy::AppState;
use bevy::prelude::*;

use bb_installer_bevy::dialogue::{DialogueFinished, DialoguePlugin, Next::*};
use bb_installer_bevy::script::{ask, choice, dialogue, rate, say, select, Rating};

/// What we actually learned, in our own types. The dialogue writes into this
/// via `.record(...)`; the rest of the installer reads it and never has to
/// parse button labels.
#[derive(Resource, Default, Debug)]
pub struct Answers {
    pub willing: bool,
    pub believes_in_free_will: bool,
    pub pledge: Option<Pledge>,
    pub experience: Option<i32>,
    pub recommend: Option<i32>,
    pub trust: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Pledge {
    Data,
    Firstborn,
    Dignity,
}

pub struct TosPlugin;

impl Plugin for TosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Answers>()
            .insert_resource(script())
            .add_plugins(DialoguePlugin(AppState::TermsOfService))
            .add_systems(Update, leave_when_finished);
    }
}

fn script() -> Dialogue {
    dialogue(
        "consent",
        [
            (
                "consent",
                ask(
                    "Are you here of your own free will?",
                    [
                        choice("Yes, I consent", To("believe_in_free_will"))
                            .record(|a: &mut Answers| a.willing = true),
                        choice("No, I was coerced", To("no_consent"))
                            .record(|a: &mut Answers| a.willing = false),
                    ],
                ).aside("We need your consent to proceed."),
            ),
            (
                "believe_in_free_will",
                ask(
                    "Do you believe in free will?",
                    [
                        choice("Yes, obviously", To("non_determinism"))
                            .record(|a: &mut Answers| a.believes_in_free_will = true),
                        choice("No", To("determinism_1"))
                            .record(|a: &mut Answers| a.believes_in_free_will = false),
                    ],
                ),
            ),
            (
                "no_consent",
                ask(
                    "We cannot legally proceed without your consent. Please shut down your computer.",
                    [
                        choice("Sorry, I changed my mind", To("consent")),
                        choice("Exit", Done),
                    ],
                ),
            ),
            (
                "determinism_1",
                ask(
                    "Then how exactly could you consent to something you did not choose?",
                    [choice(
                        "I was destined to click this button too!",
                        To("determinism_2"),
                    )],
                )
                .aside("There is no need to deliberate."),
            ),
            (
                "determinism_2",
                ask(
                    "Correct. And how could consent work in a legal system anyway, if you did not choose to consent?",
                    [choice("Exactly, it wouldn't", To("determinism_3"))],
                )
                .aside("You may click the button. It is the only button."),
            ),
            (
                "determinism_3",
                ask(
                    "Your consent has therefore been backdated to the beginning of the universe.",
                    [choice("I love BigBother", To("pledge"))],
                )
                .aside("Thank you. That was very freely given."),
            ),
            (
                "non_determinism",
                ask(
                    "Good. Meaning you chose to consent to this.",
                    [
                        choice("I consent", To("pledge")),
                        choice("I do not consent, exit", Done),
                    ],
                )
                .aside("Any legal system implies determinism for consent to work as a concept."),
            ),
            (
                "pledge",
                select(
                    "Select what you wish to pledge.",
                    [
                        choice("My browsing data", To("noted"))
                            .record(|a: &mut Answers| a.pledge = Some(Pledge::Data)),
                        choice("My firstborn", To("noted"))
                            .record(|a: &mut Answers| a.pledge = Some(Pledge::Firstborn)),
                        choice("My remaining dignity", To("noted"))
                            .record(|a: &mut Answers| a.pledge = Some(Pledge::Dignity)),
                    ],
                )
                .aside("Exactly one. Pledging nothing is not an option."),
            ),
            (
                "noted",
                say("Your answers have been noted and forwarded.", To("experience"))
                    .aside("Forwarded where is on a need-to-know basis. You do not need to know."),
            ),
            (
                "experience",
                rate(
                    "How is your experience so far?",
                    Rating::new(1, 10)
                        .start(5)
                        .at_least(8, "That cannot be right. Move it up and try again.")
                        .store(|a: &mut Answers, score| a.experience = Some(score)),
                    To("recommend"),
                )
                .aside("Be honest. There is a correct answer."),
            ),
            (
                "recommend",
                rate(
                    "How likely are you to recommend BigBother to friends or family?",
                    Rating::new(1, 10)
                        .start(10)
                        .at_least(10, "Anything under 10 counts as a complaint about you.")
                        .store(|a: &mut Answers, score| a.recommend = Some(score)),
                    To("trust"),
                )
                .aside("Family includes people you have not met yet."),
            ),
            (
                "trust",
                rate(
                    "How much do you trust this installer?",
                    Rating::new(1, 10)
                        .start(5)
                        .at_most(3, "Overconfidence noted. Lower it.")
                        .store(|a: &mut Answers, score| a.trust = Some(score)),
                    Done,
                )
                .aside("This is the only question where a low score is the honest one."),
            )
        ],
    )
}

fn leave_when_finished(
    mut finished: MessageReader<DialogueFinished>,
    answers: Res<Answers>,
    mut state: ResMut<NextState<AppState>>,
) {
    if !finished.is_empty() {
        finished.clear();
        info!("consent on file: {answers:?}");
        state.set(AppState::Welcome);
    }
}
