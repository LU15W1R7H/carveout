#![allow(clippy::single_match)]

mod canvas;
mod savefile;
mod splines;
mod toolbox;
//mod render;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
  App::build()
    .insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(std::time::Duration::from_millis(10)))
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(canvas::CanvasPlugin)
    .add_plugin(toolbox::ToolboxPlugin)
    .add_plugin(savefile::SavefilePlugin)
    .add_system(welcome_ui.system())
    .run();
}

fn welcome_ui(egui: Res<EguiContext>) {
  egui::Window::new("Welcome").show(egui.ctx(), |ui| {
    ui.label("Welcome to Carveout.");
    ui.separator();
    ui.label("A tool for modern scientific digital pen note taking.");
  });
}
