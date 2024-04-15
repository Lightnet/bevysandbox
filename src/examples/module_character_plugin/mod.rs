
use bevy::prelude::*;

use crate::asset_loader_plugin::AssetLoaderState;

use self::spawn_scenes::spawn_scenes;
mod spawn_scenes;


pub struct ModularCharacterPlugin;

impl Plugin for ModularCharacterPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes);
  }
}