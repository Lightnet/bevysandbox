/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */
use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn get_button_style() -> Style {
  Style {
    width: Val::Px(180.0),
    //width: Val::Px(300.0),
    height: Val::Px(65.0),
    // center button
    //margin: UiRect::all(Val::Auto),
    margin: UiRect::all(Val::Px(8.0)),
    // horizontally center child text
    justify_content: JustifyContent::Center,
    // vertically center child text
    align_items: AlignItems::Center,
    ..default()
  }
}

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size: 40.0,
    color: Color::rgb(0.9, 0.9, 0.9),
    ..default()
  }
}