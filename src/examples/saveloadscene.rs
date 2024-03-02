/*
  Project Name: Bevy Sandbox
  Licenses: MIT
  Created By: Lightnet
  Information:
    Note there are multiple licenses.

    Save and loading required some more config.
 */

// https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs
// https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs
// https://www.youtube.com/watch?v=4uASkH-FUWk

//! This example illustrates loading scenes from files.
use std::fs::File;
use std::io::Write;

use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}

// Resources can be serialized in scenes as well, with the same requirements `Component`s have.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub score: u32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // This tells the AssetServer to watch for changes to assets.
            // It enables our scenes to automatically reload in game when we modify their files.
            //watch_for_changes: true,
            ..default()
        }))
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ResourceA>()
        .add_systems(Startup ,infotext_system)
        .add_systems(Update ,save_scene_system)
        .add_systems(Update ,load_scene_system)
        .add_systems(Update, log_system)
        //.add_systems(Update, handle_save_input)
        .run();
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}


fn load_scene_system(world: &mut World) {
    //let commands = world;
    let asset_server = world.resource::<AssetServer>();
    let keyboard = world.resource::<ButtonInput<KeyCode>>();
    if keyboard.just_pressed(KeyCode::KeyD) {
        // "Spawning" a scene bundle creates a new entity and spawns new instances
        // of the given scene's entities as children of that entity.
        world.spawn(DynamicSceneBundle {
            // Scenes are loaded just like any other asset.
            scene: asset_server.load(SCENE_FILE_PATH),
            ..default()
        });
    }
}

// This system logs all ComponentA components in our world. Try making a change to a ComponentA in
// load_scene_example.scn. You should immediately see the changes appear in the console.
fn log_system(query: Query<(Entity, &ComponentA), Changed<ComponentA>>) {
    for (entity, component_a) in &query {
        info!("  Entity({})", entity.index());
        info!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
}

// https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs
fn save_scene_system(world: &mut World) {

    let keyboard = world.resource::<ButtonInput<KeyCode>>();

    if keyboard.just_pressed(KeyCode::KeyS) {
        println!("TEST");

        let mut scene_world = World::new();

        let type_registry = world.resource::<AppTypeRegistry>().clone();
        scene_world.insert_resource(type_registry);

        let mut component_b = ComponentB::from_world(world);
        component_b.value = "hello".to_string();
        scene_world.spawn((
            component_b,
            ComponentA { x: 1.0, y: 2.0 },
            Transform::IDENTITY,
            Name::new("joe"),
        ));
        scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
        scene_world.insert_resource(ResourceA { score: 1 });

        let scene = DynamicScene::from_world(&scene_world);
        let type_registry = world.resource::<AppTypeRegistry>();
        let serialized_scene = scene.serialize_ron(type_registry).unwrap();
        
        info!("{}", serialized_scene);

        #[cfg(not(target_arch = "wasm32"))]
        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error while writing scene to file");
            })
            .detach();
        println!("finish...")
    }
}

// This is only necessary for the info message in the UI. See examples/ui/text.rs for a standalone
// text example.
#[allow(unused)]
fn infotext_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        TextBundle::from_section(
            "Nothing to see in this window! Check the console output!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
    );
}


//fn handle_save_input(world: &mut World) {
    //let keys = world.resource::<ButtonInput<KeyCode>>();
    //if keys.just_released(KeyCode::Space) {
        //println!("KEY SPACE IN WORLD ");
    //}
//}