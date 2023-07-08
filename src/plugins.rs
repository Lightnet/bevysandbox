/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

// https://bevyengine.org/learn/book/getting-started/plugins/

use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here
    println!("Hello plugin...");
  }
}

pub struct BevySandboxPlugin;

impl Plugin for BevySandboxPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)));
    app.add_plugin(WorldInspectorPlugin::new());
  }
}