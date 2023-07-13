use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn setup(
    mut commands: Commands,
    app: &mut App,
) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins);

    commands.spawn((
        PbrBundle::default(),           // The `bevy_picking_raycast` backend works with meshes
        PickableBundle::default(),      // Makes the entity pickable
        RaycastPickTarget::default()    // Marker for the `bevy_picking_raycast` backend
    ));

    commands.spawn((
        Camera3dBundle::default(),
        RaycastPickCamera::default(),   // Enable picking with this camera
    ));
}