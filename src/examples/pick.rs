/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;
//use bevy_mod_picking::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugin(WorldInspectorPlugin::default())
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  //app: &mut App,
) {
  //App::new()
      //.add_plugins(DefaultPlugins)
      ///.add_plugins(DefaultPickingPlugins);

  commands.spawn((
    PbrBundle::default(),           // The `bevy_picking_raycast` backend works with meshes
    //PickableBundle::default(),      // Makes the entity pickable
    //RaycastPickTarget::default()    // Marker for the `bevy_picking_raycast` backend
  ));

  // cube
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });

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

  commands.spawn((
    Camera3dBundle::default(),
    //RaycastPickCamera::default(),   // Enable picking with this camera
  ))
  .insert(Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y))
  ;
}