//! Drives a small script through the engine and the glue, to prove the three
//! layers still talk to each other: the glue spawns real widgets, an `Activate`
//! on one of them moves the cursor, and `record` writes into the app's own type.

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::scene::ScenePlugin;
use bevy::state::app::StatesPlugin;
use bevy::ui_widgets::{Activate, SliderValue};

use bb_installer_bevy::dialogue::{
    Cursor, Dialogue, DialogueFinished, DialoguePlugin, Next::*, Transcript,
};
use bb_installer_bevy::script::{ask, choice, dialogue, rate, Rating};
use bb_installer_bevy::ui::button::BbButton;
use bb_installer_bevy::ui::slider::BbSlider;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Phase {
    #[default]
    Asking,
}

/// What the script writes into. The engine never sees this type.
#[derive(Resource, Default, Debug, PartialEq)]
struct Answers {
    agreed: bool,
    score: Option<i32>,
}

fn app(script: Dialogue) -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        ScenePlugin,
        StatesPlugin,
    ))
    .init_state::<Phase>()
    .init_resource::<Answers>()
    .insert_resource(script)
    .add_plugins(DialoguePlugin(Phase::Asking));
    // One update enters the state and sets the cursor, a later one builds the
    // screen from it. Scene spawning is queued, so wait for the widgets rather
    // than counting frames.
    run_until(&mut app, "the first screen to appear", |w| {
        count::<BbButton>(w) > 0
    });
    app
}

/// Runs the app until `ready` holds, then returns.
fn run_until(app: &mut App, what: &str, ready: impl Fn(&mut World) -> bool) {
    for _ in 0..20 {
        app.update();
        if ready(app.world_mut()) {
            return;
        }
    }
    panic!("{what} did not happen within 20 frames");
}

/// How many entities carry `C`.
fn count<C: Component>(world: &mut World) -> usize {
    world
        .query_filtered::<Entity, With<C>>()
        .iter(world)
        .count()
}

/// Presses the button whose caption is `label`.
fn press(app: &mut App, label: &str) {
    let world = app.world_mut();
    let mut buttons = world.query_filtered::<(Entity, &Children), With<BbButton>>();
    let found: Vec<_> = buttons.iter(world).map(|(e, c)| (e, c.to_vec())).collect();

    let target = found
        .into_iter()
        .find(|(_, children)| {
            children
                .iter()
                .any(|c| world.get::<Text>(*c).is_some_and(|t| t.0 == label))
        })
        .map(|(e, _)| e)
        .unwrap_or_else(|| panic!("no button captioned {label:?}"));

    world.trigger(Activate { entity: target });
    // The press may swap the screen, so wait for the old buttons to be gone and
    // whatever replaces them to have settled.
    let before = target;
    run_until(app, "the press to take effect", |w| {
        w.get_entity(before).is_err() || w.get::<Children>(before).is_some()
    });
}

fn script() -> Dialogue {
    dialogue(
        "start",
        [
            (
                "start",
                ask(
                    "Do you agree?",
                    [
                        choice("Yes", To("score")).record(|a: &mut Answers| a.agreed = true),
                        choice("No", Done).record(|a: &mut Answers| a.agreed = false),
                    ],
                )
                .aside("There is a correct answer."),
            ),
            (
                "score",
                rate(
                    "Rate us.",
                    Rating::new(1, 10)
                        .start(4)
                        .at_least(8, "Too low. Raise it.")
                        .store(|a: &mut Answers, score| a.score = Some(score)),
                    Done,
                ),
            ),
        ],
    )
}

#[test]
fn a_choice_records_itself_and_advances_the_cursor() {
    let mut app = app(script());
    assert_eq!(app.world().resource::<Cursor>().at, "start");

    press(&mut app, "Yes");

    assert_eq!(app.world().resource::<Cursor>().at, "score");
    assert!(app.world().resource::<Answers>().agreed);
    assert_eq!(
        app.world().resource::<Transcript>().0,
        vec![("start", "Yes".to_string())]
    );
}

/// The prompt and its aside must not only exist, they must come *before* the
/// answers in the child order, or they render underneath them.
#[test]
fn a_screen_reads_prompt_then_aside_then_answers() {
    let mut app = app(script());
    let world = app.world_mut();

    // The screen is the parent of the buttons.
    let mut buttons = world.query_filtered::<&ChildOf, With<BbButton>>();
    let screen = buttons.iter(world).next().expect("a button").parent();

    let children = world.get::<Children>(screen).expect("a screen").to_vec();
    let described: Vec<String> = children
        .iter()
        .map(|e| match world.get::<Text>(*e) {
            Some(t) => t.0.clone(),
            None => "<answer>".to_string(),
        })
        .collect();

    assert_eq!(
        described,
        vec![
            "Do you agree?".to_string(),
            "There is a correct answer.".to_string(),
            "<answer>".to_string(),
            "<answer>".to_string(),
        ],
        "screen is in the wrong order"
    );
}

#[test]
fn a_rating_below_the_floor_is_refused() {
    let mut app = app(script());
    press(&mut app, "Yes");

    // The slider starts at 4, under the floor of 8.
    let world = app.world_mut();
    let mut sliders = world.query_filtered::<&SliderValue, With<BbSlider>>();
    assert_eq!(sliders.single(world).expect("a slider").0, 4.0);

    press(&mut app, "Submit");

    // Refused: still on the same step, nothing stored, complaint shown.
    assert_eq!(app.world().resource::<Cursor>().at, "score");
    assert_eq!(app.world().resource::<Answers>().score, None);

    let world = app.world_mut();
    let mut texts = world.query::<&Text>();
    let lines: Vec<String> = texts.iter(world).map(|t| t.0.clone()).collect();
    assert!(
        lines.iter().any(|l| l == "Too low. Raise it."),
        "expected the complaint, got {lines:?}"
    );
}

#[test]
fn an_accepted_rating_stores_and_finishes() {
    let mut app = app(script());
    press(&mut app, "Yes");

    // Drag the slider up to an answer we are willing to hear.
    let world = app.world_mut();
    let mut sliders = world.query_filtered::<Entity, With<BbSlider>>();
    let slider = sliders.single(world).expect("a slider");
    world.entity_mut(slider).insert(SliderValue(9.0));
    app.update();

    press(&mut app, "Submit");

    assert_eq!(app.world().resource::<Answers>().score, Some(9));
    let finished = app.world().resource::<Messages<DialogueFinished>>();
    assert!(!finished.is_empty(), "dialogue should have finished");
}

/// The select path, which is the piece that changed most: the hand-rolled
/// `Selected` marker is gone and `RadioGroup` maintains `Checked` instead.
#[test]
fn a_select_commits_the_checked_option() {
    use bb_installer_bevy::script::select;
    use bb_installer_bevy::ui::button::BbRadioGroup;
    use bevy::ui::Checked;
    use bevy::ui_widgets::ValueChange;

    let mut app = app(dialogue(
        "pledge",
        [(
            "pledge",
            select(
                "Pledge something.",
                [
                    choice("My data", Done).record(|a: &mut Answers| a.score = Some(1)),
                    choice("My firstborn", Done).record(|a: &mut Answers| a.score = Some(2)),
                ],
            ),
        )],
    ));

    // Find the group and the option captioned "My firstborn".
    let world = app.world_mut();
    let mut groups = world.query_filtered::<Entity, With<BbRadioGroup>>();
    let group = groups.single(world).expect("a radio group");

    let mut options = world.query_filtered::<(Entity, &Children), With<BbButton>>();
    let found: Vec<_> = options.iter(world).map(|(e, c)| (e, c.to_vec())).collect();
    let wanted = found
        .into_iter()
        .find(|(_, children)| {
            children
                .iter()
                .any(|c| world.get::<Text>(*c).is_some_and(|t| t.0 == "My firstborn"))
        })
        .map(|(e, _)| e)
        .expect("the second option");

    // What a real pointer click makes the group emit.
    world.trigger(ValueChange {
        source: group,
        value: wanted,
        is_final: true,
    });
    app.update();

    // `radio_self_update` should have latched exactly that one.
    let world = app.world_mut();
    assert!(world.get::<Checked>(wanted).is_some(), "option not checked");
    let mut checked = world.query_filtered::<Entity, With<Checked>>();
    assert_eq!(
        checked.iter(world).count(),
        1,
        "more than one option latched"
    );

    press(&mut app, "Confirm");

    assert_eq!(app.world().resource::<Answers>().score, Some(2));
    assert_eq!(
        app.world().resource::<Transcript>().0,
        vec![("pledge", "My firstborn".to_string())]
    );
}
