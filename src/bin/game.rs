/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */
/*
  Information:
    Start Game Menu and other base game access default build.
*/

use bevy::prelude::*;
//use bevy_rapier3d::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevysandbox::{core::{physics::BsbPhysicsPlugin, world::{WorldPhysicsTest01Plugin, WorldTest01Plugin}}, plugins::BevySandboxPlugin};

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
		.add_plugins(BevySandboxPlugin)
		.run();
}
