/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum GameState{
    #[default]
    MainMenu,
    Gameplay,
}

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct MainUIRoot;

#[derive(Component)]
pub struct GameUIRoot;