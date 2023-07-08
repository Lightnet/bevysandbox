/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use crate::core::{ui::menu::main::styles::*, components::*};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
  
  commands.spawn(NodeBundle{
    ..default()
  }).with_children(|parent|{

    // PLAY BUTTON
    parent.spawn(ButtonBundle {
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
    }).insert(StartButton)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Play",
        TextStyle {
          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
          font_size: 40.0,
          color: Color::rgb(0.9, 0.9, 0.9),
        },
      ));
    });

    // QUIT BUTTON
    parent.spawn(ButtonBundle {
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
    }).insert(QuitButton)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Quit",
        TextStyle {
          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
          font_size: 40.0,
          color: Color::rgb(0.9, 0.9, 0.9),
        },
      ));
    });


  }).insert(MainUIRoot);
}

pub fn despawn_main_menu(
  mut commands: Commands,
  main_menu_query:Query<Entity, With<MainUIRoot>>,
){
  if let Ok(main_menu_entity) = main_menu_query.get_single(){
    commands.entity(main_menu_entity).despawn_recursive();
  }
}

pub fn spawn_game_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    })
    .insert(GameUIRoot)
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

pub fn despawn_game_menu(
  mut commands: Commands,
  main_menu_query:Query<Entity, With<GameUIRoot>>,
){
  if let Ok(main_menu_entity) = main_menu_query.get_single(){
    commands.entity(main_menu_entity).despawn_recursive();
  }
}