use crate::toolbox::Toolbox;

use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::emath;

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

#[derive(Default, Serialize, Deserialize)]
pub struct Curves {
  pub curves: Vec<Curve>,
  pub current: Option<Curve>,
}

pub struct CanvasUi {
  pub canvas_to_screen: emath::RectTransform,
  pub screen_to_canvas: emath::RectTransform,
}

impl Default for CanvasUi {
  fn default() -> Self {
    Self {
      canvas_to_screen: emath::RectTransform::identity(emath::Rect::from_center_size(
        emath::pos2(0.0, 0.0),
        emath::vec2(1.0, 1.0),
      )),
      screen_to_canvas: emath::RectTransform::identity(emath::Rect::from_center_size(
        emath::pos2(0.0, 0.0),
        emath::vec2(1.0, 1.0),
      )),
    }
  }
}

fn start_up(mut commands: Commands) {
  commands.insert_resource(Curves::default());
  commands.insert_resource(CanvasUi::default());
}

fn ui(mut canvas_ui: ResMut<CanvasUi>, egui: Res<EguiContext>, mut curves: ResMut<Curves>, toolbox: Res<Toolbox>) {
  egui::Window::new("Drawing").show(egui.ctx(), |ui| {
    use crate::toolbox::ToolMode;
    use egui::{Pos2, Sense};

    let (mut response, painter) =
      ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    match &toolbox.mode {
      ToolMode::Hand => {
        if egui.ctx().input().pointer.any_down() && response.hovered() {
          let mut delta = egui.ctx().input().pointer.delta();
          delta = delta * canvas_ui.screen_to_canvas.scale();

          let mut from = *canvas_ui.canvas_to_screen.from();
          from = from.translate(-delta);
          let to = response.rect;
          canvas_ui.canvas_to_screen = emath::RectTransform::from_to(from, to);
          canvas_ui.screen_to_canvas = canvas_ui.canvas_to_screen.inverse();
        }
      }
      ToolMode::Scale => {
        if egui.ctx().input().pointer.any_down() && response.hovered() {
          let mut delta = egui.ctx().input().pointer.delta();
          delta = delta * canvas_ui.screen_to_canvas.scale();
          delta = egui::Vec2::splat(delta.y);

          let mut from = *canvas_ui.canvas_to_screen.from();
          from = from.expand2(delta);
          let to = response.rect;
          canvas_ui.canvas_to_screen = emath::RectTransform::from_to(from, to);
          canvas_ui.screen_to_canvas = canvas_ui.canvas_to_screen.inverse();
        }
      }
      ToolMode::Pen => {
        let screen_to_canvas = canvas_ui.screen_to_canvas;
        if let Some(pointer_pos) = response.interact_pointer_pos() {
          let current = curves.current.get_or_insert(Curve::new(toolbox.stroke));

          let canvas_pos = screen_to_canvas * pointer_pos;
          if current.points.last() != Some(&canvas_pos) {
            current.points.push(canvas_pos);
            response.mark_changed();
          }
        } else if let Some(current) = curves.current.take() {
          curves.curves.push(current);
          response.mark_changed();
        }
      }
    }

    if toolbox.undo {
      let _ = curves.curves.pop();
    }

    // render lines
    let mut shapes = Vec::new();
    for curve in curves.curves.iter().chain(curves.current.as_ref()) {
      if curve.points.len() >= 2 {
        let points: Vec<Pos2> = curve
          .points
          .iter()
          .map(|p| canvas_ui.canvas_to_screen * *p)
          .collect();
        shapes.push(egui::Shape::line(points, curve.stroke));
      }
    }
    painter.extend(shapes);

    response
  });
}
