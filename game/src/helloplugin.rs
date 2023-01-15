// https://bevyengine.org/learn/book/getting-started/plugins/

// app.add_plugin <- pass
// app.add_plugins <- fail, incorrect

use bevy::{
  app::{App, Plugin}
};


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here
    let world = &mut app.world;

    println!("Hello world plugin!");
  }
}

//#[derive(Debug, Clone, Eq, PartialEq, Hash)]