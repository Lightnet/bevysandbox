

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
  #[default]
  Setup,
  Finished,
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
  //let boardkeys: Vec<ViewKey> = vec![ViewKey{press_id:0,release_id:1,key:KeyCode::Numpad0}];
  //let mut mykey: KeyBoards = KeyBoards{ key_list: boardkeys };
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
    .add_plugins(VKeyBoardPlugin)
    .run();
}

fn load_textures(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  //mut keyBoards: ResMut<DisplayKeys>,
) {
  //let boardkeys: Vec<ViewKey> = vec![
    //ViewKey{press_id:96,release_id:97,key:KeyCode::Escape,row:0}, // esc
  //];

}
pub struct VKeyBoardPlugin;

impl Plugin for VKeyBoardPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.init_state::<AppState>();
        //app.insert_resource(DisplayKeys::default());
        //app.add_systems(OnEnter(AppState::Setup), load_textures);
        //app.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)));
        //app.add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)));
        //app.add_systems(OnEnter(AppState::Finished), setup);
        app.add_systems(Startup, setup);

    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(
  mut commands: Commands,
  //rpg_sprite_handles: Res<KeyboardSpriteFolder>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
  loaded_folders: Res<Assets<LoadedFolder>>,
  mut textures: ResMut<Assets<Image>>,
  //mut displaykeys: ResMut<DisplayKeys>,
) {
  commands.spawn(Camera2dBundle::default());
  let button_text_style = TextStyle {
    font_size: 40.0,
    color: TEXT_COLOR,
    ..default()
};

let button_style = Style {
  width: Val::Px(250.0),
  height: Val::Px(65.0),
  margin: UiRect::all(Val::Px(20.0)),
  justify_content: JustifyContent::Center,
  align_items: AlignItems::Center,
  ..default()
};

  commands
            .spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|parent| {
              parent
                  .spawn(NodeBundle {
                      style: Style {
                          flex_direction: FlexDirection::Column,
                          align_items: AlignItems::Center,
                          ..default()
                      },
                      background_color: Color::CRIMSON.into(),
                      ..default()
                  })
                  .with_children(|parent| {
                      // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                      // use the default value, `FlexDirection::Row`, from left to right.
                      parent
                          .spawn(NodeBundle {
                              style: Style {
                                  align_items: AlignItems::Center,
                                  ..default()
                              },
                              background_color: Color::CRIMSON.into(),
                              ..default()
                          })
                          .with_children(|parent| {
                              // Display a label for the current setting
                              parent.spawn(TextBundle::from_section(
                                  "Display Quality",
                                  button_text_style.clone(),
                              ));
                              
                          });
                      // Display the back button to return to the settings screen
                      parent
                          .spawn((
                              ButtonBundle {
                                  style: button_style,
                                  background_color: NORMAL_BUTTON.into(),
                                  ..default()
                              }
                          ))
                          .with_children(|parent| {
                              parent.spawn(TextBundle::from_section("Back", button_text_style));
                          });
                  });
          });

}