/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

//simple IMGUI > EGUI

use bevy::prelude::*;
use bevy_egui::{egui, EguiPlugin, EguiContexts};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
    // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
    .add_systems(Update, ui_example)
    .run();
}

fn ui_example(mut egui_context: EguiContexts) {
  egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
    ui.label("world");
    if ui.button("Button Click").clicked() {
      println!("Hello Click");
    }
  });
}