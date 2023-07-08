/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */
pub mod systems;
pub mod styles;
use bevy::prelude::*;

use crate::core::{systems::spawn_camera_3d, components::GameState};

use self::systems::{layout::*, interaction::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here
    app.add_startup_system(spawn_camera_3d);

    //main menu
    app.add_system(spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)));
    app.add_system(interact_play_button.in_set(OnUpdate(GameState::MainMenu)));
    app.add_system(interact_quit_button.in_set(OnUpdate(GameState::MainMenu)));
    app.add_system(despawn_main_menu.in_schedule(OnExit(GameState::MainMenu)));

    //game
    app.add_system(spawn_game_menu.in_schedule(OnEnter(GameState::Gameplay)));
    app.add_system(interact_button_back_main.in_set(OnUpdate(GameState::Gameplay)));
    app.add_system(despawn_game_menu.in_schedule(OnExit(GameState::Gameplay)));

  }
}