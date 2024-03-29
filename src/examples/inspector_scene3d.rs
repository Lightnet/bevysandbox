/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */


//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
//use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugins(EguiPlugin)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup ,setup)
    .run();
}

/// set up a simple 3D scene
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
    ..default()
  });
  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
  // light
  commands.spawn(DirectionalLightBundle {
    transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });

  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}