
/*
  cargo run --example char01

*/
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
mod camera_plugin;
use camera_plugin::CameraPlugin;

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
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(BillboardPlugin)
    .run();
}
