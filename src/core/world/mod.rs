/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use super::{components::GameState, entity::player::{setup_physics_player, system_physics_player_input}, physics::setup_physics_ground, systems::spawn_camera_3d};


pub struct WorldPrototypePlugin;

impl Plugin for WorldPrototypePlugin {
  fn build(&self, app: &mut App) {

    //app.add_systems(Startup, spawn_camera_3d);
    app.add_systems(Startup, spawn_boxes);

    //app.add_startup_system(setup_physics_ground);
    //app.add_startup_system(setup_physics_player);
    //app.add_startup_system(spawn_boxes);


    //app.add_system(system_physics_player_input.in_set(OnUpdate(GameState::Gameplay)));
    //app.add_systems(Startup, system_physics_player_input);

  }
}

pub struct WorldTest01Plugin;

// https://bevy-cheatbook.github.io/programming/system-sets.html
impl Plugin for WorldTest01Plugin {
  fn build(&self, app: &mut App) {

    //app.add_systems(Startup, spawn_camera_3d);
    app.add_systems(OnEnter(GameState::Gameplay), spawn_boxes);

    app.add_systems(OnEnter(GameState::Gameplay), setup_physics_ground);
    app.add_systems(OnEnter(GameState::Gameplay),setup_physics_player);

    app.add_systems(Update, (system_physics_player_input).run_if(in_state(GameState::Gameplay)));
    //app.add_systems(Update, system_physics_player_input);

  }
}

// SIMPLE TEST
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
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
      ..default()
    }
  );
}
