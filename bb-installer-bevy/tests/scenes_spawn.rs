//! Scene components only report failure at runtime, so this asserts the widgets
//! really do expand into the entities their scenes describe.

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::scene::ScenePlugin;
use bevy::ui::Checked;
use bevy::ui_widgets::{Slider, SliderValue};

use bb_installer_bevy::ui::button::{BbButton, BbRadioButton, BbRadioGroup};
use bb_installer_bevy::ui::checkbox::{BbCheckbox, CheckboxBox, CheckboxMark};
use bb_installer_bevy::ui::slider::{BbSlider, TrackFill, TrackReadout};
use bb_installer_bevy::ui::text::{label, styles, text};
use bb_installer_bevy::ui::text_input::BbTextInput;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), ScenePlugin));
    app
}

/// Runs the app until `ready` holds, then returns. Scene spawning is *queued*
/// behind its dependencies, so asserting after a fixed number of frames is a
/// race; this waits for the thing itself instead.
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

#[test]
fn button_gets_its_caption() {
    let mut app = app();
    app.world_mut()
        .commands()
        .spawn_scene(bsn! { @BbButton { @caption: {label("Continue")} } });
    run_until(&mut app, "the button to spawn", |w| {
        count::<BbButton>(w) == 1
    });

    let world = app.world_mut();
    let mut buttons = world.query_filtered::<&Children, With<BbButton>>();
    let children: Vec<_> = buttons.iter(world).collect();
    assert_eq!(children.len(), 1, "expected exactly one button");

    let caption = children[0][0];
    assert_eq!(
        world.get::<Text>(caption).map(|t| t.0.clone()),
        Some("Continue".to_string())
    );
}

#[test]
fn slider_owns_its_fill_and_readout() {
    let mut app = app();
    app.world_mut().commands().spawn_scene(bsn! {
        @BbSlider { @min: 1, @max: 10, @start: 7 }
    });
    run_until(&mut app, "the slider to spawn", |w| {
        count::<TrackFill>(w) == 1 && count::<TrackReadout>(w) == 1
    });

    let world = app.world_mut();
    let mut sliders = world.query_filtered::<(&SliderValue, &Children), With<BbSlider>>();
    let (value, children) = sliders.single(world).expect("one slider");
    assert_eq!(value.0, 7.0);

    let children = children.to_vec();
    assert!(
        children
            .iter()
            .any(|e| world.get::<TrackFill>(*e).is_some()),
        "slider should own a fill bar"
    );
    let readout = children
        .iter()
        .find(|e| world.get::<TrackReadout>(**e).is_some())
        .expect("slider should own a readout");
    assert_eq!(world.get::<TrackReadout>(*readout).unwrap().max, 10);
}

#[test]
fn radio_button_inherits_the_whole_button() {
    let mut app = app();
    app.world_mut().commands().spawn_scene(bsn! {
        @BbRadioGroup {
            @options: {Box::new(bsn_list!(
                @BbRadioButton { @caption: {label("One")} },
                @BbRadioButton { @caption: {label("Two")} }
            )) as Box<dyn SceneList>}
        }
    });
    run_until(&mut app, "both options to spawn", |w| {
        count::<BbRadioButton>(w) == 2
    });

    let world = app.world_mut();
    let mut groups = world.query_filtered::<&Children, With<BbRadioGroup>>();
    let options = groups.single(world).expect("one group").to_vec();
    assert_eq!(options.len(), 2);

    for option in options {
        // Inherited from BbButton, not restated by BbRadioButton.
        assert!(world.get::<BbButton>(option).is_some(), "missing BbButton");
        assert!(world.get::<Node>(option).is_some(), "missing Node");
        assert!(world.get::<Children>(option).is_some(), "missing caption");
    }
}

#[test]
fn slider_is_not_confused_with_a_plain_node() {
    let mut app = app();
    app.world_mut()
        .commands()
        .spawn_scene(bsn! { @BbSlider {} });
    run_until(&mut app, "the slider to spawn", |w| {
        count::<BbSlider>(w) == 1
    });

    let world = app.world_mut();
    let mut sliders = world.query_filtered::<Entity, (With<Slider>, With<BbSlider>)>();
    assert_eq!(sliders.iter(world).count(), 1);
}

/// Checks the `on(...)` form in the module docs really compiles with a closure,
/// not just with a named system.
#[test]
fn a_button_can_carry_its_observer_inline() {
    use bevy::ui_widgets::Activate;

    #[derive(Resource, Default)]
    struct Pressed(bool);

    let mut app = app();
    app.init_resource::<Pressed>();
    app.world_mut().commands().spawn_scene(bsn! {
        @BbButton { @caption: {label("Go")} }
        on(|_: On<Activate>, mut pressed: ResMut<Pressed>| pressed.0 = true)
    });
    run_until(&mut app, "the button to spawn", |w| {
        count::<BbButton>(w) == 1
    });

    let world = app.world_mut();
    let mut buttons = world.query_filtered::<Entity, With<BbButton>>();
    let button = buttons.single(world).expect("one button");
    world.trigger(Activate { entity: button });
    app.update();

    assert!(app.world().resource::<Pressed>().0);
}

#[test]
fn checkbox_owns_its_box_tick_and_caption() {
    let mut app = app();
    app.world_mut().commands().spawn_scene(bsn! {
        @BbCheckbox { @caption: {text("I agree", styles::BODY)} }
    });
    run_until(&mut app, "the checkbox to spawn", |w| {
        count::<CheckboxBox>(w) == 1 && count::<CheckboxMark>(w) == 1
    });

    let world = app.world_mut();
    let mut checkboxes = world.query_filtered::<(Entity, &Children), With<BbCheckbox>>();
    let (checkbox, children) = checkboxes.single(world).expect("one checkbox");
    let children = children.to_vec();
    assert_eq!(children.len(), 2, "expected the box and the caption");
    assert!(world.get::<CheckboxBox>(children[0]).is_some());
    assert_eq!(
        world.get::<Text>(children[1]).map(|t| t.0.clone()),
        Some("I agree".to_string())
    );
    assert!(
        world.get::<Checked>(checkbox).is_none(),
        "should start unticked"
    );
}

/// The stock self-update observer is attached, so a change sticks without the
/// caller managing [`Checked`].
#[test]
fn checkbox_keeps_its_own_state() {
    use bevy::ui_widgets::ValueChange;

    let mut app = app();
    app.world_mut()
        .commands()
        .spawn_scene(bsn! { @BbCheckbox {} Checked });
    run_until(&mut app, "the checkbox to spawn", |w| {
        count::<BbCheckbox>(w) == 1
    });

    let world = app.world_mut();
    let mut checkboxes = world.query_filtered::<Entity, With<BbCheckbox>>();
    let checkbox = checkboxes.single(world).expect("one checkbox");
    assert!(
        world.get::<Checked>(checkbox).is_some(),
        "should start ticked"
    );

    world.trigger(ValueChange {
        source: checkbox,
        value: false,
        is_final: true,
    });
    app.update();
    assert!(app.world().get::<Checked>(checkbox).is_none());
}

#[test]
fn text_input_starts_with_its_initial_text() {
    use bevy::text::EditableText;

    let mut app = app();
    app.world_mut().commands().spawn_scene(bsn! {
        @BbTextInput { @initial: {"bigbother".to_string()}, @max_characters: {Some(32)} }
    });
    run_until(&mut app, "the text input to spawn", |w| {
        count::<BbTextInput>(w) == 1
    });

    let world = app.world_mut();
    let mut inputs = world.query_filtered::<&EditableText, With<BbTextInput>>();
    let editable = inputs.single(world).expect("one text input");
    assert_eq!(editable.value().to_string(), "bigbother");
    assert_eq!(editable.max_characters, Some(32));
}
