use bevy::{
    color::palettes::basic::*,
    input_focus::tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
    picking::hover::Hovered,
    prelude::*,
    reflect::Is,
    ui::{Checked, InteractionDisabled, Pressed},
    ui_widgets::{
        checkbox_self_update, observe, Activate, Button, Checkbox, Slider, SliderRange,
        SliderThumb, SliderValue, ValueChange,
    },
};
use bevy::color::palettes::css::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn button(text: &str) -> impl Bundle {
    (
        Node {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::all(Val::Percent(10.0)),

            ..default()
        },
        Button,
        Interaction::default(),
        BorderColor::all(Color::WHITE),
        BackgroundColor(NORMAL_BUTTON),
        Hovered::default(),
        children![(
            Text::new(text),
            TextColor(Color::WHITE),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
        )],
    )
}

pub fn button_system(
    mut interactions: Query<
      (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
      ),
      (Changed<Interaction>, With<Button>),
    >,
    mut texts: Query<&mut Text>,
  ) {
    for (interaction, mut color, mut border_color, children) in &mut interactions
    {
      if let Ok(mut text) = texts.get_mut(children[0]) {
        match *interaction {
          Interaction::Pressed => {
            text.0 = "Press".to_string();
            *color = PRESSED_BUTTON.into();
            border_color.set_all(BLUE);
          }
          Interaction::Hovered => {
            text.0 = "Hover".to_string();
            *color = HOVERED_BUTTON.into();
            border_color.set_all(WHITE);
          }
          Interaction::None => {
            text.0 = "Button".to_string();
            *color = NORMAL_BUTTON.into();
            border_color.set_all(BLACK);
          }
        }
      }
    }
  }