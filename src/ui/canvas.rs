use crate::{
  canvas::{CurrentCurve, Curve, Viewport},
  toolbox::Toolbox,
  util,
};

use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::emath;

// canvas_ui needs to be called last,
// because the `CentralPanel` will fill the
// remaining area.
pub(super) fn canvas_ui_sys(
  mut commands: Commands,

  egui: Res<EguiContext>,
  mut viewport: ResMut<Viewport>,

  curves: Query<&Curve>,
  mut current: ResMut<CurrentCurve>,
  toolbox: Res<Toolbox>,
) {
  let egui = egui.ctx();
  egui::CentralPanel::default().show(egui, |ui| {
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
          let curve_width = view_to_canvas.scale().length() * toolbox.curve_width;
          let current = current
            .0
            .get_or_insert(Curve::new(curve_width, toolbox.curve_color));

          let canvas_pos = view_to_canvas * pointer_pos;
          let canvas_pos = util::pos_egui2bevy(canvas_pos);
          let last = current.points.last();
          if last != Some(&canvas_pos) {
            current.points.push(canvas_pos);
            response.mark_changed();
          }
        } else if let Some(current) = current.0.take() {
          commands.spawn().insert(current);
          response.mark_changed();
        }
      }
      ToolMode::Hand => {
        if egui.input().pointer.any_down() && response.hovered() {
          let mut delta = egui.input().pointer.delta();
          delta = delta * view_to_canvas.scale();
          viewport.center -= <[f32; 2]>::from(delta).into();
        }
      }
      ToolMode::Scale => {
        if egui.input().pointer.any_down() && response.hovered() {
          let mut delta = egui.input().pointer.delta();
          delta = delta * view_to_canvas.scale();
          viewport.size += delta.y;
        }
      }
    }

    {
      let mut size_delta = egui.input().scroll_delta;
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
      let curve_width = canvas_to_view.scale().length() * curve.width;
      let stroke = egui::Stroke::new(curve_width, util::color_palette2egui(curve.color));
      let points: Vec<egui::Pos2> = curve
        .points
        .iter()
        .cloned()
        .map(util::pos_bevy2egui)
        .map(|p| canvas_to_view * p)
        .collect();
      shapes.push(egui::Shape::line(points, stroke));
    }
  }
  painter.extend(shapes);
}
