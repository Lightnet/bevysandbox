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
// https://gist.github.com/DGriffin91/e63e5f7a90b633250c2cf4bf8fd61ef8
// https://bevyengine.org/examples/3D%20Rendering/generate-custom-mesh/

use bevy::{prelude::*, winit::WinitSettings, render::{render_resource::PrimitiveTopology, mesh::{self, Indices}}};
use bevysandbox::core::camera::flycam::PlayerPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PlayerPlugin)
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
  mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0., 0., 0.], [0., 2.1, 0.], [2., 0., 0.]],);

  // In this example, normals and UVs don't matter,
  // so we just use the same value for all of them
  mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
  mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

  // A triangle using vertices 0, 2, and 1.
  // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
  mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

  commands.spawn(PbrBundle {
    //mesh: meshes.add(mesh),
    mesh: meshes.add(create_cube_mesh()),
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

  //commands.spawn(Camera3dBundle {
    //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //..default()
  //});
}

fn create_cube_mesh() -> Mesh {
  let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);

  #[rustfmt::skip]
  cube_mesh.insert_attribute(
      Mesh::ATTRIBUTE_POSITION,
      // Each array is an [x, y, z] coordinate in local space.
      // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
      // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
      vec![
          // top (facing towards +y)
          [-0.5, 0.5, -0.5], // vertex with index 0
          [0.5, 0.5, -0.5], // vertex with index 1
          [0.5, 0.5, 0.5], // etc. until 23
          [-0.5, 0.5, 0.5],
          // bottom   (-y)
          [-0.5, -0.5, -0.5],
          [0.5, -0.5, -0.5],
          [0.5, -0.5, 0.5],
          [-0.5, -0.5, 0.5],
          // right    (+x)
          [0.5, -0.5, -0.5],
          [0.5, -0.5, 0.5],
          [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
          [0.5, 0.5, -0.5],
          // left     (-x)
          [-0.5, -0.5, -0.5],
          [-0.5, -0.5, 0.5],
          [-0.5, 0.5, 0.5],
          [-0.5, 0.5, -0.5],
          // back     (+z)
          [-0.5, -0.5, 0.5],
          [-0.5, 0.5, 0.5],
          [0.5, 0.5, 0.5],
          [0.5, -0.5, 0.5],
          // forward  (-z)
          [-0.5, -0.5, -0.5],
          [-0.5, 0.5, -0.5],
          [0.5, 0.5, -0.5],
          [0.5, -0.5, -0.5],
      ],
  );

  // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
  // Take a look at the custom image (assets/textures/array_texture.png)
  // so the UV coords will make more sense
  // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
  #[rustfmt::skip]
  cube_mesh.insert_attribute(
      Mesh::ATTRIBUTE_UV_0,
      vec![
          // Assigning the UV coords for the top side.
          [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.25],
          // Assigning the UV coords for the bottom side.
          [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
          // Assigning the UV coords for the right side.
          [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
          // Assigning the UV coords for the left side. 
          [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
          // Assigning the UV coords for the back side.
          [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
          // Assigning the UV coords for the forward side.
          [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
      ],
  );

  // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
  // the surface.
  // Normals are required for correct lighting calculations.
  // Each array represents a normalized vector, which length should be equal to 1.0.
  #[rustfmt::skip]
  cube_mesh.insert_attribute(
      Mesh::ATTRIBUTE_NORMAL,
      vec![
          // Normals for the top side (towards +y)
          [0.0, 1.0, 0.0],
          [0.0, 1.0, 0.0],
          [0.0, 1.0, 0.0],
          [0.0, 1.0, 0.0],
          // Normals for the bottom side (towards -y)
          [0.0, -1.0, 0.0],
          [0.0, -1.0, 0.0],
          [0.0, -1.0, 0.0],
          [0.0, -1.0, 0.0],
          // Normals for the right side (towards +x)
          [1.0, 0.0, 0.0],
          [1.0, 0.0, 0.0],
          [1.0, 0.0, 0.0],
          [1.0, 0.0, 0.0],
          // Normals for the left side (towards -x)
          [-1.0, 0.0, 0.0],
          [-1.0, 0.0, 0.0],
          [-1.0, 0.0, 0.0],
          [-1.0, 0.0, 0.0],
          // Normals for the back side (towards +z)
          [0.0, 0.0, 1.0],
          [0.0, 0.0, 1.0],
          [0.0, 0.0, 1.0],
          [0.0, 0.0, 1.0],
          // Normals for the forward side (towards -z)
          [0.0, 0.0, -1.0],
          [0.0, 0.0, -1.0],
          [0.0, 0.0, -1.0],
          [0.0, 0.0, -1.0],
      ],
  );

  // Create the triangles out of the 24 vertices we created.
  // To construct a square, we need 2 triangles, therefore 12 triangles in total.
  // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
  // by one, in a counter-clockwise order (relative to the position of the viewer, the order
  // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
  // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
  // further examples and the implementation of the built-in shapes.
  #[rustfmt::skip]
  cube_mesh.set_indices(Some(Indices::U32(vec![
      0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
      4,5,7 , 5,6,7, // bottom (-y) 
      8,11,9 , 9,11,10, // right (+x)
      12,13,15 , 13,14,15, // left (-x)
      16,19,17 , 17,19,18, // back (+z)
      20,21,23 , 21,22,23, // forward (-z)
  ])));

  cube_mesh
}