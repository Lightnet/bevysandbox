// https://github.com/bevyengine/bevy/blob/release-0.13.1/examples/2d/texture_atlas.rs

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

#[derive(Component)]
struct KEY0;
// https://users.rust-lang.org/t/can-we-make-vector-of-structs-if-yes-than-how-to-make-and-use-it/19476
#[derive(Debug, Resource)]
pub struct ViewKey {
  press_id:usize,
  release_id:usize,
  key:KeyCode,
  row:usize,
}

#[derive(Component)]
pub struct TagInputKey {
  key:KeyCode
}


#[derive(Resource)]
pub struct DisplayKeys{
  key_list:Vec<ViewKey>
}

impl Default for DisplayKeys {
  fn default() -> Self {
    DisplayKeys{key_list: Vec::new()}
  }
}

fn main() {
  //let boardkeys: Vec<ViewKey> = vec![ViewKey{press_id:0,release_id:1,key:KeyCode::Numpad0}];
  //let mut mykey: KeyBoards = KeyBoards{ key_list: boardkeys };
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
    .add_plugins(VKeyBoardPlugin)
    //.init_state::<AppState>()//
    //.init_resource(KeyBoards{ key_list: todo!() })
    //.init_resource(KeyBoards::default())
    //.add_systems(OnEnter(AppState::Setup), load_textures)
    //.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
    //.add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)))
    //.add_systems(OnEnter(AppState::Finished), setup)
    .run();
}


pub struct VKeyBoardPlugin;

impl Plugin for VKeyBoardPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.init_state::<AppState>();
        app.insert_resource(DisplayKeys::default());
        app.add_systems(OnEnter(AppState::Setup), load_textures);
        app.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)));
        app.add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)));
        app.add_systems(OnEnter(AppState::Finished), setup);

    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
  #[default]
  Setup,
  Finished,
}

#[derive(Resource, Default)]
struct KeyboardSpriteFolder(Handle<LoadedFolder>);

fn load_textures(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut keyBoards: ResMut<DisplayKeys>,
) {
  let boardkeys: Vec<ViewKey> = vec![
    ViewKey{press_id:0,release_id:1,key:KeyCode::Numpad0,row:0},
    ViewKey{press_id:2,release_id:3,key:KeyCode::Numpad1,row:0},
    ViewKey{press_id:4,release_id:5,key:KeyCode::Numpad2,row:0},
    ViewKey{press_id:6,release_id:7,key:KeyCode::Numpad3,row:0},
    ViewKey{press_id:8,release_id:9,key:KeyCode::Numpad4,row:0},
    ViewKey{press_id:10,release_id:11,key:KeyCode::Numpad5,row:0},
    ViewKey{press_id:12,release_id:13,key:KeyCode::Numpad6,row:0},
    ViewKey{press_id:14,release_id:15,key:KeyCode::Numpad7,row:0},
    ViewKey{press_id:16,release_id:17,key:KeyCode::Numpad8,row:0},
    ViewKey{press_id:18,release_id:19,key:KeyCode::Numpad9,row:0},

    //ViewKey{press_id:54,release_id:63,key:KeyCode::Numpad9,row:1},
    //ViewKey{press_id:22,release_id:64,key:KeyCode::Numpad9,row:1},
    //ViewKey{press_id:24,release_id:65,key:KeyCode::Numpad9,row:1},//[
    //ViewKey{press_id:26,release_id:66,key:KeyCode::Numpad9,row:1},//b
    //ViewKey{press_id:28,release_id:67,key:KeyCode::Numpad9,row:1},//c
    //ViewKey{press_id:30,release_id:68,key:KeyCode::Numpad9,row:1},//cap lock
    //ViewKey{press_id:32,release_id:69,key:KeyCode::Numpad9,row:1},// shift
    //ViewKey{press_id:34,release_id:70,key:KeyCode::Numpad9,row:1},// shift
    //ViewKey{press_id:34,release_id:71,key:KeyCode::Numpad9,row:1},// caplock
    //ViewKey{press_id:34,release_id:72,key:KeyCode::Numpad9,row:1},// up
    //ViewKey{press_id:34,release_id:73,key:KeyCode::Numpad9,row:1},// up 
    //ViewKey{press_id:34,release_id:74,key:KeyCode::Numpad9,row:1},// :
    //ViewKey{press_id:34,release_id:75,key:KeyCode::Numpad9,row:1},// :
    //ViewKey{press_id:34,release_id:76,key:KeyCode::Numpad9,row:1},// ,
    //ViewKey{press_id:34,release_id:78,key:KeyCode::Numpad9,row:1},// ctrl
    //ViewKey{press_id:34,release_id:79,key:KeyCode::Numpad9,row:1},// ctrl
    //ViewKey{press_id:34,release_id:80,key:KeyCode::Numpad9,row:1},// C
    //ViewKey{press_id:34,release_id:81,key:KeyCode::Numpad9,row:1},// D
    //ViewKey{press_id:34,release_id:82,key:KeyCode::Numpad9,row:1},// Del
    //ViewKey{press_id:34,release_id:83,key:KeyCode::Numpad9,row:1},// D
    //ViewKey{press_id:34,release_id:84,key:KeyCode::Numpad9,row:1},// E
    //ViewKey{press_id:34,release_id:85,key:KeyCode::Numpad9,row:1},// E
    //ViewKey{press_id:34,release_id:86,key:KeyCode::Numpad9,row:1},// End
    //ViewKey{press_id:34,release_id:87,key:KeyCode::Numpad9,row:1},// Enter
    //ViewKey{press_id:34,release_id:88,key:KeyCode::Numpad9,row:1},// enter
    //ViewKey{press_id:34,release_id:89,key:KeyCode::Numpad9,row:1},// enter 
    //ViewKey{press_id:34,release_id:90,key:KeyCode::Numpad9,row:1},// esc
    //ViewKey{press_id:34,release_id:91,key:KeyCode::Numpad9,row:1},// esc
    //ViewKey{press_id:34,release_id:92,key:KeyCode::Numpad9,row:1},// !
    //ViewKey{press_id:34,release_id:93,key:KeyCode::Numpad9,row:1},// !
    //ViewKey{press_id:34,release_id:94,key:KeyCode::Numpad9,row:1},// E
    //ViewKey{press_id:34,release_id:95,key:KeyCode::Numpad9,row:1},// F
    //ViewKey{press_id:34,release_id:96,key:KeyCode::Numpad9,row:1},// F1
    //ViewKey{press_id:34,release_id:97,key:KeyCode::Numpad9,row:1},// F10
    //ViewKey{press_id:34,release_id:98,key:KeyCode::Numpad9,row:1},// F10
    //ViewKey{press_id:34,release_id:99,key:KeyCode::Numpad9,row:1},// F11
    //ViewKey{press_id:34,release_id:100,key:KeyCode::Numpad9,row:1},// F11
    //ViewKey{press_id:34,release_id:101,key:KeyCode::Numpad9,row:1},// F12
    //ViewKey{press_id:34,release_id:102,key:KeyCode::Numpad9,row:1},//F12
    //ViewKey{press_id:34,release_id:103,key:KeyCode::Numpad9,row:1},// F1
    //ViewKey{press_id:34,release_id:104,key:KeyCode::Numpad9,row:1},// F2
    //ViewKey{press_id:34,release_id:105,key:KeyCode::Numpad9,row:1},// F3
    //ViewKey{press_id:34,release_id:106,key:KeyCode::Numpad9,row:1},//F3
    //ViewKey{press_id:34,release_id:107,key:KeyCode::Numpad9,row:1},// F3
    //ViewKey{press_id:34,release_id:108,key:KeyCode::Numpad9,row:1},// F4
    //ViewKey{press_id:34,release_id:109,key:KeyCode::Numpad9,row:1},// F4
    //ViewKey{press_id:34,release_id:110,key:KeyCode::Numpad9,row:1},// F5
    //ViewKey{press_id:34,release_id:111,key:KeyCode::Numpad9,row:1},// F5
    //ViewKey{press_id:34,release_id:112,key:KeyCode::Numpad9,row:1},// F6
    //ViewKey{press_id:34,release_id:113,key:KeyCode::Numpad9,row:1},// F6
    //ViewKey{press_id:34,release_id:114,key:KeyCode::Numpad9,row:1},// F7
    //ViewKey{press_id:34,release_id:115,key:KeyCode::Numpad9,row:1},// F7
    ViewKey{press_id:34,release_id:116,key:KeyCode::Numpad9,row:1},// F8
    ViewKey{press_id:34,release_id:117,key:KeyCode::Numpad9,row:1},// F8


  ];


  keyBoards.key_list = boardkeys;
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
  mut displaykeys: ResMut<DisplayKeys>,
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
  /*
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
  */

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

  let mut base_y = 128.0; // y position of the sprites
  let mut x = -300.;
  let mut row = 0;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key
      );
    x += 32.;
    }
  }
  x = -300.;
  base_y = 96.;
  row = 1;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key
      );
      x += 32.;
    }
  }

  x = -300.;
  base_y = 64.;
  row = 2;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key
      );
      x += 32.;
    }
  }


  x = -300.;
  base_y = 64.;
  row = 3;
  for mykey in  displaykeys.key_list.iter(){
    if mykey.row == row{
      create_sprite_from_atlas(
        &mut commands,
        (x, base_y, 0.0),
        mykey.release_id,
        //atlas_handle,
        atlas_linear_handle.clone(),
        linear_texture.clone(),
        mykey.key
      );
      x += 32.;
    }
  }


  /*
  create_sprite_from_atlas(
    &mut commands,
    (x, base_y, 0.0),
    0,
    //atlas_handle,
    atlas_linear_handle,
    linear_texture,
  );
  */
}

fn keyboard_input_system(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut texture_atlas_query: Query<(&TagInputKey, &mut TextureAtlas)>,
  mut displaykeys: ResMut<DisplayKeys>,
){
  let shift = keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
  let ctrl = keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
  //look which key is press or release

  for mykey in  displaykeys.key_list.iter(){
    if keyboard_input.just_pressed(mykey.key) {

      for (tag_input_key, mut text_atlas ) in texture_atlas_query.iter_mut(){
        if tag_input_key.key == mykey.key {
          text_atlas.index = mykey.press_id;
          break;
        }
      }
    }
    if keyboard_input.just_released(mykey.key) {
      for (tag_input_key, mut text_atlas ) in texture_atlas_query.iter_mut(){
        if tag_input_key.key == mykey.key {
          text_atlas.index = mykey.release_id;
          break;
        }
      }
    }
  }

  /*
  if keyboard_input.just_pressed(KeyCode::Numpad0) {
    if let Ok(mut texAtlas) = texture_atlas_query.get_single_mut() {
      println!("{:?}",displaykeys.key_list);
      texAtlas.index = 0;
    }
  }
  if keyboard_input.just_released(KeyCode::Numpad0) {
    if let Ok(mut texAtlas) = texture_atlas_query.get_single_mut() {
      texAtlas.index = 1;
    }
  }
  */
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
  key:KeyCode
) {
  commands.spawn(SpriteSheetBundle {
      transform: Transform {
          translation: Vec3::new(translation.0, translation.1, translation.2),
          scale: Vec3::splat(0.64),
          ..default()
      },
      texture,
      atlas: TextureAtlas {
          index: sprite_index,
          layout: atlas_handle,
      },
      ..default()
  })
  .insert(TagInputKey{key})
  .insert(KEY0);
}
/*
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
*/

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