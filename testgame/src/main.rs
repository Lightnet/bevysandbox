

// https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs

//#[macro_use]
//extern crate log;
// https://github.com/rust-cli/env_logger/blob/main/examples/filters_from_code.rs
//use std::env;

//extern crate mylib;
//use mylib::test_lib;
//use testlib::test;

use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, DebugEventsPickingPlugin, DebugCursorPickingPlugin, PickingEvent, PickableBundle};
use bevy_rapier3d::prelude::*;
//use bevy::prelude::default;

use bevy::log::{debug,error,info,trace,warn};
use std::{f32::consts::PI, default};
//const FILTER_ENV: &str = "MY_LOG_LEVEL";

mod prefabs;
use prefabs::*;

// https://bevy-cheatbook.github.io/programming/res.html
#[derive(Resource)]
struct UserConfig {
  playername: String,
}

#[derive(Resource)]
struct PlayerInputSetting{
  axis_x:f32,
  axis_y:f32,
}

#[derive(Resource)]
struct PlayerInput{
  forward:bool,
  left:bool,
  right:bool,
  backward:bool,
  crouch:bool,
}

#[derive(Resource)]
struct WorldSettings{
  
}

// note that we can simply auto-derive Default here.
//#[derive(Component, Default)]
#[derive(Resource, Debug)]
struct TestAInstance {
  //#[default]
  name: String,
  //#[default]
  version: String,
}

impl Default for TestAInstance {
  fn default() -> Self {
    Self {
      name: "Test".into(),
      version:"0.1.0".into(),
    }
  }
}

enum BlockType{
  Grass,
  Dirt,
  Leaf,
  Wood,
  Truck,
  Clay,
  Rock,
  Stone,
  Sand,
  Water,
  Lave,
  Snow,
  Ice,
  Coal,
  Ruby,
  Dimaond,
  RedStone,
  BlueStone,
  Unknown,
}


enum PlayerMoveState{
  Ground,
  Water,
  Fly,
  Ghost,
  Vehicle,
  Climb,
  Rope,
  Slide,
  WallClimb,
  Edge,
  Pillar,
  
}

fn main() {
  //env_logger::init(); // do not use bevy log plugin conflict.

  //test_lib();
  //test();
    
  App::new()
    .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
    )))
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(TestAInstance::default())
    //.insert_resource(TestAInstance{ 
      //name:"test".into(),
      //..default()
    //})
    //.add_plugins(DefaultPlugins)
    .add_plugins(DefaultPlugins
      .set(WindowPlugin {
        window: WindowDescriptor {
          present_mode: PresentMode::AutoNoVsync, // Reduce input latency
          ..default()
        },
        ..default()
      })
      .set(AssetPlugin {
        watch_for_changes: true,
          ..default()
      })
      .set(ImagePlugin::default_nearest())
    )
    //.insert_resource(bevy::render::texture::ImageSettings::default_nearest())
    //.insert_resource(ImageSettings::default_nearest())
    //.insert_resource(ImageSettings::default_nearest())
    .add_plugin(WorldInspectorPlugin)
    .add_plugins(DefaultPickingPlugins) // <- Adds picking, interaction, and highlighting
    //.add_plugin(DebugCursorPickingPlugin) // <- Adds the debug cursor (optional)
    //.add_plugin(DebugEventsPickingPlugin) // <- Adds debug event logging (optional)
    .add_startup_system(load_models)
    .add_system(print_pick_events)

    .insert_resource(UserConfig{
      playername: "Guest".into(),
    })
    .insert_resource(
      PlayerInputSetting{
        axis_x: 1.0,
        axis_y: 1.0,
      }
    )
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    

    .add_startup_system(setup_physics_ground)
    .add_startup_system(setup_physics_cube)
    

    .add_startup_system(setup_lights)
    .add_startup_system(setup_pick01)

    .add_startup_system(get_playername)
    
    .add_startup_system(setup_camera00)
    .add_startup_system(setup_physics_player)
    .add_system(update_character_input)
    //.add_startup_system(display_logtype)
    //.add_system(input_playername)
    .run();

  //info!("a log from `MyLogger`");
  //trace!("some trace log");
  //debug!("some debug log");
  //info!("some information log");
  //warn!("some warning log");
  //error!("some error log");
  println!("END??")
}

fn load_models(
  mut commands: Commands, 
  asset_server: Res<AssetServer>,
){

  commands.spawn(SceneBundle {
    scene: asset_server.load("models/bevycube.glb#Scene0"),
    //transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(scale),
    transform: Transform {
      translation: Vec3::new(0.0, 1.0, 0.0),
      //rotation: Quat::from_rotation_x(-PI / 4.),
      scale:Vec3::new(1.0, 1.0, 1.0),
      ..default()
    },
    ..default()
  })
  .insert(Name::new("modelgltf"));
}

// https://docs.rs/bevy/latest/bevy/ecs/system/trait.Resource.html
fn get_playername(
  userconfig: Res<UserConfig>
){
  println!("player name: {}", userconfig.playername)
}

fn input_playername(
  mut userconfig: ResMut<UserConfig>,
  keyboard_input: Res<Input<KeyCode>>,
){
  //println!("player name: {}", userconfig.playername);
  //for mut controller in controllers.iter_mut() {
  //}

  if keyboard_input.pressed(KeyCode::Q) {
    userconfig.playername = "Test".into();
    info!("Player name: {}", userconfig.playername);
  }
  if keyboard_input.pressed(KeyCode::R) {
    userconfig.playername = "Test1".into();
    info!("Player name: {}", userconfig.playername);
  }

  if keyboard_input.pressed(KeyCode::E) {
    info!("Player name: {}", userconfig.playername);
  }

}

fn display_logtype(){
  debug!("debug");
  error!("error");
  info!("info");
  trace!("trace");
  warn!("warn");
}

//===============================================
// CAMERA
//===============================================
fn setup_camera00(mut commands: Commands) {
  commands.spawn((
    Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      //transform: Transform::from_xyz(-10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
      //transform: Transform::from_xyz(-20.0, 20.0, 50.0).looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
      //transform: Transform::from_xyz(-30.0, 30.0, 100.0).looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
      //transform: Transform::from_xyz(-120.0, 120.0, 400.0).looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
      ..Default::default()
    },
    PickingCameraBundle::default(), // <- Sets the camera to use for picking.
  ))
  //.insert(PickingCameraBundle::default())
  .insert(Name::new("Camera"));
  //commands.spawn(Camera3dBundle {
      //transform: Transform::from_xyz(-30.0, 30.0, 100.0)
          //.looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
      //..Default::default()
  //}).insert(Name::new("Camera"));
}

// https://www.reddit.com/r/bevy/comments/va2tug/odd_behavior_while_hovering_entities_bevy_mod/
fn setup_pick01(
  mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {


  add_cube(&mut commands, &mut meshes, &mut materials, 3.0, 2.0, 0.0);
  add_cube(&mut commands, &mut meshes, &mut materials, -3.0, 2.0, 0.0);

  //add_cube(&mut commands, &mut meshes, &mut materials, -128.0, 32.0, 0.0);
  //add_cube(&mut commands, &mut meshes, &mut materials, 128.0, 32.0, 0.0);
}

pub fn print_pick_events(mut events: EventReader<PickingEvent>) {
  for event in events.iter() {
      match event {
          PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
          PickingEvent::Hover(e) => {
            info!("Egads! A hover event!? {:?}", e);
          },
          PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
      }
  }
}

//===============================================
// PHYSICS GROUND
//===============================================
pub fn setup_physics_ground(
  mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
  /*
  * Ground
  */
  let ground_size = 128.0;
  let ground_height = 0.1;

  commands.spawn((
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    },
    PickableBundle::default(), // <- Makes the mesh pickable.
  ))
  .insert(Collider::cuboid(ground_size, ground_height, ground_size))
  .insert(Name::new("Ground"));

  //commands.spawn((
      //TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
      //Collider::cuboid(ground_size, ground_height, ground_size),
  //)).insert(Name::new("Ground"));

}


pub fn setup_lights(
  mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..Default::default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..Default::default()
  });

  commands.insert_resource(AmbientLight {
    //color: Color::ORANGE_RED,
    //brightness: 0.02,
    ..Default::default()
  });

  // directional 'sun' light
  const HALF_SIZE: f32 = 10.0;
  commands.spawn(DirectionalLightBundle {
      directional_light: DirectionalLight {
          // Configure the projection to better fit the scene
          shadow_projection: OrthographicProjection {
              left: -HALF_SIZE,
              right: HALF_SIZE,
              bottom: -HALF_SIZE,
              top: HALF_SIZE,
              near: -10.0 * HALF_SIZE,
              far: 10.0 * HALF_SIZE,
              ..default()
          },
          shadows_enabled: true,
          ..default()
      },
      transform: Transform {
          translation: Vec3::new(0.0, 2.0, 0.0),
          rotation: Quat::from_rotation_x(-PI / 4.),
          ..default()
      },
      ..default()
  });
}


//===============================================
// CUBE
//===============================================
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
pub fn setup_physics_cube(
  mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {

  add_physics_cube(&mut commands, &mut meshes, &mut materials, 6.0, 2.0, 0.0);
  add_physics_cube(&mut commands, &mut meshes, &mut materials, -6.0, 2.0, 0.0);

  /*
  let mut color = 0;
  let colors = [
    Color::hsl(220.0, 1.0, 0.3),
    Color::hsl(180.0, 1.0, 0.3),
    Color::hsl(260.0, 1.0, 0.7),
  ];

  commands.spawn((
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 28.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      transform: Transform::from_xyz(0.0, 32.0, 0.0),
      ..default()
    },
    RigidBody::Dynamic,
    //Transform::from_xyz(0.0, 32.0, 0.0),
  )
  )
  .with_children(|child| {
    child.spawn((
      //TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)),
      Collider::cuboid(16.0,16.0,16.0),
      ColliderDebugColor(colors[color % 3]),
    ));
  })
    .insert(Name::new("RigidBodyCube"));
    */
}
//===============================================
// CHARACTER BODY
//===============================================
pub fn setup_physics_player(
  mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {

  let mut color = 0;
  let colors = [
    Color::hsl(220.0, 1.0, 0.3),
    Color::hsl(180.0, 1.0, 0.3),
    Color::hsl(260.0, 1.0, 0.7),
  ];

  commands.spawn(
    TransformBundle::from(Transform::from_xyz(0.0,0.0, 0.0))
  )
  .with_children(|child| {
    child.spawn((
      TransformBundle::from(Transform::from_xyz(0.0, 10.0, 0.0)),
      RigidBody::KinematicPositionBased,
      KinematicCharacterController::default(),
      //RigidBody::Dynamic,
      //Collider::ball(10.0),
      //Collider::capsule(Vec3::new(1.,2.0,1.), Vec3::new(1.,2.,1.), 0.5),
      Collider::capsule_y(0.5, 0.5),
      ColliderDebugColor(colors[color % 3]),
    ));
  })
    .insert(Name::new("RigidBodyCube"));
}

//===============================================
// CHARACTER INPUT
//===============================================
fn update_character_input(
	mut controllers: Query<&mut KinematicCharacterController>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	//println!("update...");

	for mut controller in controllers.iter_mut() {
		//controller.translation = Some(Vec3::new(1.0, -0.5, 0.01));
		if keyboard_input.pressed(KeyCode::A) {
			controller.translation = Some(Vec3::new(0.1, -0.5, 0.0));
		}
		if keyboard_input.pressed(KeyCode::D) {
			controller.translation = Some(Vec3::new(-0.1, -0.5, 0.0));
		}
	}
}

fn read_result_character_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
	for (entity, output) in controllers.iter() {
			println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
				entity, output.effective_translation, output.grounded);
	}
}