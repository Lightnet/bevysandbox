//  profiles for the non root package will be ignored, specify profiles at the workspace root

//use bevy_egui::{egui, EguiContext, EguiPlugin};

pub fn ui_example(mut egui_context: ResMut<EguiContext>) {
  egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
      ui.label("world");
      if ui.button("Open file…").clicked() {
        println!("Hello Click")
      }
  });
}