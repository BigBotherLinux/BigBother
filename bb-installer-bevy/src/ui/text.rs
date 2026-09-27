//! Text, in the toolkit's voice.
//!
//! A [`TextStyle`] is a colour token plus a size. Naming the handful the app
//! actually uses, once, is what stops `font_size: 24.0` from being retyped in
//! thirty places and drifting.

use bevy::prelude::*;

use super::theme::{ThemedText, Token};

/// One named way of setting text.
#[derive(Clone, Copy, Debug)]
pub struct TextStyle {
    pub color: Token,
    pub size: f32,
}

/// The styles this app has. Add to these rather than passing sizes inline.
pub mod styles {
    use super::TextStyle;
    use crate::ui::theme::tokens;

    /// The app's name, and nothing else.
    pub const TITLE: TextStyle = TextStyle {
        color: tokens::TEXT_BRAND,
        size: 72.0,
    };
    /// The question being asked.
    pub const PROMPT: TextStyle = TextStyle {
        color: tokens::TEXT,
        size: 30.0,
    };
    /// Body copy.
    pub const BODY: TextStyle = TextStyle {
        color: tokens::TEXT,
        size: 28.0,
    };
    /// Small print under a prompt.
    pub const ASIDE: TextStyle = TextStyle {
        color: tokens::TEXT_DIM,
        size: 20.0,
    };
    /// A complaint.
    pub const ERROR: TextStyle = TextStyle {
        color: tokens::TEXT_ERROR,
        size: 20.0,
    };
    /// A button's label.
    pub const LABEL: TextStyle = TextStyle {
        color: tokens::BUTTON_TEXT,
        size: 24.0,
    };
    /// What someone has typed into a text field.
    pub const INPUT: TextStyle = TextStyle {
        color: tokens::TEXT_INPUT_TEXT,
        size: 24.0,
    };
    /// The number drawn on a slider track.
    pub const READOUT: TextStyle = TextStyle {
        color: tokens::SLIDER_TEXT,
        size: 24.0,
    };
}

/// One text entity, themed and sized.
///
/// Use this when you need the entity back, such as a line you intend to fill in
/// later. For the common case of handing text to a widget prop, [`text`] wraps
/// this in the list type those props take.
pub fn line(content: impl Into<String>, style: TextStyle) -> impl Scene {
    let content = content.into();
    bsn! {
        Text({content})
        TextFont {
            font_size: {FontSize::Px(style.size)},
        }
        TextColor
        ThemedText({style.color})
    }
}

/// One text entity as a list of one, which is the type widget props take for
/// their children. The same helper therefore works as a button caption and as a
/// plain line on a screen.
pub fn text(content: impl Into<String>, style: TextStyle) -> Box<dyn SceneList> {
    Box::new(vec![line(content, style)])
}

/// An empty text entity that something else will fill in later, such as a
/// validation complaint that only appears once earned.
pub fn blank(style: TextStyle) -> Box<dyn SceneList> {
    text("", style)
}

/// A button caption, which is the common case of [`text`].
pub fn label(content: impl Into<String>) -> Box<dyn SceneList> {
    text(content, styles::LABEL)
}
