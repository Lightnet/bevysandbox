/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
*/
/*
  Information: Testing game play.
*/

use bevy::prelude::*;
use bevysandbox::plugins::PrototypeSandboxPlugin;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
 
fn main() {
  println!("init app");
  App::new()
    //.insert_resource(ClearColor(Color::rgb(
      //0xF9 as f32 / 255.0,
      //0xF9 as f32 / 255.0,
      //0xFF as f32 / 255.0,
    //)))
    //.add_plugins(DefaultPlugins)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        resolution: (WIDTH, HEIGHT).into(),
        title: "Bevy Game Test".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }))
    //.add_plugin(WorldInspectorPlugin::new())
    //.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    //.add_plugin(RapierDebugRenderPlugin::default())
    .add_plugins(PrototypeSandboxPlugin)

    .run();
}
 
//fn setup_camera(mut commands: Commands) {
  //commands.spawn(Camera3dBundle {
    //transform: Transform::from_xyz(-30.0, 30.0, 100.0)
      //.looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
    //..Default::default()
  //}).insert(Name::new("Camera"));
//}
