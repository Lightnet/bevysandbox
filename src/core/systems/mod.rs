/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

pub fn spawn_camera_3d(mut commands: Commands) {
  commands
    .spawn(Camera3dBundle {
      //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      transform: Transform::from_xyz(-10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    });
    //.insert(RaycastPickCamera::default());
}