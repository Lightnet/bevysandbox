/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

// https://bevyengine.org/learn/book/getting-started/plugins/

use bevy::prelude::*;

use crate::core::{
  ui::menu::{
    main::MainMenuPlugin, 
    online::OnlineMenuPlugin, 
    settings::SettingsMenuPlugin
  }, 
  components::GameState, systems::{spawn_camera_3d, system_query_info}, resources::PlayerInfo, world::{WorldPrototypePlugin, WorldTest01Plugin}
};

//pub struct HelloPlugin;

//impl Plugin for HelloPlugin {
  //fn build(&self, app: &mut App) {
    // add things to your app here
    //println!("Hello plugin...");
  //}
//}

pub struct BevySandboxPlugin;

impl Plugin for BevySandboxPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)));
    //app.insert_resource(PlayerInfo::default());
    app.init_resource::<PlayerInfo>();

    //app.add_plugin(WorldInspectorPlugin::new());
    app.init_state::<GameState>();
    app.add_systems(Startup,spawn_camera_3d);
    app.add_plugins(MainMenuPlugin);
    //app.add_plugins(OnlineMenuPlugin);
    app.add_plugins(SettingsMenuPlugin);
    //app.add_plugins(WorldTest01Plugin);

    app.add_systems(Update,system_query_info);
  }
}

pub struct PrototypeSandboxPlugin;

impl Plugin for PrototypeSandboxPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)));
    //app.insert_resource(PlayerInfo::default());
    app.init_resource::<PlayerInfo>();
    //app.add_plugin(WorldInspectorPlugin::new());
    app.init_state::<GameState>();

    app.add_systems(Startup, set_state_gameplay);
    app.add_systems(Startup,spawn_camera_3d);


    app.add_plugins(WorldPrototypePlugin);
    
    app.add_systems(Update,system_query_info);
  }
}

fn set_state_gameplay(
  mut game_state: ResMut<NextState<GameState>>,
){
  game_state.set(GameState::Gameplay);
}