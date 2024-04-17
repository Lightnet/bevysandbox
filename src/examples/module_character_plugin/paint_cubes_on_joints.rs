
use bevy::{prelude::*, scene};

use super::spawn_scenes::SceneName;

pub fn paint_cubes_on_joints(
  mut commands: Commands,
  scene_query: Query<Entity, With<SceneName>>,
  all_entities_with_children:Query<&Children>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mesh_handle:Query<&Handle<Mesh>>,
  mut mesh: ResMut<Assets<Mesh>>,
  global_tranforms:Query<&GlobalTransform>,
){
  let cube_color = Color::rgb(1.0, 0.0, 0.0);
  let cube_material_handler = materials.add(StandardMaterial{
    base_color: cube_color.clone(),
    ..Default::default()
  });

  //
  for scene_entity in &scene_query{
    
    for entity in all_entities_with_children.iter_descendants(scene_entity){
      if let Err(_) = mesh_handle.get(entity){
        if let Ok(_) = global_tranforms.get(entity){
          //spawn a cube
          let cube_handle = mesh.add(Cuboid::new(0.01,0.01,0.01));
          let mut entity_commands = commands.spawn(PbrBundle{
            mesh:cube_handle.clone(),
            material:cube_material_handler.clone(),
            //transform: Transform::from_xyz(0.0, 0.0, 0.0)
            ..Default::default()
          });
          entity_commands.set_parent(entity);

        }
      }
    }
  }
}