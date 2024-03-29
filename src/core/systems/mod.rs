/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use super::{components::CameraUIRoot, resources::PlayerInfo};

pub fn spawn_camera_3d(mut commands: Commands) {
  commands
    .spawn(Camera3dBundle {
      //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      transform: Transform::from_xyz(-10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    })
    .insert(CameraUIRoot);
    //.insert(RaycastPickCamera::default());
}

pub fn spawn_main_menu_camera_3d(mut commands: Commands) {
  commands
    .spawn((
      Camera3dBundle {
      //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      transform: Transform::from_xyz(-10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
      },
      CameraUIRoot
    ));
    //.insert(CameraUIRoot);
    //.insert(RaycastPickCamera::default());
}

pub fn system_query_info(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  //mut player_info: ResMut<PlayerInfo>,//write
  player_info: ResMut<PlayerInfo>,//read
){
  //if keyboard_input.pressed(KeyCode::A) {
    //println!("player_info: {:?}", player_info);
    //println!("player Name: {:}", player_info.name);
  //}

  //keyboard_input.get_just_released()

  if keyboard_input.just_released(KeyCode::KeyN) {
    println!("player_info: {:?}", player_info);
    println!("player Name: {:}", player_info.name);
  }

  if keyboard_input.just_pressed(KeyCode::KeyM) {
    println!("[[player_info]]: {:?}", player_info);
  }
}

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size:28.0,
    color: Color::WHITE,
  }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}