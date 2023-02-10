//
// prefabs
// simple build for testing.
//

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_rapier3d::prelude::*;


pub fn setup_ground(){
  
}
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
pub fn add_physics_cube(
  commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
  x:f32,
  y:f32,
  z:f32,
){
  let color = 0;
  let colors = [
    Color::hsl(220.0, 1.0, 0.3),
    Color::hsl(180.0, 1.0, 0.3),
    Color::hsl(260.0, 1.0, 0.7),
  ];

  commands.spawn((
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      //transform: Transform::from_xyz(0.0, 32.0, 0.0),
      transform: Transform::from_xyz(x, y, z),
      ..default()
    },
    RigidBody::Dynamic,
    //Transform::from_xyz(0.0, 32.0, 0.0),
  )
  )
  .with_children(|child| {
    child.spawn((
      //TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)),
      Collider::cuboid(0.5,0.5,0.5),
      ColliderDebugColor(colors[color % 3]),
    ));
  })
    .insert(Name::new("RigidBodyCube"));
}

pub fn add_cube(
  commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
  x:f32,
  y:f32,
  z:f32,
){
  let color = 0;
  let colors = [
    Color::hsl(220.0, 1.0, 0.3),
    Color::hsl(180.0, 1.0, 0.3),
    Color::hsl(260.0, 1.0, 0.7),
  ];

  commands.spawn((
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      //transform: Transform::from_xyz(0.0, 32.0, 0.0),
      transform: Transform::from_xyz(x, y, z),
      ..default()
    },
  )
  )
  .insert(PickableBundle::default())
  .insert(Name::new("Cube"));
}