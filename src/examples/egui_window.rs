use bevy::prelude::*;
use bevy_egui::{egui::{self, epaint::Shadow}, EguiContext, EguiContexts, EguiPlugin};

fn main() {
  println!("init bevy...");
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.1, 0.15, 0.15)))
    .insert_resource(AmbientLight{
      color:Color::default(),
      brightness:1000.0
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
    // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
    .add_systems(Update, ui_example_system)
    .add_systems(Startup, egui_no_shadow)
    .add_systems(Startup, setup_scene)
    .run();
}

// set up once
fn egui_no_shadow(mut contexts: EguiContexts) {
  //remove shadow
  contexts.ctx_mut().set_visuals(egui::Visuals {
    window_shadow: Default::default(),
    ..Default::default()
  });
}

// https://github.com/mvlabat/bevy_egui/issues/41
// https://github.com/emilk/egui/discussions/4083
// Window Shadow::NONE
fn ui_example_system(mut contexts: EguiContexts) {
  //remove shadow
  // contexts.ctx_mut().set_visuals(egui::Visuals {
  //   window_shadow: Default::default(),
  //   ..Default::default()
  // });
  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    
    ui.label("world");
  });
}


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
  });
}