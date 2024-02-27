//

use bevy::prelude::*;
//use bevy::window::PrimaryWindow;
//use bevy_inspector_egui::prelude::*;
//use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_egui::EguiPlugin;
use bevy_egui::{
  egui, 
  EguiContexts,
  //EguiContext,
  EguiPlugin
};

pub struct UIEditorPlugin;

impl Plugin for UIEditorPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(EguiPlugin);
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_systems(Update, inspector_ui);
  }
}

fn inspector_ui(
  //world: &mut World,
  mut contexts: EguiContexts,
  //mut ev_peers: EventWriter<PeersEvent>,
) {
  //let Ok(egui_context) = world
    //.query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    //.get_single(world)
  //else {
    //return;
  //};
  //let mut egui_context = egui_context.clone();
  //egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
  egui::Window::new("UI").show(contexts.ctx_mut(), |ui| {
    ui.label("world");
    egui::ScrollArea::vertical().show(ui, |ui| {

      if ui.button("hello").clicked(){
        println!("click");
        //ev_peers.send(PeersEvent("test".into()));
      }
      if ui.button("init peer!").clicked(){
        //ev_peers.send(PeersEvent("peer".into()));
      }
      if ui.button("ping").clicked(){
        //ev_peers.send(PeersEvent("ping".into()));
      }

      // equivalent to `WorldInspectorPlugin`
      //bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

      //egui::CollapsingHeader::new("Materials").show(ui, |ui| {
        //bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
      //});

      ui.heading("Entities");
      //bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
    });
  });
}