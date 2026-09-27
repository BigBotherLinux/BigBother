//! An integer slider that owns its own fill bar and readout.
//!
//! The old version scattered this across three entities the caller had to wire
//! up by hand: a track, a fill whose parent happened to be the track, and a
//! readout holding the track's [`Entity`]. Here the scene owns all three, so a
//! caller spawns one thing and queries [`SliderValue`] on the entity it got
//! back.
//!
//! Deliberately *thumbless*. [`bevy::ui_widgets`] shortens a slider's travel by
//! the width of whatever is marked as its thumb, so with no thumb the pointer
//! maths is simply "fraction across the track", and the fill can be drawn at
//! exactly that fraction with no half-thumb fudging.

use bevy::picking::hover::Hovered;
use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ui::px;
use bevy::ui_widgets::{
    slider_self_update, Slider, SliderOrientation, SliderPrecision, SliderRange, SliderStep,
    SliderValue, TrackClick,
};

use super::text::{styles, TextStyle};
use super::theme::{tokens, ThemedBackground, ThemedBorder, ThemedText};

/// Height of the track, which also sets its corner radius.
const TRACK_HEIGHT: f32 = 28.0;

/// An integer track, `min..=max`, with the value written on it.
///
/// # Emits
/// [`ValueChange<f32>`](bevy::ui_widgets::ValueChange) while dragging and on
/// release. The stock [`slider_self_update`] observer is attached, so
/// [`SliderValue`] on this entity is always current and a caller can simply
/// read it.
#[derive(SceneComponent, Default, Clone)]
#[scene(SliderProps)]
pub struct BbSlider;

/// Construction options for [`BbSlider`].
pub struct SliderProps {
    pub min: i32,
    pub max: i32,
    /// Where the fill sits before anyone touches it.
    pub start: i32,
    pub width: Val,
    /// Draw `value / max` on the track. Off for a slider whose meaning is
    /// obvious from what it controls.
    pub show_value: bool,
}

impl Default for SliderProps {
    fn default() -> Self {
        Self {
            min: 0,
            max: 10,
            start: 5,
            width: px(520),
            show_value: true,
        }
    }
}

impl BbSlider {
    fn scene(props: SliderProps) -> impl Scene {
        debug_assert!(
            props.min < props.max,
            "slider range {}..={} is empty",
            props.min,
            props.max
        );
        let start = props.start.clamp(props.min, props.max);
        let max = props.max;
        let show_value = props.show_value;

        bsn! {
            Node {
                width: {props.width},
                height: {px(TRACK_HEIGHT)},
                border: {UiRect::all(px(2))},
                border_radius: {BorderRadius::all(px(TRACK_HEIGHT / 2.0))},
                // Keeps the square-cornered fill inside the rounded track.
                overflow: {Overflow::clip()},
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            BbSlider
            Slider {
                // Snap, not Drag: clicking "8" should mean 8, not "start
                // dragging from 8", because people click these once and move on.
                track_click: TrackClick::Snap,
                orientation: SliderOrientation::Horizontal,
            }
            SliderValue({start as f32})
            template_value(SliderRange::new(props.min as f32, max as f32))
            SliderStep({1.0_f32})
            // Whole numbers only, including mid-drag.
            SliderPrecision({0})
            Hovered
            BackgroundColor
            BorderColor
            ThemedBackground({tokens::SLIDER_TRACK})
            ThemedBorder({tokens::SLIDER_BORDER})
            // The core slider only reports changes; writing them back is the
            // app's job, and this is the stock observer that does it.
            on(slider_self_update)
            Children [
                (
                    TrackFill
                    Node {
                        position_type: PositionType::Absolute,
                        left: {px(0)},
                        top: {px(0)},
                        bottom: {px(0)},
                        width: {Val::Percent(0.0)},
                    }
                    BackgroundColor
                    ThemedBackground({tokens::SLIDER_FILL})
                    // Never swallow a click meant for the track underneath.
                    template_value(Pickable::IGNORE)
                ),
                {
                    if show_value {
                        readout(max, styles::READOUT)
                    } else {
                        Box::new(bsn_list!()) as Box<dyn SceneList>
                    }
                }
            ]
        }
    }
}

/// The number drawn over the fill.
fn readout(max: i32, style: TextStyle) -> Box<dyn SceneList> {
    Box::new(bsn_list!((
        TrackReadout { max: { max } }
        Text
        TextFont {
            font_size: {FontSize::Px(style.size)},
        }
        TextColor
        ThemedText({style.color})
        template_value(Pickable::IGNORE)
    )))
}

/// The coloured part of a track. Its parent is the slider, which is how
/// [`slider_feedback`] finds the value to draw.
#[derive(Component, Default, Clone)]
pub struct TrackFill;

/// The number printed on a track. Carries `max` so the readout can say
/// "7 / 10" without asking the range twice.
#[derive(Component, Default, Clone)]
pub struct TrackReadout {
    pub max: i32,
}

/// Redraws every track's fill and readout from the slider's own value, so
/// nothing has to cache the number anywhere.
pub fn slider_feedback(
    sliders: Query<(&SliderValue, &SliderRange), With<BbSlider>>,
    mut fills: Query<(&ChildOf, &mut Node), With<TrackFill>>,
    mut readouts: Query<(&ChildOf, &TrackReadout, &mut Text)>,
) {
    for (track, mut node) in &mut fills {
        if let Ok((value, range)) = sliders.get(track.parent()) {
            node.width = Val::Percent(range.thumb_position(value.0) * 100.0);
        }
    }
    for (track, readout, mut text) in &mut readouts {
        if let Ok((value, _)) = sliders.get(track.parent()) {
            let shown = format!("{} / {}", value.0.round() as i32, readout.max);
            if **text != shown {
                **text = shown;
            }
        }
    }
}
