use bevy::prelude::*;

use palette::LinSrgba;

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
  pub curve_width: f32,
  pub curve_color: LinSrgba,
  pub undo: bool,
}
impl Default for Toolbox {
  fn default() -> Self {
    Self {
      mode: ToolMode::Pen,
      curve_width: 1.0,
      curve_color: palette::LinSrgba::new(1.0, 1.0, 1.0, 1.0),
      undo: false,
    }
  }
}
