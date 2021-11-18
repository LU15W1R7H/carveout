use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use egui::emath;

pub struct CanvasPlugin;
impl Plugin for CanvasPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.init_resource::<CurrentCurve>();
    app.init_resource::<Viewport>();
  }
}

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

/// Canvas Viewport
pub struct Viewport {
  pub center: Vec2,
  pub size: f32,
}
impl Default for Viewport {
  fn default() -> Self {
    Self {
      center: Vec2::ZERO,
      size: 1.0,
    }
  }
}
// TODO: memoize transforms
impl Viewport {
  pub fn rect(&self) -> egui::Rect {
    egui::Rect::from_center_size(
      <[f32; 2]>::from(self.center).into(),
      egui::Vec2::splat(self.size),
    )
  }

  /// Transformation from view space (ui) to canvas space
  pub fn view_to_canvas(&self, view_rect: egui::Rect) -> emath::RectTransform {
    emath::RectTransform::from_to(view_rect, self.rect())
  }
  /// Transformation from canvas space to view space (ui)
  pub fn canvas_to_view(&self, view_rect: egui::Rect) -> emath::RectTransform {
    emath::RectTransform::from_to(self.rect(), view_rect)
  }
}
