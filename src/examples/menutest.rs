/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */


//Tests

// https://bevyengine.org/learn/book/getting-started/ecs/
// https://crates.io/crates/bevy-inspector-egui
// https://github.com/mwbryant/bevy-tower-defense-tutorial/blob/part-7/src/main_menu.rs
// https://github.com/mwbryant/bevy-tower-defense-tutorial/blob/part-7/src/main_menu.rs

use bevy::{
  prelude::*, 
  winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use bevysandbox::core::{components::*, ui::menu::main::MainMenuPlugin};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

//#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
//pub enum GameState {
  //MainMenu,
  //GamePlay,
//}
//#[derive(Component)]
//pub struct MenuUIRoot;
//#[derive(Component)]
//pub struct StartButton;
//#[derive(Component)]
//pub struct QuitButton;
//#[derive(Component)]
//pub struct MainUIRoot;
//#[derive(Component)]
//pub struct GameUIRoot;

fn main() {
    let mut app = App::new();

    app
      .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
      //bevy basic setup
      .add_plugins(DefaultPlugins)
      // Inspector Setup
      .add_plugin(WorldInspectorPlugin::new())
      .add_state::<GameState>() // state app
      //for check game state for update and render
      //.add_state(GameState::MainMenu)
      //.add_state(GameState::GamePlay)
      //.insert_resource(WinitSettings::desktop_app())
      .add_plugin(MainMenuPlugin)
      //.add_startup_system(spawn_camera)
      //.add_system(setup_button01.in_schedule(OnEnter(GameState::MainMenu)))
      //.add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))
      //.add_system(setup_button02.in_schedule(OnEnter(GameState::MainMenu)))
      //.add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))
      //.add_system(button_system02.in_set(OnUpdate(GameState::Gameplay)))
      ;

    app.run();
}

//fn spawn_camera(mut commands: Commands) {
  //commands
    //.spawn(Camera3dBundle {
        //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        //..default()
    //})
    //.insert(RaycastPickCamera::default());
//}
