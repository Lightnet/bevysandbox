//! A simple 3D scene with light shining over a cube sitting on a plane.
// https://stackoverflow.com/questions/74465897/why-does-this-not-render-a-cube-with-a-texture-on-all-sides-bevy
// https://stackoverflow.com/questions/66865541/how-to-load-part-of-image-in-bevy-engine
// 

use bevy::prelude::*;
use bevy::gltf::{Gltf, GltfMesh};

/// Helper resource for tracking our asset
#[derive(Resource)]
struct MyAssetPack{
  cube:Handle<Gltf>,
}

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    //.add_startup_system_to_stage(StartupStage::PreStartup, load_gltf)
    .add_startup_system(setup)
    .run();
}

/// set up a simple 3D scene
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {

  let texture_handle = asset_server.load("textures/rogb.png");

  let material_handle = materials.add(StandardMaterial {
    base_color_texture: Some(texture_handle.clone()),
    //shaded: false,
    ..Default::default()
  });
  
  // plane
  //commands.spawn(PbrBundle {
    //mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //..default()
  //});

  let mut uvs = Vec::new();
  uvs.push([0.0, 1.0]);
  uvs.push([0.0, 0.0]);
  uvs.push([1.0, 0.0]);
  uvs.push([1.0, 1.0]);

  let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
  //mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
  mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

  // use only the upper left part of the texture atlas
  let mut uvs1 = Vec::new();
  uvs1.push([0.0, 0.5]);
  uvs1.push([0.0, 0.0]);
  uvs1.push([0.5, 0.0]);
  uvs1.push([0.5, 0.5]);

  let mut mesh1 = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
  mesh1.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs1);

  // use only the lower right part of the texture atlas
  let mut uvs2 = Vec::new();
  uvs2.push([0.5, 1.0]);
  uvs2.push([0.5, 0.5]);
  uvs2.push([1.0, 0.5]);
  uvs2.push([1.0, 1.0]);

  let mut mesh2 = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
  mesh2.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs2);

  commands
    // the whole texture array
    .spawn(PbrBundle {
      mesh: meshes.add(mesh),
      material: material_handle.clone(),
      ..Default::default()
    });
    // only the "first" item of the texture array
  commands.spawn(PbrBundle {
      mesh: meshes.add(mesh1),
      material: material_handle.clone(),
      transform: Transform::from_translation(Vec3::new(2.0, 0.0, 0.0)),
      ..Default::default()
    });
    // only the "last" item of the texture array
  commands.spawn(PbrBundle {
      mesh: meshes.add(mesh2),
      material: material_handle.clone(),
      transform: Transform::from_translation(Vec3::new(-2.0, 0.0, 0.0)),
      ..Default::default()
    });








  // cube
  /*
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    material: material_handle,
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
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
  // camera
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });
}