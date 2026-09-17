use bevy::prelude::*;
use bb_installer_bevy::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Map),
            spawn_map,
        );
    }
}

fn spawn_map(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Map),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },
        // No BackgroundColor here: UI draws over the 2D world, so a filled
        // root node would hide the eye. The camera clears to BACKGROUND.
        children![
            (
                Text::new("BigBother"),
                TextFont {
                    font_size: FontSize::Px(72.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.1, 0.1)),
            ),
            (
                Text::new("Trigger warning!"),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.85)),
            ),
        ],
    ));
}