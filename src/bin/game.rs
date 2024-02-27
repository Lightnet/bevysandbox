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
use bevysandbox::plugins::BevySandboxPlugin;
//use bevy_rapier3d::prelude::*;

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
    //.add_systems(Startup, setup)
		.run();
}

// set up a simple 3D scene
/*
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // circular base
  commands.spawn(PbrBundle {
      mesh: meshes.add(Circle::new(4.0)),
      material: materials.add(Color::WHITE),
      transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
      ..default()
  });
  // cube
  commands.spawn(PbrBundle {
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::rgb_u8(124, 144, 255)),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  });
  // light
  commands.spawn(PointLightBundle {
      point_light: PointLight {
          shadows_enabled: true,
          ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
  });
  // camera
  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
  });
}
*/