//! A small, reusable Bevy UI toolkit.
//!
//! Three rules hold this together, and they are the reason it can be lifted
//! into another project:
//!
//! 1. **The toolkit owns its behaviour.** Everything a widget needs to work is
//!    registered by [`BbUiPlugin`]. Adding the plugin is the whole setup; no app
//!    has to remember to schedule a styling system.
//! 2. **No colour is written in a widget.** Widgets name a
//!    [`Token`](theme::Token) and [`theme`] resolves it. Restyling is a new
//!    [`Theme`](theme::Theme) value, not a fork.
//! 3. **Widgets emit events and never import app types.** A button fires
//!    [`Activate`](bevy::ui_widgets::Activate) and has no idea what it means.
//!    The app attaches an observer where it spawns the widget. This is what
//!    keeps the dependency arrow pointing one way, from app to toolkit.
//!
//! Widgets are *scene components*: a marker component with a scene attached, in
//! the style of `bevy_feathers`. They can only be spawned through the scene
//! API. `commands.spawn(BbButton)` compiles and then logs an error at runtime,
//! so reach for `spawn_scene`, or [`spawn_child`] when the widget belongs to a
//! screen you have already made.
//!
//! Two sharp edges worth knowing before you build on this:
//!
//! - `queue_spawn_related_scenes` looks like the tidy way to add several
//!   children at once, but it resolves *asynchronously*. Mixing it with plain
//!   `spawn_scene` siblings gets you a child order that does not match the order
//!   you wrote, and a frame where only some children exist.
//! - A component named inside `bsn!` needs `Default` and `Clone`, or
//!   `FromTemplate` if one of its fields is an `Entity` or a `Handle`.
//!
//! ```ignore
//! commands.spawn_scene(bsn! {
//!     @BbButton {
//!         @caption: {label("Continue")},
//!         @variant: ButtonVariant::Primary
//!     }
//!     on(|_: On<Activate>, mut state: ResMut<NextState<AppState>>| {
//!         state.set(AppState::Welcome);
//!     })
//! });
//! ```
//!
//! What is deliberately *not* here: input handling. `bevy_ui_widgets` already
//! ships the headless behaviour for buttons, sliders, radio groups, checkboxes,
//! scroll areas and menus, and `bevy_text` ships editable text. Each widget
//! here is that behaviour plus a themed frame.

pub mod button;
pub mod checkbox;
pub mod slider;
pub mod text;
pub mod text_input;
pub mod theme;

use bevy::prelude::*;

// The toolkit's public surface, gathered in one place.
pub use button::{
    BbButton, BbRadioButton, BbRadioGroup, ButtonProps, ButtonVariant, RadioGroupProps,
};
pub use checkbox::{BbCheckbox, CheckboxProps};
pub use slider::{BbSlider, SliderProps};
pub use text::{label, styles, text, TextStyle};
pub use text_input::{BbTextInput, TextInputProps};
pub use theme::{tokens, Theme, ThemedBackground, ThemedBorder, ThemedCursor, ThemedText, Token};

/// Spawns a scene as a child of `parent`, handing back its commands so a
/// handler can be attached in the same breath:
///
/// ```ignore
/// spawn_child(commands, screen, bsn! { @BbButton { @caption: {label("Yes")} } })
///     .observe(move |_: On<Activate>, ..| { .. });
/// ```
///
/// This exists because scenes have no `children!`-style shorthand: a scene
/// component cannot be nested in an ordinary bundle, so a child is spawned on its
/// own and then given a parent. Doing it one command at a time also keeps the
/// child order the same as the order you wrote.
pub fn spawn_child<'a>(
    commands: &'a mut Commands,
    parent: Entity,
    scene: impl Scene,
) -> EntityCommands<'a> {
    let mut child = commands.spawn_scene(scene);
    child.insert(ChildOf(parent));
    child
}

/// Installs the toolkit. Add it once and every widget in here works.
pub struct BbUiPlugin;

impl Plugin for BbUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Theme>().add_systems(
            Update,
            // Chained because the feedback systems choose tokens and `paint`
            // turns tokens into colours. Running them in this order means a
            // hover is visible on the same frame it happens.
            (
                button::button_feedback,
                checkbox::checkbox_feedback,
                slider::slider_feedback,
                text_input::text_input_feedback,
                theme::paint,
                theme::paint_screen,
            )
                .chain(),
        );
    }
}
