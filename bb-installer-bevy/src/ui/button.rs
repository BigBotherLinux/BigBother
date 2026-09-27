//! Buttons, and the radio group built out of them.
//!
//! [`BbButton`] is a *scene component*: a marker component with a scene
//! attached. Spawning it through the scene API builds the node, the border, the
//! picking behaviour and the caption in one go. Bevy's own rule of thumb is
//! that anything which spawns children belongs in a scene component rather than
//! in required components, and a button owns its label.
//!
//! Input handling is not ours. [`bevy::ui_widgets::Button`] already turns
//! clicks, Enter and Space into an [`Activate`](bevy::ui_widgets::Activate)
//! event, and [`RadioGroup`] already does arrow-key navigation and mutual
//! exclusion. This file is styling and layout only.

use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::{px, Checked, InteractionDisabled, Pressed};
use bevy::ui_widgets::{radio_self_update, Button, RadioButton, RadioGroup};

use super::theme::{tokens, ThemedBackground, ThemedBorder, Token};

/// How loud a button is. Also the component the styling system reads, so a
/// button can be promoted to primary at runtime by writing to it.
#[derive(Component, Default, Clone, PartialEq, Eq, Debug)]
pub enum ButtonVariant {
    /// The ordinary one.
    #[default]
    Normal,
    /// The one you are meant to press.
    Primary,
}

/// A button that sizes itself to its caption, so an answer can be a full
/// sentence.
///
/// # Emits
/// [`Activate`](bevy::ui_widgets::Activate) on click, Enter or Space. Attach a
/// handler at the spawn site; the button itself knows nothing about the app.
#[derive(SceneComponent, Default, Clone)]
#[scene(ButtonProps)]
pub struct BbButton;

/// Construction options for [`BbButton`].
pub struct ButtonProps {
    /// What the button says. Usually [`super::text::label`].
    pub caption: Box<dyn SceneList>,
    pub variant: ButtonVariant,
    /// Floor on width, so a row of short answers still lines up.
    pub min_width: Val,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            caption: Box::new(bsn_list!()),
            variant: ButtonVariant::default(),
            min_width: px(150),
        }
    }
}

impl BbButton {
    fn scene(props: ButtonProps) -> impl Scene {
        bsn! {
            Node {
                min_width: {props.min_width},
                padding: {UiRect::axes(px(24), px(12))},
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: {UiRect::all(px(2))},
                border_radius: {BorderRadius::all(px(8))},
            }
            BbButton
            Button
            Hovered
            template_value(props.variant)
            BackgroundColor
            BorderColor
            ThemedBackground({tokens::BUTTON_BG})
            ThemedBorder({tokens::BUTTON_BORDER})
            Children [
                {props.caption}
            ]
        }
    }
}

/// One option of a [`BbRadioGroup`]: a [`BbButton`] that latches instead of
/// firing.
///
/// This is what scene-component inheritance is for. `@BbButton` pulls in the
/// whole button, and the only thing added here is the headless
/// [`RadioButton`] behaviour.
#[derive(SceneComponent, Default, Clone)]
#[scene(ButtonProps)]
pub struct BbRadioButton;

impl BbRadioButton {
    fn scene(props: ButtonProps) -> impl Scene {
        bsn! {
            @BbButton {
                @caption: {props.caption},
                @variant: {props.variant},
                @min_width: {props.min_width}
            }
            BbRadioButton
            RadioButton
        }
    }
}

/// A column of [`BbRadioButton`]s, exactly one of which is checked.
///
/// # Emits
/// [`ValueChange<Entity>`](bevy::ui_widgets::ValueChange) naming the chosen
/// button. The stock [`radio_self_update`] observer is attached, so the
/// [`Checked`] component is maintained for you.
#[derive(SceneComponent, Default, Clone)]
#[scene(RadioGroupProps)]
pub struct BbRadioGroup;

/// Construction options for [`BbRadioGroup`].
pub struct RadioGroupProps {
    /// The options, normally a list of [`BbRadioButton`]s.
    pub options: Box<dyn SceneList>,
}

impl Default for RadioGroupProps {
    fn default() -> Self {
        Self {
            options: Box::new(bsn_list!()),
        }
    }
}

impl BbRadioGroup {
    fn scene(props: RadioGroupProps) -> impl Scene {
        bsn! {
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: px(8),
            }
            BbRadioGroup
            RadioGroup
            on(radio_self_update)
            Children [
                {props.options}
            ]
        }
    }
}

/// Everything the look of a button depends on.
type ButtonState<'a> = (
    &'a ButtonVariant,
    &'a Hovered,
    Has<Pressed>,
    Has<Checked>,
    Has<InteractionDisabled>,
    &'a mut ThemedBackground,
    &'a mut ThemedBorder,
);

/// Chooses each button's tokens from its state.
///
/// Note that this writes *tokens*, not colours, and only when the token would
/// actually change. Resolving a token to a colour is
/// [`super::theme::paint`]'s job, which is why nothing here mentions a shade of
/// grey.
pub fn button_feedback(mut buttons: Query<ButtonState, With<BbButton>>) {
    for (variant, hovered, pressed, checked, disabled, mut background, mut border) in &mut buttons {
        let (bg, edge) = tokens_for(variant, hovered.0, pressed, checked, disabled);

        // Guarded so an unchanged button does not look changed to the painter.
        if background.0 != bg {
            background.0 = bg;
        }
        if border.0 != edge {
            border.0 = edge;
        }
    }
}

/// The state machine, in priority order: disabled beats pressed, pressed beats
/// checked, checked beats hover.
fn tokens_for(
    variant: &ButtonVariant,
    hovered: bool,
    pressed: bool,
    checked: bool,
    disabled: bool,
) -> (Token, Token) {
    use ButtonVariant::*;

    if disabled {
        return (tokens::BUTTON_BG_DISABLED, tokens::BUTTON_BORDER_DISABLED);
    }

    match (variant, pressed, checked, hovered) {
        (Normal, true, _, _) => (tokens::BUTTON_BG_PRESSED, tokens::BUTTON_BORDER_PRESSED),
        (Normal, false, true, _) => (tokens::BUTTON_BG_CHECKED, tokens::BUTTON_BORDER_CHECKED),
        (Normal, false, false, true) => (tokens::BUTTON_BG_HOVER, tokens::BUTTON_BORDER_HOVER),
        (Normal, false, false, false) => (tokens::BUTTON_BG, tokens::BUTTON_BORDER),

        (Primary, true, _, _) => (
            tokens::BUTTON_PRIMARY_BG_PRESSED,
            tokens::BUTTON_BORDER_PRESSED,
        ),
        (Primary, false, true, _) => (
            tokens::BUTTON_PRIMARY_BG_CHECKED,
            tokens::BUTTON_BORDER_CHECKED,
        ),
        (Primary, false, false, true) => (
            tokens::BUTTON_PRIMARY_BG_HOVER,
            tokens::BUTTON_PRIMARY_BORDER,
        ),
        (Primary, false, false, false) => {
            (tokens::BUTTON_PRIMARY_BG, tokens::BUTTON_PRIMARY_BORDER)
        }
    }
}
