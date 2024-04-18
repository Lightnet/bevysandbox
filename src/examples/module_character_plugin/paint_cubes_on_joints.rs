
use std::fmt::format;

use bevy::{prelude::*, scene};
use bevy_mod_billboard::BillboardTextBundle;

use crate::asset_loader_plugin::MyAssets;

use super::spawn_scenes::SceneName;

pub fn paint_cubes_on_joints(
  mut commands: Commands,
  scene_query: Query<Entity, With<SceneName>>,
  all_entities_with_children:Query<&Children>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mesh_handle:Query<&Handle<Mesh>>,
  mut mesh: ResMut<Assets<Mesh>>,
  global_tranforms:Query<&GlobalTransform>,
  asset_pack: Res<MyAssets>,
  names:Query<&Name>,
){
  let cube_color = Color::rgb(1.0, 0.0, 0.0);
  let cube_material_handler = materials.add(StandardMaterial{
    base_color: cube_color.clone(),
    ..Default::default()
  });

  let font_handle = asset_pack
    .font_files
    .get("fonts/FiraSans-Bold.ttf")
    .expect("to have loaded the font");

  //
  for scene_entity in &scene_query{
    
    for entity in all_entities_with_children.iter_descendants(scene_entity){
      if let Err(_) = mesh_handle.get(entity){
        if let Ok(_) = global_tranforms.get(entity){
          let name_result = names.get(entity);
          let name = if let Ok(name) = name_result{
            format!("{}", name)
          }else{
            "unname entity".to_string()
          };

          //spawn a cube
          let cube_handle = mesh.add(Cuboid::new(0.01,0.01,0.01));
          let mut entity_commands = commands.spawn(PbrBundle{
            mesh:cube_handle.clone(),
            material:cube_material_handler.clone(),
            //transform: Transform::from_xyz(0.0, 0.0, 0.0)
            ..Default::default()
          });
          entity_commands.set_parent(entity);

          let mut billboard_entity_commands = commands.spawn(BillboardTextBundle{
            transform:Transform::from_xyz(0.0, 0.01, 0.0).with_scale(Vec3::splat(0.0005)),
            text: Text::from_sections([TextSection{
              value: name,
              style: TextStyle{
                font_size: 12.0,
                font: font_handle.clone(),
                color: Color::WHITE
              }
            }]),
            ..Default::default()
          });
          billboard_entity_commands.set_parent(entity);

        }
      }
    }
  }
}