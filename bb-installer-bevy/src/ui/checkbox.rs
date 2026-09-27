//! A checkbox: a themed box, a tick, and whatever caption you give it.
//!
//! Input handling is [`bevy::ui_widgets::Checkbox`]'s. It turns clicks, Enter
//! and Space into a [`ValueChange<bool>`](bevy::ui_widgets::ValueChange), and
//! the stock [`checkbox_self_update`] observer keeps [`Checked`] in step. This
//! file is styling and layout only, the same split as [`super::button`].

use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::{px, Checked, InteractionDisabled, Pressed, UiTransform};
use bevy::ui_widgets::{checkbox_self_update, Checkbox};

use super::theme::{tokens, ThemedBackground, ThemedBorder, Token};

/// Edge length of the box, matching the slider track's height so the two line
/// up on a form.
const BOX_SIZE: f32 = 28.0;

/// A box that ticks, with a caption beside it. Clicking the caption counts.
///
/// Starts unticked. To start ticked, add [`Checked`] at the spawn site:
///
/// ```ignore
/// bsn! { @BbCheckbox { @caption: {text("Send telemetry", styles::BODY)} } Checked }
/// ```
///
/// # Emits
/// [`ValueChange<bool>`](bevy::ui_widgets::ValueChange) with the new state on
/// click, Enter or Space. [`Checked`] on this entity is always current, so a
/// caller can simply read it instead.
#[derive(SceneComponent, Default, Clone)]
#[scene(CheckboxProps)]
pub struct BbCheckbox;

/// Construction options for [`BbCheckbox`].
pub struct CheckboxProps {
    /// What is being agreed to. Usually [`super::text::text`] in
    /// [`styles::BODY`](super::text::styles::BODY).
    pub caption: Box<dyn SceneList>,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            caption: Box::new(bsn_list!()),
        }
    }
}

impl BbCheckbox {
    fn scene(props: CheckboxProps) -> impl Scene {
        bsn! {
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: px(12),
            }
            BbCheckbox
            Checkbox
            Hovered
            TabIndex(0)
            on(checkbox_self_update)
            Children [
                (
                    CheckboxBox
                    Node {
                        width: {px(BOX_SIZE)},
                        height: {px(BOX_SIZE)},
                        // Never squashed by a long caption.
                        flex_shrink: 0.0,
                        border: {UiRect::all(px(2))},
                        border_radius: {BorderRadius::all(px(6))},
                    }
                    BackgroundColor
                    BorderColor
                    ThemedBackground({tokens::CHECKBOX_BG})
                    ThemedBorder({tokens::CHECKBOX_BORDER})
                    Children [(
                        // The tick: two sides of a tall rectangle, turned 45°.
                        CheckboxMark
                        Node {
                            position_type: PositionType::Absolute,
                            left: {px(8)},
                            top: {px(2)},
                            width: {px(8)},
                            height: {px(15)},
                            border: {UiRect {
                                bottom: px(3),
                                right: px(3),
                                ..default()
                            }},
                        }
                        template_value(UiTransform::from_rotation(Rot2::FRAC_PI_4))
                        template_value(Visibility::Hidden)
                        BorderColor
                        ThemedBorder({tokens::CHECKBOX_MARK})
                    )]
                ),
                {props.caption}
            ]
        }
    }
}

/// The square part of a [`BbCheckbox`]. Its parent is the checkbox.
#[derive(Component, Default, Clone)]
pub struct CheckboxBox;

/// The tick inside a [`CheckboxBox`].
#[derive(Component, Default, Clone)]
pub struct CheckboxMark;

/// Everything the look of a checkbox depends on.
type CheckboxState<'a> = (
    &'a Hovered,
    Has<Pressed>,
    Has<Checked>,
    Has<InteractionDisabled>,
);

/// The boxes, which [`checkbox_feedback`] repaints.
type Boxes<'w, 's> = Query<
    'w,
    's,
    (
        &'static ChildOf,
        &'static mut ThemedBackground,
        &'static mut ThemedBorder,
    ),
    (With<CheckboxBox>, Without<CheckboxMark>),
>;

/// The ticks, which [`checkbox_feedback`] repaints and hides.
type Marks<'w, 's> = Query<
    'w,
    's,
    (
        &'static ChildOf,
        &'static mut ThemedBorder,
        &'static mut Visibility,
    ),
    (With<CheckboxMark>, Without<CheckboxBox>),
>;

/// Chooses tokens for each box and tick from the state of the checkbox that
/// owns it, and shows the tick only when checked.
///
/// Like [`super::button::button_feedback`], this writes tokens and only when
/// they change; [`super::theme::paint`] turns them into colours.
pub fn checkbox_feedback(
    checkboxes: Query<CheckboxState, With<BbCheckbox>>,
    mut boxes: Boxes,
    mut marks: Marks,
) {
    for (checkbox, mut background, mut border) in &mut boxes {
        let Ok((hovered, pressed, checked, disabled)) = checkboxes.get(checkbox.parent()) else {
            continue;
        };
        let (bg, edge, _) = tokens_for(hovered.0, pressed, checked, disabled);

        if background.0 != bg {
            background.0 = bg;
        }
        if border.0 != edge {
            border.0 = edge;
        }
    }

    for (in_box, mut border, mut visibility) in &mut marks {
        let Some(state) = boxes
            .get(in_box.parent())
            .ok()
            .and_then(|(checkbox, ..)| checkboxes.get(checkbox.parent()).ok())
        else {
            continue;
        };
        let (hovered, pressed, checked, disabled) = state;
        let (_, _, tick) = tokens_for(hovered.0, pressed, checked, disabled);

        if border.0 != tick {
            border.0 = tick;
        }
        let shown = if checked {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != shown {
            *visibility = shown;
        }
    }
}

/// The state machine, as `(box background, box border, tick)`.
///
/// Disabled beats everything. Checked decides the fill; pressed and hover only
/// touch the border, so a hovered checked box still reads as checked.
fn tokens_for(
    hovered: bool,
    pressed: bool,
    checked: bool,
    disabled: bool,
) -> (Token, Token, Token) {
    if disabled {
        return (
            tokens::CHECKBOX_BG_DISABLED,
            tokens::CHECKBOX_BORDER_DISABLED,
            tokens::CHECKBOX_MARK_DISABLED,
        );
    }

    let bg = match (checked, hovered) {
        (true, _) => tokens::CHECKBOX_BG_CHECKED,
        (false, true) => tokens::CHECKBOX_BG_HOVER,
        (false, false) => tokens::CHECKBOX_BG,
    };
    let border = match (pressed, hovered, checked) {
        (true, _, _) => tokens::CHECKBOX_BORDER_PRESSED,
        (false, true, _) => tokens::CHECKBOX_BORDER_HOVER,
        (false, false, true) => tokens::CHECKBOX_BORDER_CHECKED,
        (false, false, false) => tokens::CHECKBOX_BORDER,
    };
    (bg, border, tokens::CHECKBOX_MARK)
}
