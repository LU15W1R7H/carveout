use bevy::prelude::*;
use egui::emath;

use palette::rgb::LinSrgba;
use serde::{Deserialize, Serialize};

pub struct CanvasPlugin;
impl Plugin for CanvasPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<CurrentCurve>();
    app.init_resource::<Viewport>();
  }
}

#[derive(Default)]
pub struct CurrentCurve(pub Option<Curve>);

/// in canvas space
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct Curve {
  pub points: Vec<Vec2>,
  pub width: f32,
  pub color: LinSrgba,
}
impl Curve {
  pub fn new(width: f32, color: LinSrgba) -> Self {
    Self {
      points: Vec::new(),
      width,
      color,
    }
  }
}
impl Default for Curve {
  fn default() -> Self {
    Self {
      points: Vec::new(),
      width: 1.0,
      color: LinSrgba::new(1.0, 1.0, 1.0, 1.0),
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
