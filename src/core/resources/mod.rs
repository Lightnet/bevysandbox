/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerInfo {
  pub name:&'static str,
  pub is_loaded:bool
}

impl Default for PlayerInfo {
  fn default() -> Self { 
    PlayerInfo{
      name:"Guest",
      is_loaded: false
    }
  }
}

