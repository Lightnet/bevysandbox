/*
  cargo run --example char01
*/
// time 32.20

use asset_loader_plugin::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
mod camera_plugin;
use camera_plugin::CameraPlugin;
use module_character_plugin::ModularCharacterPlugin;
mod asset_loader_plugin;
mod module_character_plugin;

fn main(){
  App::new()
    //BEVY
    .insert_resource(ClearColor(Color::rgb(0.1, 0.15, 0.15)))
    .insert_resource(AmbientLight{
      color:Color::default(),
      brightness:1000.0
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(CameraPlugin)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(ModularCharacterPlugin)
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(BillboardPlugin)
    .run();
}
