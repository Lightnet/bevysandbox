/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */


use bevy::prelude::*;

use super::{
  systems::spawn_camera_3d, 
  physics::setup_physics_ground, 
  entity::player::{setup_physics_player}, 
  components::GameState
};

pub struct WorldTest01Plugin;

impl Plugin for WorldTest01Plugin {
  fn build(&self, app: &mut App) {

    app.add_systems(Startup, spawn_camera_3d);
    app.add_systems(Startup, spawn_boxes);

    //app.add_startup_system(setup_physics_ground);
    //app.add_startup_system(setup_physics_player);
    //app.add_startup_system(spawn_boxes);


    //app.add_system(system_physics_player_input.in_set(OnUpdate(GameState::Gameplay)));
    //app.add_systems(Startup, system_physics_player_input);

  }
}

pub struct WorldPhysicsTest01Plugin;

impl Plugin for WorldPhysicsTest01Plugin {
  fn build(&self, app: &mut App) {

    app.add_systems(Startup, spawn_camera_3d);
    app.add_systems(Startup, spawn_boxes);

    //app.add_startup_system(setup_physics_ground);
    //app.add_startup_system(setup_physics_player);
    //app.add_startup_system(spawn_boxes);


    //app.add_system(system_physics_player_input.in_set(OnUpdate(GameState::Gameplay)));
    //app.add_systems(Startup, system_physics_player_input);

  }
}

fn spawn_boxes(
  mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
){
  commands.spawn(
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..default()
    }
  );
}