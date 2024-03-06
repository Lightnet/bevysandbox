
use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
    .add_plugins(VKeyBoardPlugin)
    .run();
}

pub struct VKeyBoardPlugin;

impl Plugin for VKeyBoardPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        //app.init_state::<AppState>();
        //app.insert_resource(DisplayKeys::default());
        //app.add_systems(OnEnter(AppState::Setup), load_textures);
        //app.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)));
        //app.add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)));
        //app.add_systems(OnEnter(AppState::Finished), setup);

        app.add_systems(Startup, setup);
        app.add_systems(Startup, setup_virtual_keyboard);

    }
}

fn load_textures(){

}

fn setup(
  mut commands: Commands,
){
  // setup 2d scene
  commands.spawn(Camera2dBundle::default());
}

fn setup_virtual_keyboard(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
){

  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  // setup 2d scene
  //commands.spawn(Camera2dBundle::default());
  // padding label text style
  let text_style: TextStyle = TextStyle {
    font: font.clone(),
    font_size: 50.0,
    color: Color::WHITE,
  };

  let enity_keyboard = commands
    .spawn(NodeBundle {
      style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          flex_direction: FlexDirection::Column,
          //width: Val::Percent(100.0),
          //height: Val::Percent(100.0),
          width: Val::Px(800.0),
          height: Val::Px(600.0),
          ..default()
      },
      background_color: Color::CRIMSON.into(),
      //background_color: Color::BLUE.into(),
      ..default()
    }).id();

  let enity_keyboard_row1 = commands
    .spawn(NodeBundle {
      style: Style {
          align_items: AlignItems::Default,
          justify_content: JustifyContent::Default,
          top:Val::Px(8.0),
          //flex_direction: FlexDirection::Row,
          //width: Val::Percent(100.0),
          //height: Val::Percent(100.0),
          ..default()
      },
      //background_color: Color::CRIMSON.into(),
      //background_color: Color::ALICE_BLUE.into(),
      ..default()
    }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row1]);

  for i in 1..10 {
    let enity_keyicon = commands
    .spawn(NodeBundle {
      style: Style {
        height: Val::Px(32.0),
        width: Val::Px(32.0),
        align_items: AlignItems::Default,
        justify_content: JustifyContent::Default,
        margin: UiRect {
          left: Val::Px(2.0),
          right:Val::Px(2.0),
          top: Val::Px(2.0),
          bottom: Val::Px(2.0),
        },
        ..default()
      },
      //background_color: Color::CRIMSON.into(),
      //background_color: Color::ALICE_BLUE.into(),
      background_color: Color::GREEN.into(),
      ..default()
    }).id();
    commands.entity(enity_keyboard_row1).push_children(&[enity_keyicon]);
  }


  let enity_keyboard_row2 = commands
    .spawn(NodeBundle {
      style: Style {
          align_items: AlignItems::Default,
          justify_content: JustifyContent::Default,
          ..default()
      },
      ..default()
    }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row2]);

  for i2 in 1..10 {
    let enity_keyicon = commands
    .spawn(NodeBundle {
      style: Style {
        height: Val::Px(32.0),
        width: Val::Px(32.0),
        align_items: AlignItems::Default,
        justify_content: JustifyContent::Default,
        margin: UiRect {
          left: Val::Px(2.0),
          right:Val::Px(2.0),
          top: Val::Px(2.0),
          bottom: Val::Px(2.0),
        },
        ..default()
      },
      background_color: Color::ALICE_BLUE.into(),
      ..default()
    }).id();
    commands.entity(enity_keyboard_row2).push_children(&[enity_keyicon]);
  }

  let enity_keyboard_row3: Entity = commands
    .spawn(NodeBundle {
      style: Style {
          align_items: AlignItems::Default,
          justify_content: JustifyContent::Default,
          ..default()
      },
      ..default()
    }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row3]);

  for i3 in 1..10 {
    let enity_keyicon = commands
    .spawn(NodeBundle {
      style: Style {
        height: Val::Px(32.0),
        width: Val::Px(32.0),
        align_items: AlignItems::Default,
        justify_content: JustifyContent::Default,
        margin: UiRect {
          left: Val::Px(2.0),
          right:Val::Px(2.0),
          top: Val::Px(2.0),
          bottom: Val::Px(2.0),
        },
        ..default()
      },
      background_color: Color::ALICE_BLUE.into(),
      ..default()
    }).id();
    commands.entity(enity_keyboard_row3).push_children(&[enity_keyicon]);
  }

  let enity_keyboard_row4: Entity = commands
    .spawn(NodeBundle {
      style: Style {
          align_items: AlignItems::Default,
          justify_content: JustifyContent::Default,
          ..default()
      },
      ..default()
    }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row4]);

  for i4 in 1..10 {
    let enity_keyicon = commands
    .spawn(NodeBundle {
      style: Style {
        height: Val::Px(32.0),
        width: Val::Px(32.0),
        align_items: AlignItems::Default,
        justify_content: JustifyContent::Default,
        margin: UiRect {
          left: Val::Px(2.0),
          right:Val::Px(2.0),
          top: Val::Px(2.0),
          bottom: Val::Px(2.0),
        },
        ..default()
      },
      background_color: Color::ALICE_BLUE.into(),
      ..default()
    }).id();
    commands.entity(enity_keyboard_row4).push_children(&[enity_keyicon]);
  }

  let enity_keyboard_row5: Entity = commands
    .spawn(NodeBundle {
      style: Style {
          align_items: AlignItems::Default,
          justify_content: JustifyContent::Default,
          ..default()
      },
      ..default()
    }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row5]);

  for i5 in 1..10 {
    let enity_keyicon = commands
    .spawn(NodeBundle {
      style: Style {
        height: Val::Px(32.0),
        width: Val::Px(32.0),
        align_items: AlignItems::Default,
        justify_content: JustifyContent::Default,
        margin: UiRect {
          left: Val::Px(2.0),
          right:Val::Px(2.0),
          top: Val::Px(2.0),
          bottom: Val::Px(2.0),
        },
        ..default()
      },
      background_color: Color::ALICE_BLUE.into(),
      ..default()
    }).id();
    commands.entity(enity_keyboard_row5).push_children(&[enity_keyicon]);
  }

  let enity_keyboard_row6: Entity = commands
    .spawn(NodeBundle {
      style: Style {
          align_items: AlignItems::Default,
          justify_content: JustifyContent::Default,
          ..default()
      },
      ..default()
    }).id();
  commands.entity(enity_keyboard).push_children(&[enity_keyboard_row6]);

  for i6 in 1..2 {
    println!("{:?}",i6);
    let enity_keyicon = commands
    .spawn(NodeBundle {
      style: Style {
        height: Val::Px(32.0),
        width: Val::Px(32.0),
        align_items: AlignItems::Default,
        justify_content: JustifyContent::Default,
        margin: UiRect {
          left: Val::Px(2.0),
          right:Val::Px(2.0),
          top: Val::Px(2.0),
          bottom: Val::Px(2.0),
        },
        ..default()
      },
      background_color: Color::ALICE_BLUE.into(),
      ..default()
    })
    .insert(UiImage::new(asset_server.load("branding/icon.png")))
    .id();
    commands.entity(enity_keyboard_row6).push_children(&[enity_keyicon]);
  }

}

















