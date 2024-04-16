
use bevy::{gltf::{self, Gltf}, prelude::*};
use clap::command;

use crate::asset_loader_plugin::MyAssets;

#[derive(Debug,Component)]
pub struct SceneName(String);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum SpawnScenesState{
  #[default]
  Spawning,
  Spawned,
  Done
}

pub fn spawn_scenes(
  mut commands: Commands,
  asset_pack:Res<MyAssets>,
  assets_gltf: Res<Assets<Gltf>>,
  mut next_state: ResMut<NextState<SpawnScenesState>>
){
  //Spawn Scenes
  for (name, gltf_handle) in &asset_pack.gltf_files{
    if let Some(gltf) = assets_gltf.get(gltf_handle){
     commands.spawn((
      SceneBundle{
      scene:gltf.named_scenes["Scene"].clone(),
      ..Default::default()
     },
     SceneName(name.clone())
    ));
    }
  }

  next_state.set(SpawnScenesState::Spawned);
}