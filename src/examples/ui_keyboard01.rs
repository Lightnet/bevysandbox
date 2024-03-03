/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

// https://taintedcoders.com/bevy/sprites/
// https://github.com/bevyengine/bevy/blob/main/examples/2d/texture_atlas.rs <-this one from bevy example

use bevy::{asset::LoadedFolder, prelude::*, render::render_resource::Texture};
//use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// https://github.com/bevyengine/bevy/blob/main/examples/2d/texture_atlas.rs
#[derive(Resource, Default)]
struct KeySpriteFolder(Handle<LoadedFolder>);

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugins(EguiPlugin)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup ,load_keys_textures)
    .add_systems(Update ,check_textures)
    .add_systems(Startup ,setup)
    .add_systems(Startup ,setup_sprite)
    .add_systems(Update ,button_input)
    .run();
}

fn load_keys_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
  // load multiple, individual sprites from a folder
  commands.insert_resource(KeySpriteFolder(asset_server.load_folder("kenney_input-prompts/Keyboard & Mouse/Default")));
}

fn check_textures(
  key_sprite_folder: Res<KeySpriteFolder>,
  mut events: EventReader<AssetEvent<LoadedFolder>>,
){
  // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
  for event in events.read() {
    if event.is_loaded_with_dependencies(&key_sprite_folder.0) {
      //next_state.set(AppState::Finished);
      println!("LOAD KEY IMAGE");
    }
  }
}



#[derive(Component)]
struct KEY0;

/// set up a simple 3D scene
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  // plane
  
  commands.spawn(PbrBundle {
    mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
    ..default()
  });
  
  // cube
  
  commands.spawn(PbrBundle {
    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
  /*
  */
  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
  // light
  commands.spawn(DirectionalLightBundle {
    transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });

  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });

  
  commands.spawn(TextBundle::from_section(
    "[= Key Board Test UI =]\n\
    ---\n\
    ---",
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 24.0,
        color: Color::WHITE,
    },
  ));
  
}

// kenney_input-prompts\Flairs\Vector\ flair_circle_0.svg //nope png format
// kenney_input-prompts\Flairs\Vector\ flair_disabled.svg //nope png format
// kenney_input-prompts/Keyboard & Mouse/Default/keyboard_1_outline.png // okay
// kenney_input-prompts/Keyboard & Mouse/Default/keyboard_0.png // okay

#[allow(unused_mut, unused_variables)]
fn setup_sprite(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
){

  let text_style = TextStyle {
    //font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size: 20.0,
    ..default()
  };

  //let image = asset_server.load("branding/icon.png");
  //let image = asset_server.load("kenney_input-prompts/Flairs/Default/flair_disabled.png");
  // kenney_input-prompts/Keyboard & Mouse/Default/keyboard_0.png
  let image = asset_server.load("kenney_input-prompts/Keyboard & Mouse/Default/keyboard_0.png");

  commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            //background_color: Color::ANTIQUE_WHITE.into(),
            ..Default::default()
        })
        .with_children(|parent| {

          
          parent.spawn((
            ImageBundle {
                image: UiImage::new(image.clone()),
                style: Style {
                    min_width: Val::Px(16.),
                    min_height: Val::Px(16.),
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),

                ..Default::default()
            },
            Interaction::default(),
            Outline {
                width: Val::Px(2.),
                offset: Val::Px(2.),
                color: Color::NONE,
            },
          )).insert(KEY0);


        });
        
}

fn button_input(
  keyboard: Res<ButtonInput<KeyCode>>,
  asset_server: Res<AssetServer>,
  //UIImage texture, tag KEYO
  mut texture_query: Query<&mut UiImage, With<KEY0>>,
){
  if keyboard.just_pressed(KeyCode::Numpad0) {
    if let Ok(mut tex) = texture_query.get_single_mut() {
      let image = asset_server.load("kenney_input-prompts/Keyboard & Mouse/Default/keyboard_1_outline.png");
      *tex = image.into();
    }
    println!("0");
  }

  if keyboard.just_released(KeyCode::Numpad0) {
    if let Ok(mut tex) = texture_query.get_single_mut() {
      let image = asset_server.load("kenney_input-prompts/Keyboard & Mouse/Default/keyboard_1.png");
      *tex = image.into();
    }
    println!("0");
  }
}
