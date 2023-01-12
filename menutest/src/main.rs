//Tests

// https://bevyengine.org/learn/book/getting-started/ecs/

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

//fn hello_world() {
    //println!("hello world!");
//}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    GamePlay,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_plugin(EguiPlugin)
        .add_state(GameState::MainMenu)
        .add_system(ui_example)
        //.add_system(hello_world)//loop
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
        if ui.button("Open file…").clicked() {
            println!("Hello Click")
        }
    });
}


