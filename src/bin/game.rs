/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevysandbox::core::{physics::BsbPhysicsPlugin, world::WorldPhysicsTest01Plugin};

fn main() {
	App::new()
		//.insert_resource(ClearColor(Color::rgb(
			//0xF9 as f32 / 255.0,
			//0xF9 as f32 / 255.0,
			//0xFF as f32 / 255.0,
		//)))
		.add_plugins(DefaultPlugins)
		.add_plugin(WorldInspectorPlugin::new())
		.add_plugin(BsbPhysicsPlugin)
		//.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
		//.add_plugin(RapierDebugRenderPlugin::default())
		//.add_startup_system(setup_camera)
		//.add_startup_system(setup_physics)
		//.add_system(update_system)
		//.add_system(read_result_system)
		.add_plugin(WorldPhysicsTest01Plugin)
		.run();
}

//fn setup_camera(mut commands: Commands) {
	//commands.spawn(Camera3dBundle {
		//transform: Transform::from_xyz(-30.0, 30.0, 100.0)
			//.looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
		//..Default::default()
	//}).insert(Name::new("Camera"));
//}

/*
pub fn setup_physics(
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

		commands.spawn(
			TransformBundle::from(Transform::from_xyz(0.0,0.0, 0.0))
		)
		.with_children(|child| {
			child.spawn((
				TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)),
				RigidBody::KinematicPositionBased,
				KinematicCharacterController::default(),
				//RigidBody::Dynamic,
				Collider::ball(10.0),
				//ColliderDebugColor(colors[color % 3]),
			));
		})
			.insert(Name::new("RigidBodyCube"));

}
*/
/*
fn system_input_update_(
	mut controllers: Query<&mut KinematicCharacterController>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	//println!("update...");

	for mut controller in controllers.iter_mut() {
		//controller.translation = Some(Vec3::new(1.0, -0.5, 0.01));
		if keyboard_input.pressed(KeyCode::A) {
			controller.translation = Some(Vec3::new(1.0, -0.5, 0.01));
		}else if keyboard_input.pressed(KeyCode::D) {
			controller.translation = Some(Vec3::new(-1.0, -0.5, -0.01));
		}else{
			controller.translation = Some(Vec3::new(0.0, -0.5, 0.0));
		}
	}
}
*/

pub fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
	for (entity, output) in controllers.iter() {
			println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
				entity, output.effective_translation, output.grounded);
	}
}
