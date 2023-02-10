//! A simple 3D scene with light shining over a cube sitting on a plane.
// https://stackoverflow.com/questions/74465897/why-does-this-not-render-a-cube-with-a-texture-on-all-sides-bevy

use bevy::prelude::*;
use bevy::gltf::{Gltf, GltfMesh};

/// Helper resource for tracking our asset
#[derive(Resource)]
struct MyAssetPack{
    cube:Handle<Gltf>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_plugin(ImagePlugin::default_nearest())
        .add_startup_system_to_stage(StartupStage::PreStartup, load_gltf)
        //.add_startup_system(load_gltf)
        .add_startup_system(setup)
        .add_startup_system(spawn_gltf_objects)
        .run();
}

fn load_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let gltf = ass.load("models/bevycube.gltf#Scene0");
    commands.insert_resource(MyAssetPack{
        cube:gltf
    });
}
// https://bevy-cheatbook.github.io/3d/gltf.html
fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<MyAssetPack>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    println!("heleased");
    if let Some(gltf) = assets_gltf.get(&my.cube) {
        println!("loaded...");
        let my_cube = assets_gltfmesh.get(&gltf.named_meshes["Cube"]).unwrap();

        // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
        commands.spawn(PbrBundle {
            //mesh: my_cube.primitives[0].mesh.clone(),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            // (unwrap: material is optional, we assume this primitive has one)
            //material: myCube.primitives[0].material.clone().unwrap(),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}