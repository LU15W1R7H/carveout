use crate::{
  canvas::{CurrentCurve, Curve, Viewport},
  geometry::{GeometryLine, GeometryPoint},
  util,
};

use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{emath, epaint::CircleShape};

pub(super) struct CanvasUiPlugin;
impl Plugin for CanvasUiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.init_resource::<CanvasUiInfo>();
    app.add_system(canvas_ui_sys.system().label("canvas"));
  }
}

#[derive(Default)]
pub struct CanvasUiInfo {
  pub cursor_canvas_pos: Option<Vec2>,
  pub response: Option<egui::Response>,
}

// canvas_ui needs to be called last,
// because the `CentralPanel` will fill the
// remaining area.
pub(super) fn canvas_ui_sys(
  egui: Res<EguiContext>,
  mut ui_info: ResMut<CanvasUiInfo>,
  viewport: Res<Viewport>,

  current_curve: Res<CurrentCurve>,
  curves: Query<&Curve>,

  points: Query<&GeometryPoint>,
  lines: Query<&GeometryLine>,
) {
  let egui = egui.ctx();

  // reset ui info
  *ui_info = Default::default();
  egui::CentralPanel::default().show(egui, |ui| {
    let (response, mut painter) = ui.allocate_painter(
      ui.available_size_before_wrap(),
      // swallow all iteractions
      egui::Sense {
        click: true,
        drag: true,
        focusable: true,
      },
    );
    // interaction
    ui_info.response = Some(response.clone());

    let view_to_canvas = viewport.view_to_canvas(response.rect);
    let canvas_to_view = viewport.canvas_to_view(response.rect);

    if let Some(pointer_pos) = response.interact_pointer_pos() {
      let canvas_pos = view_to_canvas * pointer_pos;
      let canvas_pos = util::pos_egui2bevy(canvas_pos);
      ui_info.cursor_canvas_pos = Some(canvas_pos);
    }

    // rendering
    painter.rect_filled(response.rect, 0.0, egui::Color32::BLACK);

    let all_curves = curves.iter().chain(current_curve.0.as_ref());
    render_curves(&mut painter, canvas_to_view, all_curves);
    render_points(&mut painter, canvas_to_view, points.iter());
    render_lines(&mut painter, canvas_to_view, lines.iter());

    response
  });
}

fn render_curves<'a>(
  painter: &mut egui::Painter,
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

fn render_points<'a>(
  painter: &mut egui::Painter,
  canvas_to_view: emath::RectTransform,
  points: impl Iterator<Item = &'a GeometryPoint>,
) {
  let mut shapes = Vec::new();
  for point in points {
    let pos = point.pos;
    let mut pos = util::pos_bevy2egui(pos);
    pos = canvas_to_view * pos;
    shapes.push(egui::Shape::Circle(CircleShape::filled(
      pos,
      4.0,
      egui::Color32::BLACK,
    )));
    shapes.push(egui::Shape::Circle(CircleShape::stroke(
      pos,
      5.0,
      egui::Stroke::new(1.0, egui::Color32::WHITE),
    )));
  }
  painter.extend(shapes);
}

fn render_lines<'a>(
  painter: &mut egui::Painter,
  canvas_to_view: emath::RectTransform,
  lines: impl Iterator<Item = &'a GeometryLine>,
) {
  let mut shapes = Vec::new();
  for line in lines {
    let points = [&line.p, &line.q]
      .into_iter()
      .map(|p| p.pos)
      .map(util::pos_bevy2egui)
      .map(|p| canvas_to_view * p)
      .collect();
    shapes.push(egui::Shape::line(
      points,
      egui::Stroke::new(2.0, egui::Color32::WHITE),
    ));
  }
  painter.extend(shapes);
}
