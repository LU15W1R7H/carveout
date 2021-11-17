use crate::{
  canvas::{CurrentCurve, Curve},
  toolbox::Toolbox,
};

use bevy::prelude::*;

use bevy_egui::EguiContext;
use egui::emath;

pub struct CanvasUiPlugin;
impl Plugin for CanvasUiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.init_resource::<CurrentCurve>();
    app.add_system(canvas_ui.system());
  }
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

fn canvas_ui(
  mut commands: Commands,
  curves: Query<&Curve>,
  mut current: ResMut<CurrentCurve>,
  toolbox: Res<Toolbox>,

  mut canvas_ui: Local<CanvasUi>,
  egui: Res<EguiContext>,
) {
  egui::CentralPanel::default().show(egui.ctx(), |ui| {
    use crate::toolbox::ToolMode;

    let (mut response, painter) =
      ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());

    let from = *canvas_ui.canvas_to_screen.from();
    let to = response.rect;
    canvas_ui.canvas_to_screen = emath::RectTransform::from_to(from, to);
    canvas_ui.screen_to_canvas = canvas_ui.canvas_to_screen.inverse();

    let all_curves = curves.iter().chain(current.0.as_ref());
    render_curves(painter, &*canvas_ui, all_curves);

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
          let current = current.0.get_or_insert(Curve::new(toolbox.stroke));

          let canvas_pos = screen_to_canvas * pointer_pos;
          if current.points.last() != Some(&canvas_pos) {
            current.points.push(canvas_pos);
            response.mark_changed();
          }
        } else if let Some(current) = current.0.take() {
          commands.spawn().insert(current);
          response.mark_changed();
        }
      }
    }

    if toolbox.undo {
      unimplemented!();
    }

    response
  });
}

fn render_curves<'a>(
  painter: egui::Painter,
  canvas_ui: &CanvasUi,
  curves: impl Iterator<Item = &'a Curve>,
) {
  let mut shapes = Vec::new();
  for curve in curves {
    if curve.points.len() >= 2 {
      let points: Vec<egui::Pos2> = curve
        .points
        .iter()
        .map(|p| canvas_ui.canvas_to_screen * *p)
        .collect();
      shapes.push(egui::Shape::line(points, curve.stroke));
    }
  }
  painter.extend(shapes);
}
