use crate::{
  canvas::{CurrentCurve, Curve, Viewport},
  toolbox::Toolbox,
};

use bevy::prelude::*;

use bevy_egui::EguiContext;
use egui::emath;

pub struct CanvasUiPlugin;
impl Plugin for CanvasUiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(canvas_ui.system());
  }
}

fn canvas_ui(
  mut commands: Commands,
  egui: Res<EguiContext>,

  mut viewport: ResMut<Viewport>,
  curves: Query<&Curve>,
  mut current: ResMut<CurrentCurve>,
  toolbox: Res<Toolbox>,
) {
  egui::SidePanel::right("canvas_panel").show(egui.ctx(), |ui| {
    use crate::toolbox::ToolMode;

    let (mut response, painter) = ui.allocate_painter(
      ui.available_size_before_wrap(),
      // swallow all iteractions
      egui::Sense {
        click: true,
        drag: true,
        focusable: true,
      },
    );
    painter.rect_filled(response.rect, 0.0, egui::Color32::BLACK);

    let view_to_canvas = viewport.view_to_canvas(response.rect);
    let canvas_to_view = viewport.canvas_to_view(response.rect);

    let all_curves = curves.iter().chain(current.0.as_ref());
    render_curves(painter, canvas_to_view, all_curves);

    match &toolbox.mode {
      ToolMode::Pen => {
        if let Some(pointer_pos) = response.interact_pointer_pos() {
          let current = current.0.get_or_insert(Curve::new(toolbox.stroke));

          let canvas_pos = view_to_canvas * pointer_pos;
          if current.points.last() != Some(&canvas_pos) {
            current.points.push(canvas_pos);
            response.mark_changed();
          }
        } else if let Some(current) = current.0.take() {
          commands.spawn().insert(current);
          response.mark_changed();
        }
      }
      ToolMode::Hand => {
        if egui.ctx().input().pointer.any_down() && response.hovered() {
          let mut delta = egui.ctx().input().pointer.delta();
          delta = delta * view_to_canvas.scale();
          viewport.center -= <[f32; 2]>::from(delta).into();
        }
      }
      ToolMode::Scale => {
        if egui.ctx().input().pointer.any_down() && response.hovered() {
          let mut delta = egui.ctx().input().pointer.delta();
          delta = delta * view_to_canvas.scale();
          viewport.size += delta.y;
        }
      }
    }

    {
      let mut size_delta = egui.ctx().input().scroll_delta;
      size_delta = size_delta * view_to_canvas.scale() * 4.0;
      viewport.size -= size_delta.y;
    }

    if toolbox.undo {
      unimplemented!();
    }

    response
  });
}

fn render_curves<'a>(
  painter: egui::Painter,
  canvas_to_view: emath::RectTransform,
  curves: impl Iterator<Item = &'a Curve>,
) {
  let mut shapes = Vec::new();
  for curve in curves {
    if curve.points.len() >= 2 {
      let points: Vec<egui::Pos2> = curve.points.iter().map(|p| canvas_to_view * *p).collect();
      shapes.push(egui::Shape::line(points, curve.stroke));
    }
  }
  painter.extend(shapes);
}
