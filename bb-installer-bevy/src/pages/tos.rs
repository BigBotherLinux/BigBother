use crate::AppState;
use bevy::feathers::controls::FeathersButton;
use bevy::feathers::controls::FeathersScrollbar;
use bevy::feathers::display::label;
use bevy::feathers::theme::{ThemeBackgroundColor, ThemedText};
use bevy::feathers::tokens;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bevy::ui_widgets::{ControlOrientation, ScrollArea};

pub struct TosPlugin;

impl Plugin for TosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::TermsOfService),
            terms_of_service_root.spawn(),
        );
    }
}

const TOS_TEXT: &str = "This distribution is provided \"AS IS,\" with no warranty of any kind.

Installing will erase data or leave your system unbootable.

You install at your own risk, and the authors are not liable for any damage or data loss.

This distribution installs non-free proprietary software (such as firmware, drivers, and codecs) under its owners' license terms, which you agree to follow.

By selecting \"I Agree,\" you accept these risks and consent to installing non-free software.";

fn terms_of_service_root() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
        }
        DespawnOnExit::<AppState>(AppState::TermsOfService)
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[
            (Text::new("Terms of Service") ThemedText),
            // Outer frame: fixed size, with room on the right for the scrollbar.
            (
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect { right: Val::Px(10.0) },
                }
                Children[
                    (
                        #tos_text
                        Node {
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            overflow: Overflow::scroll_y(),
                        }
                        ScrollArea
                        Children[
                            label(TOS_TEXT),
                            (
                                @FeathersButton { @caption: bsn! { Text("I Agree") ThemedText} }
                                Node {width: percent(30), top: Val::Vh(5.0) }
                                on(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| {
                                    state.set(AppState::SecondaryTos);
                                })
                            )
                        ]
                    ),
                    @FeathersScrollbar {
                        @target: #tos_text,
                        @orientation: {ControlOrientation::Vertical}
                    }
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        width: Val::Px(6.0),
                    }
                ]
            ),
        ]
    }
}
