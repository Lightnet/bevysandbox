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

use self::systems::{layout::*, interaction::*};

pub struct OnlineMenuPlugin;

impl Plugin for OnlineMenuPlugin {
  fn build(&self, app: &mut App) {

    //online menu
    app.add_systems( OnEnter(GameState::Online),spawn_online_menu);
    app.add_systems(Update, (
      interact_host_button,
      interact_back_button,
      //interact_settings_button,
      //interact_quit_button
    ).run_if(in_state(GameState::Online)) );
    app.add_systems( OnExit(GameState::Online),despawn_online_menu);

  }
}