use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_system(ui.system())
    .run();
}

fn ui(egui_context: Res<EguiContext>) {
  egui::Window::new("Welcome").show(egui_context.ctx(), |ui| {
    ui.label("Welcome to Delusion.");
    ui.separator();
    ui.label("A tool for modern scientific digital pen note taking.");
  });
}
