
use bevy::{gltf::{self, Gltf}, prelude::*, utils::hashbrown::HashMap};
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

#[derive(Resource, Debug)]
pub struct Animations(HashMap<String, Handle<AnimationClip>>);

pub fn spawn_scenes(
  mut commands: Commands,
  asset_pack:Res<MyAssets>,
  assets_gltf: Res<Assets<Gltf>>,
  mut next_state: ResMut<NextState<SpawnScenesState>>
){
  let mut animations = HashMap::new();
  let mut x = 0.0;
  //Spawn Scenes
  for (name, gltf_handle) in &asset_pack.gltf_files{
    if let Some(gltf) = assets_gltf.get(gltf_handle){
        commands.spawn((
        SceneBundle{
        scene:gltf.named_scenes["Scene"].clone(),
        transform:Transform::from_xyz(x, 0.0, 0.0),
        ..Default::default()
        },
        SceneName(name.clone())
      ));
      for named_animation in gltf.named_animations.iter(){
        println!("inserting animation: {}",named_animation.0);
        animations.insert(
          named_animation.0.clone(),
          gltf.named_animations[named_animation.0].clone()
        );

      }
    }
    //x+=1.0;
    
  }
  commands.insert_resource(Animations(animations));

  next_state.set(SpawnScenesState::Spawned);
}