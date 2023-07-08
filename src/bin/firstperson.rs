/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

//Tests

// https://bevyengine.org/learn/book/getting-started/ecs/

use bevy::{prelude::*, input::mouse::MouseMotion, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::PlayerPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
  let mut app = App::new();
    //.add_plugins(DefaultPlugins)
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        //width: WIDTH,
        //height: HEIGHT,
        title: "Bevy Game Test".to_string(),
        present_mode: PresentMode::AutoNoVsync,
        resizable: false,
        ..default()
      }),
      ..default()
    }))
    .add_plugin(PlayerPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(setup)
    //.add_system(player_camera_controller)
    .run();
}

fn setup_camera(
  mut commands: Commands,
){
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-20.0, 20.5, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(100.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  }).insert(Name::new("plane"));
  // cube
  //commands.spawn(PbrBundle {
      //mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      //material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
      //transform: Transform::from_xyz(0.0, 0.5, 0.0),
      //..default()
  //}).insert(Name::new("cube"));

  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size:32.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
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
  //commands.spawn(Camera3dBundle {
    //transform: Transform::from_xyz(-20.0, 20.5, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    //..default()
  //});
}

/*
fn player_camera_controller(
  mut mouse_motion: EventReader<MouseMotion>,
  //windows: Res<Windows>,
  mut query: Query<(&mut Transform), With<Camera>>,
){
  //let window = get_primary_window_size(&windows);
  let mut camera_transform = query.single_mut();
  let mut yaw:f32 = 0.;
  let mut pitch:f32 = 0.;
  let mut TAU:f32 = 1.0;

  for ev in mouse_motion.iter() { //rewrite
    //camera_transform.rotate_y(ev.delta.x * TAU * -0.001);
    //camera_transform.rotate_x(ev.delta.y * TAU * -0.001);
    //camera_transform.rotate_z(0.0);

    camera_transform
      .rotate(Quat::from_euler(EulerRot::XYZ,
        ev.delta.y * -0.001, ev.delta.x * -0.001, 0.));
  }

}
*/