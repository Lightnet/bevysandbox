/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

//Tests

use bevy::{
  prelude::*, 
  winit::WinitSettings,
};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_mod_picking::prelude::*;
use bevysandbox::core::{components::*, ui::menu::{main::MainMenuPlugin, inventory::InventoryMenuPlugin}};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
  let mut app = App::new();

  app
    .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
    //bevy basic setup
    .add_plugins(DefaultPlugins)
    // 
    .add_state::<GameState>() // state app
    //.state_set(GameState::)
    .add_systems(Startup, spawn_camera)
    .add_plugins(InventoryMenuPlugin);

  app.run();
}

fn spawn_camera(
  mut commands: Commands
) {
  commands
    .spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    //.insert(RaycastPickCamera::default());
}
