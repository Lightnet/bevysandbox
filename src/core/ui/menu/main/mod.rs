/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */
pub mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;

use crate::core::{ components::GameState};

use self::systems::{layout::*, interaction::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here
    //app.add_startup_system(spawn_camera_3d);
    //app.add_systems(Startup,spawn_camera_3d);

    //main menu
    app.add_systems( OnEnter(GameState::MainMenu),spawn_main_menu);
    app.add_systems(Update, (
      interact_play_button,
      interact_online_button,
      interact_settings_button,
      interact_quit_button
    ).run_if(in_state(GameState::MainMenu)) );
    app.add_systems( OnExit(GameState::MainMenu),despawn_main_menu);

    //game
    //app.add_system(spawn_game_menu.in_schedule(OnEnter(GameState::Gameplay)));
    //app.add_system(interact_button_back_main.in_set(OnUpdate(GameState::Gameplay)));
    //app.add_system(despawn_game_menu.in_schedule(OnExit(GameState::Gameplay)));

  }
}