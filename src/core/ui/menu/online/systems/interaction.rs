/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use crate::core::{components::GameState, ui::menu::{online::components::{ButtonHost, ButtonBack}, main::styles::*}};

pub fn interact_host_button(
  mut game_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ButtonHost>),
  >,
  //mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color) in &mut interaction_query {
    match *interaction {
      Interaction::Pressed => {
        println!("start CLICK...");
        *color = PRESSED_BUTTON.into();
        game_state.set(GameState::Gameplay);
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}


pub fn interact_back_button(
  mut game_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ButtonBack>),
  >,
  //mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color) in &mut interaction_query {
    match *interaction {
      Interaction::Pressed => {
        println!("start CLICK...");
        *color = PRESSED_BUTTON.into();
        game_state.set(GameState::MainMenu);
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}