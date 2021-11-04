use crate::toolbox::Toolbox;

use bevy::prelude::*;
use bevy_egui::EguiContext;

use serde::{Deserialize, Serialize};

pub struct CanvasPlugin;
impl Plugin for CanvasPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(start_up.system())
      .add_system(ui.system());
  }
}

#[derive(Serialize, Deserialize)]
pub struct Curve {
  /// in 0-1 normalized coordinates
  points: Vec<egui::Pos2>,
  stroke: egui::Stroke,
}
impl Curve {
  fn new(stroke: egui::Stroke) -> Self {
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

#[derive(Default, Serialize, Deserialize)]
pub struct Canvas {
  curves: Vec<Curve>,
  current: Option<Curve>,
}

fn start_up(mut commands: Commands) {
  commands.insert_resource(Canvas::default());
}

fn ui(egui: Res<EguiContext>, mut canvas: ResMut<Canvas>, toolbox: Res<Toolbox>) {
  egui::Window::new("Drawing").show(egui.ctx(), |ui| {
    use egui::{emath, Pos2, Rect, Sense};

    ui.visuals_mut().extreme_bg_color = egui::Color32::BLACK;

    let (mut response, painter) =
      ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    let to_screen = emath::RectTransform::from_to(
      Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
      response.rect,
    );
    let from_screen = to_screen.inverse();

    if let Some(pointer_pos) = response.interact_pointer_pos() {
      let current = canvas.current.get_or_insert(Curve::new(toolbox.stroke));

      let canvas_pos = from_screen * pointer_pos;
      if current.points.last() != Some(&canvas_pos) {
        current.points.push(canvas_pos);
        response.mark_changed();
      }
    } else if let Some(current) = canvas.current.take() {
      canvas.curves.push(current);
      response.mark_changed();
    }

    let mut shapes = Vec::new();
    for curve in canvas.curves.iter().chain(canvas.current.as_ref()) {
      if curve.points.len() >= 2 {
        let points: Vec<Pos2> = curve.points.iter().map(|p| to_screen * *p).collect();
        shapes.push(egui::Shape::line(points, curve.stroke));
      }
    }
    painter.extend(shapes);

    response
  });
}
