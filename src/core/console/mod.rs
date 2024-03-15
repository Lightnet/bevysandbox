use bevy::prelude::*;
use bevy_console::{
  AddConsoleCommand, 
  ConsoleCommand, 
  ConsoleConfiguration, 
  ConsolePlugin
};
use clap::Parser;
pub struct SandboxConsolePlugin;

impl Plugin for SandboxConsolePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(ConsolePlugin);
		app.insert_resource(ConsoleConfiguration {
      // override config here
      ..Default::default()
    });
    app.add_console_command::<ExampleCommand, _>(example_command);
  }
}

// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct ExampleCommand {
  // Some message
  msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
  if let Some(Ok(ExampleCommand { msg })) = log.take() {
    // handle command
    println!("example params.");
  }
}
