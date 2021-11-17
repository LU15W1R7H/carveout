use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct CurrentCurve(pub Option<Curve>);

#[derive(Serialize, Deserialize, Clone)]
pub struct Curve {
  /// in 0-1 normalized coordinates
  pub points: Vec<egui::Pos2>,
  pub stroke: egui::Stroke,
}
impl Curve {
  pub fn new(stroke: egui::Stroke) -> Self {
    Self {
      points: Vec::new(),
      stroke,
    }
  }
}
impl Default for Curve {
  fn default() -> Self {
    Self {
      points: Vec::new(),
      stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
    }
  }
}
