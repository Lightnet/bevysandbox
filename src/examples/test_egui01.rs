/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

//simple IMGUI > EGUI
// https://github.com/bevyengine/bevy/blob/main/examples/ecs/event.rs

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_egui::{egui, EguiPlugin, EguiContexts};

// A group of related system sets, used for controlling the order of systems. Systems can be
// added to any number of sets.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MySet {
    BeforeRound,
    Round,
    AfterRound,
}

#[derive(Event)]
struct MyEvent {
    pub message: String,
}

#[derive(Resource)]
struct EventTriggerState {
    event_timer: Timer,
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs(5)))
    .configure_sets(
      Update,
      // chain() will ensure sets run in the order they are listed
      (MySet::BeforeRound, MySet::Round, MySet::AfterRound).chain(),
    )
    .add_event::<MyEvent>()
    // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
    // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
    .add_systems(Update, ui_example)
    .add_systems(Update, keyboard_system)
    //.add_systems(PreUpdate, (exclusive_event_system).in_set(MySet::BeforeRound) )
    .add_systems(
      Update,
      (
        //(
          //(new_round_system, new_player_system).chain(),
          //exclusive_event_system,
        //).in_set(MySet::BeforeRound),
        //exclusive_event_system.in_set(MySet::BeforeRound),
        exclusive_events_system.in_set(MySet::BeforeRound),
        score_system.in_set(MySet::Round),
        (
          score_check_system,
          // In addition to chain(), you can also use `before(system)` and `after(system)`. This also works
          // with sets!
          game_over_system.after(score_check_system),
        )
          .in_set(MySet::AfterRound),
      )
    )
    .run();
}

fn new_round_system() {

}

fn new_player_system() {

}

fn score_system() {
  println!("MySet::Round");
}
fn score_check_system() {

}

fn game_over_system() {
  println!("MySet::AfterRound");
}

fn ui_example(
  mut egui_context: EguiContexts,
  mut my_events: EventWriter<MyEvent>,
) {
  egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
    ui.label("world");
    if ui.button("Button Click").clicked() {
      println!("Hello Click");
      my_events.send(MyEvent {
        message: "MyEvent just happened!".to_string(),
      });
    }
  });
}
// https://github.com/bevyengine/bevy/blob/main/examples/input/keyboard_input.rs
fn keyboard_system(
  keyboard_input:  Res<ButtonInput<KeyCode>>,
  mut my_events: EventWriter<MyEvent>,
){
  if keyboard_input.pressed(KeyCode::KeyA) {
    info!("'A' currently pressed");
  }
}

#[allow(dead_code)]
fn exclusive_event_system(
  //world: &mut World,
  mut events: EventReader<MyEvent>
){
  info!("test");
  for my_event in events.read() {
    info!("{}", my_event.message);
  }
}

fn exclusive_events_system(
  world: &mut World,
){
  println!("MySet::BeforeRound");
  println!("{:?}", world);
}

fn exclusive_world_system(world: &mut World){//not safe and can't add more params
  println!("MySet::BeforeRound");
  println!("{:?}", world);
}


