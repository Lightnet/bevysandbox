/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;
//use bevy_mod_picking::prelude::RaycastPickCamera;
use bevy_rapier3d::prelude::*;

#[allow(unused_variables, unused_mut)]
pub fn setup_physics_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
  println!("set up camera3d...");
  commands.spawn((
    PbrBundle {
      //mesh: meshes.add(shape::Plane::from_size(1.0).into()),
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
      ..default()
    },
    //RigidBody::KinematicPositionBased,
    KinematicCharacterController::default(),
    RigidBody::Dynamic,
    Collider::ball(1.0),
    //ColliderDebugColor(colors[color % 3]),
  ))
  .insert(TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)))
  .insert(Name::new("RigidBodyCube"))
  .with_children(|parent|{
    parent.spawn((
      Camera3dBundle {
        camera: Camera  { 
          order:1,
          //priority: 1 ,
          ..default()
        },
      //transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      //transform: Transform::from_xyz(10.0, 10., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
      transform: Transform::from_xyz(0.0, 10., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
      },
      //PlayerCamera,
      //RaycastPickCamera::default()//when main camera is active and select to update ray cast
    ));
  })
  
  ;

}

pub fn system_physics_player_input(
	mut controllers: Query<(&mut Transform, &mut KinematicCharacterController)>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
) {
	//println!("update...");

	for(mut entity_transform, mut controller )in controllers.iter_mut() {
		//controller.translation = Some(Vec3::new(1.0, -0.5, 0.01));
    let gravity = Vec3::new(0.0, -0.1, 0.0);
		if keyboard_input.pressed(KeyCode::KeyW) {
      //println!("W");
      let direction = entity_transform.forward() * 0.1;
      controller.translation = Some(direction + gravity);
		}else if keyboard_input.pressed(KeyCode::KeyS) {
      let direction = entity_transform.back() * 0.1;
      controller.translation = Some(direction + gravity);
		}else{
			controller.translation = Some(Vec3::new(0.0, -0.5, 0.0));
		}

    if keyboard_input.pressed( KeyCode::KeyA) {
      entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        0., 1.0 * 0.1, 0.)
      );
    }
    if keyboard_input.pressed( KeyCode::KeyD) {
      entity_transform.rotate(Quat::from_euler(EulerRot::XYZ,
        0., 1.0 * -0.1, 0.)
      );
    }
  
    if keyboard_input.pressed(KeyCode::Space) {
      let mut direction = Vec3::ZERO;
      direction.y = 20.;
      entity_transform.translation += time.delta_seconds() * 1.0 * direction;
    }
	}
}