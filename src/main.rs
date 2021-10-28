use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use enterpolation::{bspline::BSpline, Curve};

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_startup_system(init.system())
    .add_system(ui.system())
    .add_system(spline_system.system())
    .run();
}

fn init(mut commands: Commands) {
  commands.insert_resource(Painting::new());
  commands.insert_resource(Spline::new());
}

fn ui(egui_context: ResMut<EguiContext>, mut painting: ResMut<Painting>) {
  egui::Window::new("Welcome").show(egui_context.ctx(), |ui| {
    ui.label("Welcome to Carveout.");
    ui.separator();
    ui.label("A tool for modern scientific digital pen note taking.");
  });

  egui::Window::new("Drawing").show(egui_context.ctx(), |ui| {
    use egui::{emath, Pos2, Rect, Sense};
    let (mut response, painter) =
      ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    let to_screen = emath::RectTransform::from_to(
      Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
      response.rect,
    );
    let from_screen = to_screen.inverse();

    if painting.lines.is_empty() {
      painting.lines.push(vec![]);
    }

    let current_line = painting.lines.last_mut().unwrap();

    if let Some(pointer_pos) = response.interact_pointer_pos() {
      let canvas_pos = from_screen * pointer_pos;
      if current_line.last() != Some(&canvas_pos) {
        current_line.push(canvas_pos);
        response.mark_changed();
      }
    } else if !current_line.is_empty() {
      painting.lines.push(vec![]);
      response.mark_changed();
    }

    let mut shapes = vec![];
    for line in &painting.lines {
      if line.len() >= 2 {
        let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
        shapes.push(egui::Shape::line(points, painting.stroke));
      }
    }
    painter.extend(shapes);

    response
  });
}

pub struct Painting {
  /// in 0-1 normalized coordinates
  lines: Vec<Vec<egui::Pos2>>,
  stroke: egui::Stroke,
}
impl Painting {
  fn new() -> Self {
    Self {
      stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
      lines: Vec::new(),
    }
  }
}

fn spline_system(egui_context: ResMut<EguiContext>, splines: Res<Spline>) {
  egui::Window::new("BSpline").show(egui_context.ctx(), |ui| {
    use egui::{emath, Pos2, Rect, Sense};

    let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    let to_screen = emath::RectTransform::from_to(
      Rect::from_x_y_ranges(0.0..=1.0, splines.domain.clone()),
      response.rect,
    );

    let mut shapes = vec![];
    let points: Vec<Pos2> = splines
      .points
      .iter()
      .cloned()
      .map(|p| to_screen * p)
      .collect();
    shapes.push(egui::Shape::line(points, splines.stroke));
    painter.extend(shapes);

    response
  });
}

struct Spline {
  points: Vec<egui::Pos2>,
  stroke: egui::Stroke,
  domain: RangeInclusive<f32>,
}

impl Spline {
  fn new() -> Self {
    let spline = BSpline::builder()
      .clamped()
      .elements([0.0, 0.0, 1.0, 6.0, 0.0, -3.0, 0.0])
      .equidistant::<f64>() // evenly spaced knots
      .degree(3)
      .domain(-2.0, 2.0)
      .constant::<4>() // degree + 1
      .build()
      .unwrap();

    let nsamples = 100;
    let points: Vec<egui::Pos2> = spline
      .take(nsamples)
      .enumerate()
      .map(|(i, v)| egui::Pos2::new(i as f32 / nsamples as f32, v as f32))
      .collect();
    let min = points
      .iter()
      .map(|p| p.y)
      .min_by(|a, b| PartialOrd::partial_cmp(a, b).unwrap())
      .unwrap();
    let max = points
      .iter()
      .map(|p| p.y)
      .max_by(|a, b| PartialOrd::partial_cmp(a, b).unwrap())
      .unwrap();

    Self {
      points,
      stroke: egui::Stroke::new(3.0, egui::Color32::GREEN),
      domain: min..=max,
    }
  }
}
