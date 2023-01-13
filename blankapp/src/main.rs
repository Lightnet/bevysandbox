//Tests

// https://bevyengine.org/learn/book/getting-started/ecs/

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

//fn hello_world() {
    //println!("hello world!");
//}

fn startup_message() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        //.add_system(startup_message)//loop
        .add_startup_system(startup_message)//loop
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
        if ui.button("Click Here!").clicked() {
            println!("Hello Click")
        }
    });
}