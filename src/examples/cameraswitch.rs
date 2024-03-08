

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

fn main() {
  //let boardkeys: Vec<ViewKey> = vec![ViewKey{press_id:0,release_id:1,key:KeyCode::Numpad0}];
  //let mut mykey: KeyBoards = KeyBoards{ key_list: boardkeys };
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
    .add_plugins(CameraSwitchPlugin)
    .run();
}

pub struct CameraSwitchPlugin;

#[derive(Event)]
struct CameraEvent(String);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum  CameraState{
  #[default]
  FlyView,
  FirstPersonView,
}

impl Plugin for CameraSwitchPlugin {
  fn build(&self, app: &mut App) {
    // add things to your app here
    // app.init_state::<AppState>();
    // app.insert_resource(DisplayKeys::default());
    // app.add_systems(OnEnter(AppState::Setup), load_textures);
    // app.add_systems(Update, check_textures.run_if(in_state(AppState::Setup)));
    // app.add_systems(Update, keyboard_input_system.run_if(in_state(AppState::Finished)));
    // app.add_systems(OnEnter(AppState::Finished), setup);
    app.init_state::<CameraState>();
    app.add_event::<CameraEvent>();
    app.add_systems(Startup, setup_scene);
    app.add_systems(Startup, setup);
    app.add_systems(Update, switch_camera);

  }
}

#[derive(Component)]
pub struct FlyCamera;

#[derive(Component)]
pub struct FirstPersonCamera;

fn setup(
  mut commands: Commands,
){
  // commands.spawn(Camera3dBundle{
  //   camera: Camera{
  //     is_active:true,
  //     ..default()
  //   },
  //   ..default()
  // })
  // .insert(FlyCamera);


  commands.spawn(Camera3dBundle{
    transform: Transform::from_xyz(-2.5, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    camera: Camera{
      is_active:false,
      ..default()
    },
    ..default()
  })
  .insert(FirstPersonCamera);
}

/// set up a simple 3D scene
fn setup_scene(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // circular base
  commands.spawn(PbrBundle {
      mesh: meshes.add(Circle::new(4.0)),
      material: materials.add(Color::WHITE),
      transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
      ..default()
  });
  // cube
  commands.spawn(PbrBundle {
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::rgb_u8(124, 144, 255)),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  });
  // light
  commands.spawn(PointLightBundle {
      point_light: PointLight {
          shadows_enabled: true,
          ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
  });
  // camera
  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
  }).insert(FlyCamera);
}

// https://bevyengine.org/examples/Math/render-primitives/
fn switch_camera(
  mut commands: Commands,
  key: Res<ButtonInput<KeyCode>>,
  mut cam_next_state: ResMut<NextState<CameraState>>,
  state: Res<State<CameraState>>,
  mut fpv_q: Query<&mut Camera, (With<FirstPersonCamera>, Without<FlyCamera>)>,
  mut fly_q: Query<&mut Camera, (With<FlyCamera>, Without<FirstPersonCamera> )>,
){
  if key.just_pressed(KeyCode::KeyC){
    println!("camera toggle... {:?}", state.get());
    match state.get(){
        CameraState::FlyView => {
          cam_next_state.set(CameraState::FirstPersonView);
          let mut firstperson = fpv_q.single_mut();
          let mut flyperson = fly_q.single_mut();

          flyperson.is_active = false;
          firstperson.is_active = true;

          cam_next_state.set(CameraState::FirstPersonView);
        },
        CameraState::FirstPersonView => {

          let mut firstperson = fpv_q.single_mut();
          let mut flyperson = fly_q.single_mut();
          firstperson.is_active = false;
          flyperson.is_active = true;
          cam_next_state.set(CameraState::FlyView);
        },
    }
  }
}

fn camera_events(

){

}