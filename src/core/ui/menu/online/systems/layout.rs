/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use crate::core::ui::menu::{main::styles::NORMAL_BUTTON, online::components::*};
 
pub fn spawn_online_menu(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  println!("init menu...");
  commands.spawn(NodeBundle{
    style: Style {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      flex_direction:FlexDirection::Column,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      ..default()
    },
    ..default()
  }).with_children(|parent|{

    let button_style:Style = Style {
      //size: Size::new(Val::Px(150.0), Val::Px(65.0)),
      width: Val::Px(180.0),
      height: Val::Px(65.0),
      // center button
      //margin: UiRect::all(Val::Auto),
      margin: UiRect::all(Val::Px(8.0)),
      // horizontally center child text
      justify_content: JustifyContent::Center,
      // vertically center child text
      align_items: AlignItems::Center,
      ..default()
    };

    let text_style:TextStyle = TextStyle {
      //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
      font_size: 40.0,
      color: Color::rgb(0.9, 0.9, 0.9),
      ..default()
    };

    // Host BUTTON
    parent.spawn(ButtonBundle {
      style: button_style.clone(),
      background_color: NORMAL_BUTTON.into(),
      ..default()
    }).insert(ButtonHost)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Host",
        TextStyle {
          //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
          font_size: 40.0,
          color: Color::rgb(0.9, 0.9, 0.9),
          ..default()
        },
      ));
    });

    // Join BUTTON
    parent.spawn(ButtonBundle {
      style: button_style.clone(),
      background_color: NORMAL_BUTTON.into(),
      ..default()
    }).insert(ButtonJoin)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Join",
        text_style.clone(),
      ));
    });

    // SETTINGS BUTTON
    parent.spawn(ButtonBundle {
      style: button_style.clone(),
      background_color: NORMAL_BUTTON.into(),
      ..default()
    }).insert(ButtonNetworkConfig)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Settings",
        text_style.clone(),
      ));
    });

    // QUIT BUTTON
    parent.spawn(ButtonBundle {
      style: button_style.clone(),
      background_color: NORMAL_BUTTON.into(),
      ..default()
    }).insert(ButtonBack)
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Back",
        text_style.clone(),
      ));
    });


  }).insert(OnlineUIRoot);
}

pub fn despawn_online_menu(
  mut commands: Commands,
  menu_query:Query<Entity, With<OnlineUIRoot>>,
){
  if let Ok(menu_entity) = menu_query.get_single(){
    commands.entity(menu_entity).despawn_recursive();
  }
}