#![allow(dead_code)]
#![allow(unused_variables)]
// https://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust

// https://github.com/mwbryant/bevy-tower-defense-tutorial/blob/part-7/src/main.rs

use bevy::prelude::*;
//use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_console::{AddConsoleCommand, ConsolePlugin};
use bevy::{winit::WinitSettings};
use bevy_mod_picking::*;

//use clap::Parser;
// https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
mod mod_console;
use mod_console::{LogCommand, log_command};

mod mod_ui;
use mod_ui::{setup, button_system};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState{
    MainMenu,
    Gameplay,
}

fn main() {
    let binspect:bool = false;
    let begui:bool = false;
    let bconsole:bool = true;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.insert_resource(WinitSettings::desktop_app());
    app.add_state(GameState::MainMenu);

    // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
    // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
    //.add_plugin(EguiPlugin)

    //simple button
    app.add_startup_system(setup);
    app.add_system(button_system);


    if bconsole == true {
        app.add_plugin(ConsolePlugin);
        app.add_console_command::<LogCommand, _>(log_command);
    }
        //.add_system(ui_example)
    ;
    app.run();
}



// END MAIN