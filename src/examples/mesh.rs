/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.
// https://docs.rs/bevy/latest/bevy/prelude/struct.Mesh.html
// https://github.com/bevyengine/bevy/blob/3b2c6ce49b3b9ea8bc5cb68f8d350a80ff928af6/crates/bevy_render/src/mesh/shape.rs#L224
use bevy::{prelude::*, winit::WinitSettings, render::{render_resource::PrimitiveTopology, mesh}};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
    //.insert_resource(WinitSettings::desktop_app())
    .add_systems(Startup, setup)
    .run();
}


fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  
  //commands.spawn(Camera2dBundle::default());
  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

  // Positions of the vertices
  // See https://bevy-cheatbook.github.io/features/coords.html
  mesh.insert_attribute(
      Mesh::ATTRIBUTE_POSITION,
      vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
  );

  // In this example, normals and UVs don't matter,
  // so we just use the same value for all of them
  mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
  mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

  // A triangle using vertices 0, 2, and 1.
  // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
  mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

  commands.spawn(PbrBundle {
      mesh: meshes.add(mesh),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..default()
  });

  commands.spawn(PointLightBundle {
      point_light: PointLight {
          intensity: 1500.0,
          shadows_enabled: true,
          ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
  });

  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
  });
}