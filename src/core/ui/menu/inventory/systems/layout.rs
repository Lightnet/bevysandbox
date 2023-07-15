/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

use crate::core::ui::menu::{inventory::{components::InventoryUIRoot, styles::get_icon_slot_style}, main::styles::NORMAL_BUTTON};

pub fn spawn_inventory_menu(
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
  })
  .insert(InventoryUIRoot)
  .with_children(|parent|{
    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|parent|{

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

    });

    parent.spawn(NodeBundle{
      style: Style {
        //width: Val::Percent(100.0),
        //height: Val::Percent(100.0),
        flex_direction:FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    }).with_children(|parent|{

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

      parent.spawn(ButtonBundle {
        style: get_icon_slot_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
      });

    });




  });

}

pub fn despawn_inventory_menu(
  mut commands: Commands,
  menu_query:Query<Entity, With<InventoryUIRoot>>,
){
  if let Ok(menu_entity) = menu_query.get_single(){
    commands.entity(menu_entity).despawn_recursive();
  }
}

pub fn build_inventory(){

}