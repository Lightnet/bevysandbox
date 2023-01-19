//Tests

// https://bevyengine.org/learn/book/getting-started/ecs/
// https://crates.io/crates/bevy-inspector-egui
// https://github.com/mwbryant/bevy-tower-defense-tutorial/blob/part-7/src/main_menu.rs

use bevy::{prelude::*, winit::WinitSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;


pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    GamePlay,
}

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;



fn main() {
    let mut app = App::new();
    app
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        //bevy basic setup
        .add_plugins(DefaultPlugins)
        // Inspector Setup
        .add_plugin(WorldInspectorPlugin)

        //for check game state for update and render
        .add_state(GameState::MainMenu)
        //.add_state(GameState::GamePlay)

        .insert_resource(WinitSettings::desktop_app())
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_button01))

        //.add_startup_system(setup_button)
        //.add_system(button_system)
        //.add_system_set(
            //SystemSet::on_enter(GameState::MainMenu)
            //SystemSet::on_update(GameState::MainMenu)
                //.with_system(spawn_main_menu)
                //.with_system(menu_system)
        //)
        //.add_system_set(
            //SystemSet::on_update(GameState::GamePlay)
                //.with_system(game_system)
        //)

        ;

    app.run();
}

fn menu_system(){
    //println!("menu");
}

fn game_system(){
    //println!("game");
}


fn setup_button01(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());//need camera to see the UI button
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}




fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = spawn_button(&mut commands, &asset_server, "Start Game", Color::RED);
    commands.entity(start_button).insert(StartButton);

    let quit_button = spawn_button(&mut commands, &asset_server, "Quit", Color::BLUE);
    commands.entity(quit_button).insert(QuitButton);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuUIRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Tower Defense Tutorial",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 96.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .add_child(start_button)
        .add_child(quit_button);
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(65.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .id()
}



const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                println!("CLICK...");
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}


fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PickingCameraBundle::default());
}