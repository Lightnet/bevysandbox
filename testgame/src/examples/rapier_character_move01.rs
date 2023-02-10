

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


fn main() {
		App::new()
				.insert_resource(ClearColor(Color::rgb(
						0xF9 as f32 / 255.0,
						0xF9 as f32 / 255.0,
						0xFF as f32 / 255.0,
				)))
				.add_plugins(DefaultPlugins)
				.add_plugin(WorldInspectorPlugin)
				.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
				.add_plugin(RapierDebugRenderPlugin::default())
				.add_startup_system(setup_graphics)
				.add_startup_system(setup_physics)
				.add_system(update_system)
				.add_system(read_result_system)
				.run();
}

fn setup_graphics(mut commands: Commands) {
		commands.spawn(Camera3dBundle {
				transform: Transform::from_xyz(-30.0, 30.0, 100.0)
						.looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
				..Default::default()
		}).insert(Name::new("Camera"));;
}

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
		)).insert(Name::new("Ground"));;

		/*
		 * Create the cubes
		 */
		let num = 8;
		let rad = 16.0;

		let shift = rad * 2.0 + rad;
		let centerx = shift * (num / 2) as f32;
		let centery = shift / 2.0;
		let centerz = shift * (num / 2) as f32;

		let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
		let mut color = 0;
		let colors = [
				Color::hsl(220.0, 1.0, 0.3),
				Color::hsl(180.0, 1.0, 0.3),
				Color::hsl(260.0, 1.0, 0.7),
		];
/*
		commands
			.spawn(TransformBundle::from(Transform::from_rotation(
					Quat::from_rotation_x(0.2),
			)))
			.with_children(|child| {
					child.spawn((
							TransformBundle::from(Transform::from_xyz(0.0, 32.0, 0.0)),
							RigidBody::Dynamic,
							Collider::cuboid(rad, rad, rad),
							ColliderDebugColor(colors[color % 3]),
					));
			}).insert(Name::new("CubePhysics"));
			*/
		// cube
		/*
		commands.spawn(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 32.0 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			..default()
		}).insert(Name::new("Cube"));
		*/
		

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
				ColliderDebugColor(colors[color % 3]),
			));
		})
			.insert(Name::new("RigidBodyCube"));

}

fn update_system(mut controllers: Query<&mut KinematicCharacterController>) {
	//println!("update...");
	for mut controller in controllers.iter_mut() {
		controller.translation = Some(Vec3::new(1.0, -0.5, 0.01));
	}
}


fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
	for (entity, output) in controllers.iter() {
			println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
				entity, output.effective_translation, output.grounded);
	}
}
