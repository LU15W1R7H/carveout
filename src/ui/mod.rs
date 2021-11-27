pub mod canvas;
mod misc;
mod sidebar;

use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system_to_stage(
      CoreStage::Update,
      sidebar::sidebar_ui_sys.system().before("canvas"),
    );
    app.add_plugin(canvas::CanvasUiPlugin);
  }
}
