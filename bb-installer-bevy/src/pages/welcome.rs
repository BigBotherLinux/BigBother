use crate::AppState;
use crate::cursor::CursorAtraction;
use crate::network::NetworkConnectivityLabel;
use bevy::feathers::controls::FeathersButton;
use bevy::feathers::theme::{ThemeBackgroundColor, ThemedText};
use bevy::feathers::tokens;
use bevy::input_focus::AutoFocus;
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;

pub struct WelcomePlugin;

impl Plugin for WelcomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Welcome), welcome_root.spawn());
    }
}

fn welcome_root() -> impl Scene {
    bsn! {
        Name::new("Welcome")
        DespawnOnExit::<AppState>(AppState::Welcome)
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[
            Text::new("BigBother Installer") ThemedText
            --
            Text::new("Network: Unknown") ThemedText NetworkConnectivityLabel
            --
            @FeathersButton {
                @caption: bsn! { Text::new("Install") ThemedText }
            }
            AutoFocus
            CursorAtraction
            on(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| { state.set(AppState::TermsOfService) })

        ]
    }
}
