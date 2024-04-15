use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
  fn build(&self, app:&mut App){
    add.add_systems(Startup, spawn_camera);
  }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands:Commands){
  commands.spawn((
    Camera3DBundle{
      transform:Transform::from_xyz(0.0,0.0,3.0),
      ..Default::default()
    }
  ))
}