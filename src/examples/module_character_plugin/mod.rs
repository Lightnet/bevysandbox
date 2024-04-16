
use bevy::prelude::*;
use crate::asset_loader_plugin::AssetLoaderState;
use self::{print_scene_tree::print_scene_tree_log, spawn_scenes::{spawn_scenes, SpawnScenesState}};
mod spawn_scenes;
mod print_scene_tree;

pub struct ModularCharacterPlugin;

impl Plugin for ModularCharacterPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<SpawnScenesState>()
      .add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes)
      .add_systems(OnEnter(SpawnScenesState::Spawned), print_scene_tree_log);
  }
}