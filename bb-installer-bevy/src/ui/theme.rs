//! Every colour in the toolkit is decided here, and nowhere else.
//!
//! Widgets never name a colour. They name a [`Token`] — "the background of a
//! hovered button" — and park it on the entity as [`ThemedBackground`],
//! [`ThemedBorder`] or [`ThemedText`]. One system resolves tokens to colours
//! through the [`Theme`] resource.
//!
//! The point of the indirection is that a second look is a different [`Theme`]
//! value, not a fork of every widget. It also means a widget's state machine
//! (hover, press, check) is written in terms of *meaning* rather than RGB, so
//! you can read `BUTTON_BG_PRESSED` and know what it is for.

use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::text::TextCursorStyle;

/// The name of one colour in the palette.
///
/// A plain interned string rather than an enum, so a game can invent tokens the
/// toolkit has never heard of and put them in its own [`Theme`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Token(pub &'static str);

/// The tokens this toolkit's own widgets use.
pub mod tokens {
    use super::Token;

    /// What the camera clears to.
    pub const SCREEN_BG: Token = Token("screen.bg");

    /// A question, or any other primary text.
    pub const TEXT: Token = Token("text");
    /// The product name, in the product's colour.
    pub const TEXT_BRAND: Token = Token("text.brand");
    /// Small print, footnotes, snide remarks.
    pub const TEXT_DIM: Token = Token("text.dim");
    /// A complaint about what the user just did.
    pub const TEXT_ERROR: Token = Token("text.error");

    /// Regular button, at rest.
    pub const BUTTON_BG: Token = Token("button.bg");
    pub const BUTTON_BG_HOVER: Token = Token("button.bg.hover");
    pub const BUTTON_BG_PRESSED: Token = Token("button.bg.pressed");
    /// Latched on, as one option of a radio group.
    pub const BUTTON_BG_CHECKED: Token = Token("button.bg.checked");
    pub const BUTTON_BG_DISABLED: Token = Token("button.bg.disabled");
    pub const BUTTON_BORDER: Token = Token("button.border");
    pub const BUTTON_BORDER_HOVER: Token = Token("button.border.hover");
    pub const BUTTON_BORDER_PRESSED: Token = Token("button.border.pressed");
    pub const BUTTON_BORDER_CHECKED: Token = Token("button.border.checked");
    pub const BUTTON_BORDER_DISABLED: Token = Token("button.border.disabled");
    pub const BUTTON_TEXT: Token = Token("button.text");

    /// The call-to-action variant: same shape, louder colours.
    pub const BUTTON_PRIMARY_BG: Token = Token("button.primary.bg");
    pub const BUTTON_PRIMARY_BG_HOVER: Token = Token("button.primary.bg.hover");
    pub const BUTTON_PRIMARY_BG_PRESSED: Token = Token("button.primary.bg.pressed");
    pub const BUTTON_PRIMARY_BG_CHECKED: Token = Token("button.primary.bg.checked");
    pub const BUTTON_PRIMARY_BORDER: Token = Token("button.primary.border");

    /// The empty part of a slider track.
    pub const SLIDER_TRACK: Token = Token("slider.track");
    /// The part behind the value.
    pub const SLIDER_FILL: Token = Token("slider.fill");
    pub const SLIDER_BORDER: Token = Token("slider.border");
    /// The number drawn on the track.
    pub const SLIDER_TEXT: Token = Token("slider.text");

    /// The box of a checkbox. Its fill says checked or not; hover and press
    /// only move the border.
    pub const CHECKBOX_BG: Token = Token("checkbox.bg");
    pub const CHECKBOX_BG_HOVER: Token = Token("checkbox.bg.hover");
    pub const CHECKBOX_BG_CHECKED: Token = Token("checkbox.bg.checked");
    pub const CHECKBOX_BG_DISABLED: Token = Token("checkbox.bg.disabled");
    pub const CHECKBOX_BORDER: Token = Token("checkbox.border");
    pub const CHECKBOX_BORDER_HOVER: Token = Token("checkbox.border.hover");
    pub const CHECKBOX_BORDER_PRESSED: Token = Token("checkbox.border.pressed");
    pub const CHECKBOX_BORDER_CHECKED: Token = Token("checkbox.border.checked");
    pub const CHECKBOX_BORDER_DISABLED: Token = Token("checkbox.border.disabled");
    /// The tick.
    pub const CHECKBOX_MARK: Token = Token("checkbox.mark");
    pub const CHECKBOX_MARK_DISABLED: Token = Token("checkbox.mark.disabled");

    /// A text field's frame.
    pub const TEXT_INPUT_BG: Token = Token("text_input.bg");
    pub const TEXT_INPUT_BORDER: Token = Token("text_input.border");
    pub const TEXT_INPUT_BORDER_HOVER: Token = Token("text_input.border.hover");
    /// The field being typed into.
    pub const TEXT_INPUT_BORDER_FOCUS: Token = Token("text_input.border.focus");
    /// What has been typed.
    pub const TEXT_INPUT_TEXT: Token = Token("text_input.text");
    pub const TEXT_INPUT_CARET: Token = Token("text_input.caret");
    pub const TEXT_INPUT_SELECTION: Token = Token("text_input.selection");
    /// A selection left behind in a field that has since lost focus.
    pub const TEXT_INPUT_SELECTION_UNFOCUSED: Token = Token("text_input.selection.unfocused");
}

/// The palette in force. Overwrite the resource to restyle the whole app.
#[derive(Resource, Debug)]
pub struct Theme {
    colors: HashMap<Token, Color>,
}

impl Theme {
    /// A theme with no colours at all. Useful as a starting point for your own.
    pub fn empty() -> Self {
        Self {
            colors: HashMap::new(),
        }
    }

    /// Assign a colour to a token, replacing any previous assignment.
    pub fn set(&mut self, token: Token, color: Color) -> &mut Self {
        self.colors.insert(token, color);
        self
    }

    /// The colour for `token`.
    ///
    /// An unassigned token is a bug in the widget or a gap in the theme, so it
    /// resolves to magenta and complains once rather than failing quietly.
    pub fn color(&self, token: Token) -> Color {
        match self.colors.get(&token) {
            Some(color) => *color,
            None => {
                warn_once!("theme has no colour for token {:?}", token.0);
                Color::srgb(1.0, 0.0, 1.0)
            }
        }
    }
}

impl Default for Theme {
    /// BigBother's dark palette: near-black, off-white text, sickly green accent.
    fn default() -> Self {
        use tokens::*;

        let grey = |v: f32| Color::srgb(v, v, v);
        let green = Color::srgb(0.35, 0.75, 0.35);
        let dark_green = Color::srgb(0.2, 0.45, 0.2);

        let mut theme = Self::empty();
        theme
            .set(SCREEN_BG, Color::srgb(0.05, 0.05, 0.07))
            .set(TEXT, Color::srgb(0.9, 0.9, 0.95))
            .set(TEXT_BRAND, Color::srgb(0.9, 0.1, 0.1))
            .set(TEXT_DIM, Color::srgb(0.5, 0.5, 0.55))
            .set(TEXT_ERROR, Color::srgb(0.85, 0.3, 0.3))
            .set(BUTTON_BG, grey(0.15))
            .set(BUTTON_BG_HOVER, grey(0.25))
            .set(BUTTON_BG_PRESSED, green)
            .set(BUTTON_BG_CHECKED, dark_green)
            .set(BUTTON_BG_DISABLED, grey(0.1))
            .set(BUTTON_BORDER, Color::WHITE)
            .set(BUTTON_BORDER_HOVER, Color::WHITE)
            .set(BUTTON_BORDER_PRESSED, Color::srgb(0.3, 0.3, 1.0))
            .set(BUTTON_BORDER_CHECKED, Color::srgb(0.0, 1.0, 0.0))
            .set(BUTTON_BORDER_DISABLED, grey(0.3))
            .set(BUTTON_TEXT, Color::WHITE)
            .set(BUTTON_PRIMARY_BG, dark_green)
            .set(BUTTON_PRIMARY_BG_HOVER, Color::srgb(0.25, 0.55, 0.25))
            .set(BUTTON_PRIMARY_BG_PRESSED, green)
            .set(BUTTON_PRIMARY_BG_CHECKED, green)
            .set(BUTTON_PRIMARY_BORDER, green)
            .set(SLIDER_TRACK, Color::srgb(0.12, 0.12, 0.14))
            .set(SLIDER_FILL, green)
            .set(SLIDER_BORDER, Color::WHITE)
            .set(SLIDER_TEXT, Color::WHITE)
            .set(CHECKBOX_BG, grey(0.15))
            .set(CHECKBOX_BG_HOVER, grey(0.25))
            .set(CHECKBOX_BG_CHECKED, dark_green)
            .set(CHECKBOX_BG_DISABLED, grey(0.1))
            .set(CHECKBOX_BORDER, Color::WHITE)
            .set(CHECKBOX_BORDER_HOVER, green)
            .set(CHECKBOX_BORDER_PRESSED, Color::srgb(0.3, 0.3, 1.0))
            .set(CHECKBOX_BORDER_CHECKED, Color::srgb(0.0, 1.0, 0.0))
            .set(CHECKBOX_BORDER_DISABLED, grey(0.3))
            .set(CHECKBOX_MARK, Color::WHITE)
            .set(CHECKBOX_MARK_DISABLED, grey(0.4))
            .set(TEXT_INPUT_BG, Color::srgb(0.12, 0.12, 0.14))
            .set(TEXT_INPUT_BORDER, grey(0.6))
            .set(TEXT_INPUT_BORDER_HOVER, Color::WHITE)
            .set(TEXT_INPUT_BORDER_FOCUS, green)
            .set(TEXT_INPUT_TEXT, Color::srgb(0.9, 0.9, 0.95))
            .set(TEXT_INPUT_CARET, green)
            .set(TEXT_INPUT_SELECTION, dark_green)
            .set(TEXT_INPUT_SELECTION_UNFOCUSED, grey(0.25));
        theme
    }
}

/// Paint this entity's [`BackgroundColor`] from a token.
#[derive(Component, Default, Clone, PartialEq, Debug)]
pub struct ThemedBackground(pub Token);

/// Paint this entity's [`BorderColor`] from a token, on all four sides.
#[derive(Component, Default, Clone, PartialEq, Debug)]
pub struct ThemedBorder(pub Token);

/// Paint this entity's [`TextColor`] from a token.
#[derive(Component, Default, Clone, PartialEq, Debug)]
pub struct ThemedText(pub Token);

/// Paint this entity's [`TextCursorStyle`] from tokens. A text field's caret
/// and selection are colours like any other, so they are themed like any other.
#[derive(Component, Default, Clone, PartialEq, Debug)]
pub struct ThemedCursor {
    pub caret: Token,
    pub selection: Token,
    pub selection_unfocused: Token,
}

/// Resolves tokens to colours.
///
/// Repaints an entity when its token changes, and repaints everything when the
/// [`Theme`] itself changes. Widgets therefore only ever *swap tokens*, which
/// is a cheap comparison, and never compute a colour themselves.
pub fn paint(
    theme: Res<Theme>,
    mut backgrounds: Query<(Ref<ThemedBackground>, &mut BackgroundColor)>,
    mut borders: Query<(Ref<ThemedBorder>, &mut BorderColor)>,
    mut texts: Query<(Ref<ThemedText>, &mut TextColor)>,
    mut cursors: Query<(Ref<ThemedCursor>, &mut TextCursorStyle)>,
) {
    let restyled = theme.is_changed();

    for (token, mut color) in &mut backgrounds {
        if restyled || token.is_changed() {
            color.0 = theme.color(token.0);
        }
    }
    for (token, mut color) in &mut borders {
        if restyled || token.is_changed() {
            color.set_all(theme.color(token.0));
        }
    }
    for (token, mut color) in &mut texts {
        if restyled || token.is_changed() {
            color.0 = theme.color(token.0);
        }
    }
    for (tokens, mut style) in &mut cursors {
        if restyled || tokens.is_changed() {
            style.color = theme.color(tokens.caret);
            style.selection_color = theme.color(tokens.selection);
            style.unfocused_selection_color = theme.color(tokens.selection_unfocused);
        }
    }
}

/// Keeps the window's clear colour in step with the theme, so the background is
/// themed like everything else instead of being set by hand in `main`.
pub fn paint_screen(theme: Res<Theme>, mut clear: ResMut<ClearColor>) {
    if theme.is_changed() {
        clear.0 = theme.color(tokens::SCREEN_BG);
    }
}
