

 * https://github.com/sburris0/bevy_flycam
 * https://github.com/BlackPhlox/bevy_config_cam
 * 

Test build for first person view.

```
cargo run --package firstperson
```



https://discord.com/channels/691052431525675048/1023162244000919654/1023162244000919654

```rust
fn player_camera_controller(
    mut motion: EventReader<MouseMotion>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &Player), With<Camera>>,
){
    let window = get_primary_window_size(&windows);
    let mut yaw:f32 = 0.;
    let mut pitch:f32 = 0.;

    for (mut transform, player) in query.iter_mut() {
        for ev in motion.iter() {

            yaw = ev.delta.x / window.x * player.sensitivity;
            pitch = ev.delta.y / window.y * player.sensitivity;
        }
        transform.rotate(Quat::from_rotation_y(-yaw) * Quat::from_rotation_x(pitch) * Quat::from_rotation_z(0.));
    }
}
```



https://discord.com/channels/691052431525675048/1057896864336314409/1057896864336314409
```rust
fn camera_controls(
    mut q_camera_transform: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut q_player_transform: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let mut camera_transform = q_camera_transform.single_mut();
    let mut player_transform = q_player_transform.single_mut();

    for ev in mouse_motion.iter() { //rewrite
        camera_transform.rotate_y(ev.delta.x * TAU * -0.001);
        camera_transform.rotate_x(ev.delta.y * TAU * -0.001);
    }
    println!("{:?}",camera_transform.rotation );

    camera_transform.translation = player_transform.translation; // this line is unrelated, just makes the camera attach to the player character
}
```


https://discord.com/channels/691052431525675048/742884593551802431/979734709460430899
```rust
fn camera_look_around(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_transform: Query<&mut Transform, With<Camera>>,
) {
    for event in mouse_motion_events.iter() {
        camera_transform.single_mut()
            .rotate(Quat::from_euler(EulerRot::XYZ, event.delta.y * 0.01, event.delta.x * 0.01, 0.));
    }
}
```