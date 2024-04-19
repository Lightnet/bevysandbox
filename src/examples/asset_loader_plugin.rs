
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum AssetLoaderState{
  #[default]
  Loading,
  Done
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut App) {
    app.init_state::<AssetLoaderState>()
      .add_loading_state(LoadingState::new(
        AssetLoaderState::Loading
      ).continue_to_state(AssetLoaderState::Done)
      .load_collection::<MyAssets>()
    );
  }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets{
  //
  #[asset(paths("models/main_skeleton.glb","models/male_toon_block01_chest01.glb"),collection(typed, mapped))]
  //#[asset(paths("models/male_toon_block01.glb"),collection(typed, mapped))]
  pub gltf_files:HashMap<String, Handle<Gltf>>,
  #[asset(paths("fonts/FiraSans-Bold.ttf"),collection(typed, mapped))]
  pub font_files:HashMap<String, Handle<Font>>
}