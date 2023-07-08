/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use crate::core::{components::*, ui::menu::main::styles::*};

pub fn interact_play_button(
  mut commands: Commands,
  menu_root: Query<Entity, With<MainUIRoot>>,
  mut game_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<
      (&Interaction, &mut BackgroundColor, &Children),
      (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
      //let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
        Interaction::Clicked => {
          println!("start CLICK...");
          //text.sections[0].value = "Press".to_string();
          *color = PRESSED_BUTTON.into();
          //let root_entity = menu_root.single();
          //commands.entity(root_entity).despawn_recursive();
          //game_state.set(GameState::GamePlay).unwrap();
          game_state.set(GameState::Gameplay);
        }
        Interaction::Hovered => {
          //text.sections[0].value = "Hover".to_string();
          *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
          //text.sections[0].value = "Button".to_string();
          *color = NORMAL_BUTTON.into();
        }
      }
    }
}

pub fn interact_button_back_main(
  mut commands: Commands,
  menu_root: Query<Entity, With<GameUIRoot>>,
  mut game_state: ResMut<NextState<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, &Children),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
    //let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Clicked => {
        println!("CLICK...");
        //text.sections[0].value = "Press".to_string();
        *color = PRESSED_BUTTON.into();
        //let root_entity = menu_root.single();
        //commands.entity(root_entity).despawn_recursive();
        //game_state.set(GameState::MainMenu).unwrap();
        game_state.set(GameState::MainMenu);
      }
      Interaction::Hovered => {
        //text.sections[0].value = "Hover".to_string();
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        //text.sections[0].value = "Button".to_string();
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}