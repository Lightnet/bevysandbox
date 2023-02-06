There are different way to query.


 * https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html


 ```rust
 fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
	for (entity, output) in controllers.iter() {
			println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
				entity, output.effective_translation, output.grounded);
	}
}
 ```