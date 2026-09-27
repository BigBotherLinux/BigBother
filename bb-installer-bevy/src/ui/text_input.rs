//! A single-line text field.
//!
//! Editing is [`EditableText`]'s: typing, selection, clipboard, IME and
//! pointer placement all come from `bevy_text` and `bevy_ui_widgets`. What this
//! adds is the themed frame and one convenience, turning Enter into
//! [`Activate`] so a field can submit a form the way a button does.
//!
//! The frame is drawn by the *same* entity that holds the text, using the
//! node's own padding, border and background. There is no wrapper to reach
//! through: the entity a caller spawns is the one whose value they read.

use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ButtonState;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::input_focus::{FocusedInput, InputFocus};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::{EditableText, LineBreak, TextCursorStyle};
use bevy::ui::px;
use bevy::ui_widgets::Activate;

use super::text::{styles, TextStyle};
use super::theme::{tokens, ThemedBackground, ThemedBorder, ThemedCursor, ThemedText};

/// A one-line text field.
///
/// # Emits
/// - [`TextEditChange`](bevy::text::TextEditChange) after every edit. It also
///   fires for caret movement, so compare [`EditableText::value`] if you only
///   care about the text.
/// - [`Activate`] when Enter is pressed while the field has focus.
///
/// Either way, read the text from [`EditableText`] on this entity.
#[derive(SceneComponent, Default, Clone)]
#[scene(TextInputProps)]
pub struct BbTextInput;

/// Construction options for [`BbTextInput`].
pub struct TextInputProps {
    /// What the field holds before anyone types. The caret starts at its end.
    pub initial: String,
    /// Edits past this many characters are ignored.
    pub max_characters: Option<usize>,
    pub width: Val,
    pub style: TextStyle,
}

impl Default for TextInputProps {
    fn default() -> Self {
        Self {
            initial: String::new(),
            max_characters: None,
            width: px(520),
            style: styles::INPUT,
        }
    }
}

impl BbTextInput {
    fn scene(props: TextInputProps) -> impl Scene {
        let mut editable = EditableText::new(&props.initial);
        editable.max_characters = props.max_characters;

        bsn! {
            Node {
                width: {props.width},
                padding: {UiRect::axes(px(12), px(8))},
                border: {UiRect::all(px(2))},
                border_radius: {BorderRadius::all(px(8))},
            }
            BbTextInput
            template_value(editable)
            TextLayout {
                linebreak: LineBreak::NoWrap,
            }
            TextFont {
                font_size: {FontSize::Px(props.style.size)},
            }
            Hovered
            TabIndex(0)
            BackgroundColor
            BorderColor
            TextColor
            TextCursorStyle
            ThemedBackground({tokens::TEXT_INPUT_BG})
            ThemedBorder({tokens::TEXT_INPUT_BORDER})
            ThemedText({props.style.color})
            ThemedCursor {
                caret: {tokens::TEXT_INPUT_CARET},
                selection: {tokens::TEXT_INPUT_SELECTION},
                selection_unfocused: {tokens::TEXT_INPUT_SELECTION_UNFOCUSED},
            }
            on(submit_on_enter)
        }
    }
}

/// Turns Enter into [`Activate`].
///
/// `bevy_ui_widgets` lets Enter through on a field that does not allow
/// newlines precisely so something like this can claim it. It is skipped while
/// an IME is composing, where Enter means "accept this candidate".
fn submit_on_enter(
    mut key: On<FocusedInput<KeyboardInput>>,
    inputs: Query<&EditableText, With<BbTextInput>>,
    mut commands: Commands,
) {
    let Ok(editable) = inputs.get(key.focused_entity) else {
        return;
    };
    let input = &key.input;
    if input.key_code == KeyCode::Enter
        && input.state == ButtonState::Pressed
        && !input.repeat
        && !editable.is_composing()
    {
        key.propagate(false);
        commands.trigger(Activate {
            entity: key.focused_entity,
        });
    }
}

/// Picks each field's border from hover and focus. Focus wins, so the field
/// being typed into is always the obvious one.
pub fn text_input_feedback(
    focus: Option<Res<InputFocus>>,
    mut inputs: Query<(Entity, &Hovered, &mut ThemedBorder), With<BbTextInput>>,
) {
    let focused = focus.and_then(|focus| focus.get());

    for (entity, hovered, mut border) in &mut inputs {
        let edge = if focused == Some(entity) {
            tokens::TEXT_INPUT_BORDER_FOCUS
        } else if hovered.0 {
            tokens::TEXT_INPUT_BORDER_HOVER
        } else {
            tokens::TEXT_INPUT_BORDER
        };
        if border.0 != edge {
            border.0 = edge;
        }
    }
}
