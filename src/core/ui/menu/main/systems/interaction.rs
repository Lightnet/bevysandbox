/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::core::{components::*, ui::menu::main::styles::*};

pub fn interact_play_button(
  mut game_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<StartButton>),
  >,
  //mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color) in &mut interaction_query {
    match *interaction {
      Interaction::Clicked => {
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

pub fn interact_quit_button(
  mut app_exit_event_writer:EventWriter<AppExit>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>),
  >,
) {
  for (interaction, mut color) in &mut interaction_query {
    match *interaction {
      Interaction::Clicked => {
        *color = PRESSED_BUTTON.into();
        app_exit_event_writer.send(AppExit);
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

pub fn interact_button_back_main(
  mut game_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color) in &mut interaction_query {
    match *interaction {
      Interaction::Clicked => {
        println!("CLICK...");
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