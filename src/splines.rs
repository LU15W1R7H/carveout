use bevy::prelude::*;

use bevy_egui::{egui, EguiContext};

use enterpolation::{bspline::BSpline, Curve};

pub struct SplinesPlugin;
impl Plugin for SplinesPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(start_up.system())
      .add_system(ui.system());
  }
}

pub struct Spline {
  stroke: egui::Stroke,
  elements: Vec<f32>,
}

impl Spline {
  pub fn new() -> Self {
    Self {
      stroke: egui::Stroke::new(3.0, egui::Color32::GREEN),
      elements: vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0],
    }
  }
}

fn start_up(mut commands: Commands) {
  commands.insert_resource(Spline::new());
}

fn ui(egui: Res<EguiContext>, mut splines: ResMut<Spline>) {
  egui::Window::new("BSpline Control").show(egui.ctx(), |ui| {
    ui.label("Number of elements");
    let mut nelements = splines.elements.len();
    ui.add(egui::DragValue::new(&mut nelements));
    splines.elements.resize(nelements, 1.0);
    ui.separator();
    for i in 0..nelements {
      ui.add(egui::DragValue::new(&mut splines.elements[i]).speed(0.1));
    }
  });

  let spline = BSpline::builder()
    .elements(splines.elements.clone())
    .knots(vec![-2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0])
    .dynamic()
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

  let domain = min..=max;

  egui::Window::new("BSpline").show(egui.ctx(), |ui| {
    use egui::{emath, Pos2, Rect, Sense};

    let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

    let to_screen =
      emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, domain), response.rect);

    let mut shapes = vec![];
    let points: Vec<Pos2> = points.iter().map(|p| to_screen * *p).collect();
    shapes.push(egui::Shape::line(points, splines.stroke));
    painter.extend(shapes);

    response
  });
}
