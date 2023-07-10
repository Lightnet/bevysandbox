/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

pub mod components;
pub mod systems;
pub mod styles;

use bevy::prelude::*;

use crate::core::components::GameState;

use self::systems::{layout::{spawn_online_menu, despawn_online_menu}, interaction::{interact_back_button, interact_test_button}};

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here

    //settings menu
    
    app.add_systems( OnEnter(GameState::Settings),spawn_online_menu);
    app.add_systems(Update, (
      interact_back_button,
      interact_test_button
    ).run_if(in_state(GameState::Settings)) );
    app.add_systems( OnExit(GameState::Settings),despawn_online_menu);
    
  }
}