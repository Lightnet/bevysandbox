/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BsbPhysicsPlugin;

impl Plugin for BsbPhysicsPlugin {
  fn build(&self, app: &mut App) {
    
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
		app.add_plugin(RapierDebugRenderPlugin::default());

  }
}

#[allow(unused_mut, unused_variables)]
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