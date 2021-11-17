mod canvas;
mod misc;
mod sidebar;

use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_plugin(canvas::CanvasUiPlugin);
    app.add_plugin(sidebar::SidebarUiPlugin);
    app.add_plugin(misc::MiscUiPlugin);
  }
}
