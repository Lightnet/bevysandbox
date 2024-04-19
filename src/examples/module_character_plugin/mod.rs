/*
  cargo run --example char01
https://www.youtube.com/watch?v=jbYDljqf4kg&t=1s
  Time:1.34.47
*/
use bevy::prelude::*;
use crate::asset_loader_plugin::AssetLoaderState;
use self::{
  link_animations::link_animations,
  print_scene_tree::print_scene_tree_log,
  spawn_scenes::{spawn_scenes, SpawnScenesState},
  paint_cubes_on_joints::paint_cubes_on_joints,
  
};
pub mod spawn_scenes;
pub mod print_scene_tree;
pub mod paint_cubes_on_joints;
pub mod link_animations;


pub struct ModularCharacterPlugin;

impl Plugin for ModularCharacterPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<SpawnScenesState>()
      .add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes)
      .add_systems(
        OnEnter(SpawnScenesState::Spawned), 
        (
          link_animations,
          print_scene_tree_log, 
          paint_cubes_on_joints
        )
      );
  }
}