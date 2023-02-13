// 

use bevy::prelude::{Plugin, App};

pub fn test_lib(){
  println!("test");
}

// https://bevyengine.org/learn/book/getting-started/plugins/
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
      // add things to your app here
      println!("Hello plugin...");
  }
}