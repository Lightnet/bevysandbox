/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
*/
/*
  Informtion:
    For debug testing...
*/

//Tests
// https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs
use bevy::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
  let mut app = App::new();
    //.add_plugins(DefaultPlugins)
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        resolution: (WIDTH, HEIGHT).into(),
        title: "Bevy Game Test".to_string(),
        resizable: false,
        ..default()
      }),
      ..default()
    }))
    //.add_plugin(WorldInspectorPlugin::default())
    //.add_startup_system(setup)
    .add_systems(Startup, setup)
    .run();
}

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
  }).insert(Name::new("plane"));
  // cube
  commands.spawn(PbrBundle {
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  }).insert(Name::new("cube"));
  // light
  commands.spawn(PointLightBundle {
      point_light: PointLight {
          intensity: 1500.0,
          shadows_enabled: true,
          ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
  }).insert(Name::new("light"));
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}