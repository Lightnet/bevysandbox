

// https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs

//#[macro_use]
//extern crate log;
// https://github.com/rust-cli/env_logger/blob/main/examples/filters_from_code.rs
//use std::env;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use bevy::log::{debug,error,info,trace,warn};
//const FILTER_ENV: &str = "MY_LOG_LEVEL";

mod prefabs;
//use prefabs::setup_ground;

// https://bevy-cheatbook.github.io/programming/res.html
#[derive(Resource)]
struct UserConfig {
  playername: String,
}

fn main() {
  //env_logger::init(); // do not use bevy log plugin conflict.
    
  App::new()
    .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
    )))
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin)
    .insert_resource(UserConfig{
      playername: "Guest".into(),
    })
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())


    .add_startup_system(setup_physics_ground)
    .add_startup_system(setup_physics_cube)
    .add_system(update_character_input)

    .add_startup_system(get_playername)
    
    .add_startup_system(setup_camera00)
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

fn setup_camera00(mut commands: Commands) {
  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-30.0, 30.0, 100.0)
          .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
      ..Default::default()
  }).insert(Name::new("Camera"));;
}

pub fn setup_physics_ground(
  mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
  /*
  * Ground
  */
  let ground_size = 200.1;
  let ground_height = 0.1;

  commands.spawn((
      TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
      Collider::cuboid(ground_size, ground_height, ground_size),
  )).insert(Name::new("Ground"));

}
//===============================================
// CUBE
//===============================================
pub fn setup_physics_cube(
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
      TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)),
      RigidBody::KinematicPositionBased,
      KinematicCharacterController::default(),
      //RigidBody::Dynamic,
      Collider::ball(10.0),
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
			controller.translation = Some(Vec3::new(1.0, -0.5, 0.0));
		}
		if keyboard_input.pressed(KeyCode::D) {
			controller.translation = Some(Vec3::new(-1.0, -0.5, 0.0));
		}
	}
}

fn read_result_character_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
	for (entity, output) in controllers.iter() {
			println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
				entity, output.effective_translation, output.grounded);
	}
}