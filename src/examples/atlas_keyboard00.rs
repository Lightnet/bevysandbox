// https://github.com/bevyengine/bevy/blob/release-0.13.1/examples/2d/texture_atlas.rs

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

#[derive(Component)]
struct KEY0;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
    .init_state::<AppState>()
    .add_systems(OnEnter(AppState::Setup), load_textures)
    .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
    .add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)))
    .add_systems(OnEnter(AppState::Finished), setup)
    .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
  #[default]
  Setup,
  Finished,
}

#[derive(Resource, Default)]
struct KeyboardSpriteFolder(Handle<LoadedFolder>);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    //commands.insert_resource(KeyboardSpriteFolder(asset_server.load_folder("kenney_input-prompts/Keyboard & Mouse/Default")));
    commands.insert_resource(KeyboardSpriteFolder(asset_server.load_folder("kenney_input-prompts/Keyboard & Mouse")));
}

fn check_textures(
  mut next_state: ResMut<NextState<AppState>>,
  rpg_sprite_folder: Res<KeyboardSpriteFolder>,
  mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
  // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
  for event in events.read() {
      if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
          next_state.set(AppState::Finished);
      }
  }
}

fn setup(
  mut commands: Commands,
  rpg_sprite_handles: Res<KeyboardSpriteFolder>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
  loaded_folders: Res<Assets<LoadedFolder>>,
  mut textures: ResMut<Assets<Image>>,
) {
  let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();

  // create texture atlases with different padding and sampling

  let (texture_atlas_linear, linear_texture) = create_texture_atlas(
      loaded_folder,
      None,
      Some(ImageSampler::linear()),
      &mut textures,
  );
  let atlas_linear_handle = texture_atlases.add(texture_atlas_linear.clone());

  // setup 2d scene
  commands.spawn(Camera2dBundle::default());

  // padded textures are to the right, unpadded to the left

  // draw unpadded texture atlas
  commands.spawn(SpriteBundle {
      texture: linear_texture.clone(),
      transform: Transform {
          translation: Vec3::new(-250.0, -130.0, 0.0),
          scale: Vec3::splat(0.8),
          //scale: Vec3::splat(1.0),
          ..default()
      },
      ..default()
  });

  let font = asset_server.load("fonts/FiraSans-Bold.ttf");

  // padding label text style
  let text_style: TextStyle = TextStyle {
      font: font.clone(),
      font_size: 50.0,
      color: Color::WHITE,
  };

  // labels to indicate padding

  // No padding
  create_label(
      &mut commands,
      (-250.0, 330.0, 0.0),
      "No padding",
      text_style.clone(),
  );

  // Padding
  create_label(&mut commands, (250.0, 330.0, 0.0), "Padding", text_style);

  let base_y = 64.0; // y position of the sprites
  let x = 0.;
  create_sprite_from_atlas(
    &mut commands,
    (x, base_y, 0.0),
    0,
    //atlas_handle,
    atlas_linear_handle,
    linear_texture,
  );

}

fn keyboard_input_system(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut texture_atlas_query: Query<&mut TextureAtlas, With<KEY0>>,
){
  if keyboard_input.just_pressed(KeyCode::Numpad0) {
    if let Ok(mut texAtlas) = texture_atlas_query.get_single_mut() {
      texAtlas.index = 1;
    }
  }
  if keyboard_input.just_released(KeyCode::Numpad0) {
    if let Ok(mut texAtlas) = texture_atlas_query.get_single_mut() {
      texAtlas.index = 2;
    }
  }
}

fn create_texture_atlas(
  folder: &LoadedFolder,
  padding: Option<UVec2>,
  sampling: Option<ImageSampler>,
  textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
  // Build a texture atlas using the individual sprites
  let mut texture_atlas_builder =
      TextureAtlasBuilder::default().padding(padding.unwrap_or_default());
  for handle in folder.handles.iter() {
      let id = handle.id().typed_unchecked::<Image>();
      let Some(texture) = textures.get(id) else {
          warn!(
              "{:?} did not resolve to an `Image` asset.",
              handle.path().unwrap()
          );
          continue;
      };

      texture_atlas_builder.add_texture(Some(id), texture);
  }

  let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
  let texture = textures.add(texture);

  // Update the sampling settings of the texture atlas
  let image = textures.get_mut(&texture).unwrap();
  image.sampler = sampling.unwrap_or_default();

  (texture_atlas_layout, texture)
}

/// Create and spawn a sprite from a texture atlas
fn create_sprite_from_atlas(
  commands: &mut Commands,
  translation: (f32, f32, f32),
  sprite_index: usize,
  atlas_handle: Handle<TextureAtlasLayout>,
  texture: Handle<Image>,
) {
  commands.spawn(SpriteSheetBundle {
      transform: Transform {
          translation: Vec3::new(translation.0, translation.1, translation.2),
          scale: Vec3::splat(3.0),
          ..default()
      },
      texture,
      atlas: TextureAtlas {
          index: sprite_index,
          layout: atlas_handle,
      },
      ..default()
  }).insert(KEY0);
}

/// Create and spawn a label (text)
fn create_label(
  commands: &mut Commands,
  translation: (f32, f32, f32),
  text: &str,
  text_style: TextStyle,
) {
  commands.spawn(Text2dBundle {
      text: Text::from_section(text, text_style).with_justify(JustifyText::Center),
      transform: Transform {
          translation: Vec3::new(translation.0, translation.1, translation.2),
          ..default()
      },
      ..default()
  });
}