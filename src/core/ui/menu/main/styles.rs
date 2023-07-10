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

//not working 0.11.0
/*
pub const BUTTON_STYLE:Style = Style {
  //size: Size::new(Val::Px(150.0), Val::Px(65.0)),
  width: Val::Px(150.0),
  height: Val::Px(65.0),
  // center button
  margin: UiRect::all(Val::Auto),
  // horizontally center child text
  justify_content: JustifyContent::Center,
  // vertically center child text
  align_items: AlignItems::Center,
  ..Style::DEFAULT
};
*/