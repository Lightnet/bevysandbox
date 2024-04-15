
use bevy::{gltf::{self, Gltf}, prelude::*};
use clap::command;

use crate::asset_loader_plugin::MyAssets;

pub fn spawn_scenes(
  mut commands: Commands,
  asset_pack:Res<MyAssets>,
  assets_gltf: Res<Assets<Gltf>>
){
  //Spawn Scenes
  for (_name, gltf_handle) in &asset_pack.gltf_files{
    if let Some(gltf) = assets_gltf.get(gltf_handle){
     commands.spawn(SceneBundle{
      scene:gltf.named_scenes["Scene"].clone(),
      ..Default::default()
     });
    }
  }
}