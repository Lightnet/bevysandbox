


use bevy::{prelude::*, render::camera::Projection};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickableBundle};
use bevy_transform_gizmo::{TransformGizmoPlugin, GizmoPickSource, GizmoTransformable};

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
  left: f32,
  top: f32,
  right: f32,
  bottom: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn main() {
  App::new()
    //.add_plugins(DefaultPlugins)
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            //present_mode: AutoNoVsync,
            ..Default::default()
        },
        ..default()
    }))
    .add_plugins(DefaultPickingPlugins)
    //.add_plugin(TransformGizmoPlugin::default())
    .add_plugin(TransformGizmoPlugin::new(
      Quat::from_rotation_y(0.0), // Align the gizmo to a different coordinate system.
      //Quat::from_rotation_y(-0.2), // Align the gizmo to a different coordinate system.
                                   // Use TransformGizmoPlugin::default() to align to the
                                   // scene's coordinate system.
    ))
    .add_plugin(EguiPlugin)
    .init_resource::<OccupiedScreenSpace>()
    .add_startup_system(setup_system)
    .add_system(ui_example_system)
    .add_system(update_camera_transform_system)
    .run();
}

fn ui_example_system(
  mut egui_context: ResMut<EguiContext>,
  mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  occupied_screen_space.left = egui::SidePanel::left("left_panel")
    .resizable(true)
    .show(egui_context.ctx_mut(), |ui| {
      ui.label("Left Panel");
      if ui.button("Test").clicked(){
        commands.spawn(PbrBundle {
          mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
          material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
          transform: Transform::from_xyz(0.0, 0.5, 0.0),
          ..Default::default()
        })
        .insert(PickableBundle::default())
        .insert(GizmoTransformable);
      }


      ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    })
    .response
    .rect
    .width();
  occupied_screen_space.right = egui::SidePanel::right("right_panel")
    .resizable(true)
    .show(egui_context.ctx_mut(), |ui| {
      ui.label("Right Panel");
      ui.button("Test");
      ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    })
    .response
    .rect
    .width();
  occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
    .resizable(true)
    .show(egui_context.ctx_mut(), |ui| {
      ui.label("Top Panel");
      ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    })
    .response
    .rect
    .height();
  occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
    .resizable(true)
    .show(egui_context.ctx_mut(), |ui| {
      ui.label("Bottom Panel");

      ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    })
    .response
    .rect
    .height();
}

fn setup_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

  //plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..Default::default()
  });

  //cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..Default::default()
  })
  .insert(PickableBundle::default())
  .insert(GizmoTransformable)
  ;
  //Light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..Default::default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..Default::default()
  });
  // Camera
  let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
  let camera_transform =
    Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
  commands.insert_resource(OriginalCameraTransform(camera_transform));

  commands.spawn(Camera3dBundle {
    transform: camera_transform,
    ..Default::default()
  })
  .insert(PickingCameraBundle::default())
  .insert(GizmoPickSource::default())
  ;
}

fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Res<Windows>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    //conflict or missing mod pick transform
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
      Ok((Projection::Perspective(projection), transform)) => (projection, transform),
      //_ => unreachable!(),
      _ => return,
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.get_primary().unwrap();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
    transform.translation = original_camera_transform.translation
      + transform.rotation.mul_vec3(Vec3::new(
        (right_taken - left_taken) * frustum_width * 0.5,
        (top_taken - bottom_taken) * frustum_height * 0.5,
        0.0,
      ));
}