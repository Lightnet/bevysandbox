#![allow(dead_code)]
#![allow(unused_variables)]
// https://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://github.com/mwbryant/bevy-tower-defense-tutorial/blob/part-7/src/main.rs
// https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
use bevy::prelude::*;
//use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_console::{AddConsoleCommand, ConsolePlugin};
use bevy::{winit::WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_mod_picking::*;
//use clap::Parser;

mod mod_console;
use mod_console::{LogCommand, log_command};

mod mod_ui;
use mod_ui::{setup_button, button_system};

//mod helloplugin;
//use helloplugin::{HelloPlugin};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState{
    MainMenu,
    Gameplay,
}

fn setup_console(app: &mut App){
    app.add_plugin(ConsolePlugin);
    app.add_console_command::<LogCommand, _>(log_command);
}

fn main() {
    let binspect:bool = false;
    let begui:bool = false;
    let bconsole:bool = true;
    println!("init bevy app!");

    let mut app = App::new();

    //app.add_plugins(DefaultPlugins);
    app.insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Bevy Game Test".to_string(),
            resizable: false,
            ..default()
        },
        ..default()
    }));
    app.add_state(GameState::MainMenu);
    //app.add_plugin(HelloPlugin);
    app.add_plugin(WorldInspectorPlugin);
    app.insert_resource(WinitSettings::desktop_app());

    //simple button
    app.add_startup_system(setup_button);
    app.add_system(button_system);

    if bconsole == false {
        //app.add_plugin(ConsolePlugin);
        //app.add_console_command::<LogCommand, _>(log_command);
        setup_console(&mut app);
    }
        //.add_system(ui_example)
    ;
    app.run();
}


// https://bevy-cheatbook.github.io/programming/ec.html

#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct PlayerXp(u32);

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}
//Friend and foe tag
/// Marker for hostile game units
#[derive(Component)]
struct Enemy;
/// This will be used to identify the main player entity
#[derive(Component)]
struct Player;

/// Tag all creatures that are currently friendly towards the player
#[derive(Component)]
struct Friendly;

#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    _p: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    // #[bundle]
    // sprite: SpriteSheetBundle,
}

// https://bevy-cheatbook.github.io/programming/commands.html
//commands.spawn(PlayerBundle {
    //name: PlayerName("Henry".into()),
    //xp: PlayerXp(1000),
    //health: Health {
        //hp: 100.0, extra: 20.0
    //},
    //_p: Player,
    //sprite: Default::default(),
//});

// END MAIN