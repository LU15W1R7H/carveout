use bevy::prelude::*;

pub struct ToolboxPlugin;
impl Plugin for ToolboxPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.init_resource::<Toolbox>();
  }
}

#[derive(PartialEq)]
pub enum ToolMode {
  Pen,
  Hand,
  Scale,
}

pub struct Toolbox {
  pub mode: ToolMode,
  pub stroke: egui::Stroke,
  pub undo: bool,
}
impl Default for Toolbox {
  fn default() -> Self {
    Self {
      mode: ToolMode::Pen,
      stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
      undo: false,
    }
  }
}
