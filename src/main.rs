#![allow(clippy::single_match)]

mod canvas;
mod geometry;
mod savefile;
mod splines;
mod tool;
mod ui;
mod util;

use bevy::prelude::*;

fn main() {
  App::new()
    .insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
      std::time::Duration::from_millis(10),
    ))
    .add_plugins(DefaultPlugins)
    .add_plugin(bevy_egui::EguiPlugin)
    .add_plugin(ui::UiPlugin)
    .add_plugin(canvas::CanvasPlugin)
    .add_plugin(tool::ToolPlugin)
    .add_plugin(savefile::SavefilePlugin)
    .run();
}
