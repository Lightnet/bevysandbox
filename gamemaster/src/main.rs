// this for admain game or create for game master to edit the world
#![allow(dead_code)]
#![allow(unused_variables)]
// https://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust

use bevy::prelude::*;
//use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_plugin(ConsolePlugin)
        .add_console_command::<LogCommand, _>(log_command)    
        //.add_system(ui_example)
        .run();
}
/*
fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
        if ui.button("Open file…").clicked() {
            println!("Hello Click")
        }
    });
}
*/
/// Prints given arguments to the console
#[derive(Parser, ConsoleCommand)]
#[command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}

fn log_command(mut log: ConsoleCommand<LogCommand>) {
    if let Some(Ok(LogCommand { msg, num })) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}