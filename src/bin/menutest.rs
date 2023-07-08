/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */


//Tests

// https://bevyengine.org/learn/book/getting-started/ecs/
// https://crates.io/crates/bevy-inspector-egui
// https://github.com/mwbryant/bevy-tower-defense-tutorial/blob/part-7/src/main_menu.rs
// https://github.com/mwbryant/bevy-tower-defense-tutorial/blob/part-7/src/main_menu.rs

use bevy::{
  prelude::*, 
  winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use bevysandbox::core::components::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

//#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
//pub enum GameState {
  //MainMenu,
  //GamePlay,
//}
//#[derive(Component)]
//pub struct MenuUIRoot;
//#[derive(Component)]
//pub struct StartButton;
//#[derive(Component)]
//pub struct QuitButton;
//#[derive(Component)]
//pub struct MainUIRoot;
//#[derive(Component)]
//pub struct GameUIRoot;

fn main() {
    let mut app = App::new();

    app
      .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
      //bevy basic setup
      .add_plugins(DefaultPlugins)
      // Inspector Setup
      .add_plugin(WorldInspectorPlugin::new())
      .add_state::<GameState>() // state app

      //for check game state for update and render
      //.add_state(GameState::MainMenu)
      //.add_state(GameState::GamePlay)

      //.insert_resource(WinitSettings::desktop_app())

      .add_startup_system(spawn_camera)

      .add_system(setup_button01.in_schedule(OnEnter(GameState::MainMenu)))
      .add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))

      .add_system(setup_button02.in_schedule(OnEnter(GameState::MainMenu)))
      .add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))

      .add_system(button_system02.in_set(OnUpdate(GameState::Gameplay)))
      ;

    app.run();
}

fn spawn_camera(mut commands: Commands) {
  commands
    .spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(RaycastPickCamera::default());
}

fn setup_button01(mut commands: Commands, asset_server: Res<AssetServer>) {
  // ui camera
  //commands.spawn(Camera2dBundle::default());//need camera to see the UI button
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
    }).insert(MainUIRoot)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Main Button",
        TextStyle {
          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
          font_size: 40.0,
          color: Color::rgb(0.9, 0.9, 0.9),
        },
      ));
    });
}

fn setup_button02(mut commands: Commands, asset_server: Res<AssetServer>) {
  // ui camera
  //commands.spawn(Camera2dBundle::default());//need camera to see the UI button
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
    }).insert(GameUIRoot)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Game",
        TextStyle {
          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
          font_size: 40.0,
          color: Color::rgb(0.9, 0.9, 0.9),
        },
      ));
    });
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
  mut commands: Commands,
  menu_root: Query<Entity, With<MainUIRoot>>,
  mut game_state: ResMut<State<GameState>>,
  mut interaction_query: Query<
      (&Interaction, &mut BackgroundColor, &Children),
      (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
      //let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
        Interaction::Clicked => {
          println!("start CLICK...");
          //text.sections[0].value = "Press".to_string();
          *color = PRESSED_BUTTON.into();
          let root_entity = menu_root.single();
          commands.entity(root_entity).despawn_recursive();

          
          //game_state.set(GameState::GamePlay).unwrap();
        }
        Interaction::Hovered => {
          //text.sections[0].value = "Hover".to_string();
          *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
          //text.sections[0].value = "Button".to_string();
          *color = NORMAL_BUTTON.into();
        }
      }
    }
}

fn button_system02(
  mut commands: Commands,
  menu_root: Query<Entity, With<GameUIRoot>>,
  mut game_state: ResMut<State<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, &Children),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
    //let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Clicked => {
        println!("CLICK...");
        //text.sections[0].value = "Press".to_string();
        *color = PRESSED_BUTTON.into();
        let root_entity = menu_root.single();
        commands.entity(root_entity).despawn_recursive();
        //game_state.set(GameState::MainMenu).unwrap();
      }
      Interaction::Hovered => {
        //text.sections[0].value = "Hover".to_string();
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        //text.sections[0].value = "Button".to_string();
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}

