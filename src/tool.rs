use bevy::prelude::*;

use crate::{
  canvas::{CurrentCurve, Curve, Viewport},
  geometry::{GeometryLine, GeometryPoint},
  ui::canvas::CanvasUiInfo,
};

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<CursorTool>();
    app.add_system(pen_tool_sys.system());
    app.add_system(eraser_tool_sys.system());
    app.add_system(hand_tool_sys.system());
    app.add_system(scale_tool_sys.system());
    app.add_system(point_placer_tool_sys.system());
    app.add_system(line_placer_tool_sys.system());
  }
}

pub struct CursorTool {
  pub canvas_pos: Vec2,
  pub active: bool,
  pub specific: SpecificTool,
}
impl Default for CursorTool {
  fn default() -> Self {
    Self {
      canvas_pos: Default::default(),
      active: false,
      specific: SpecificTool::Pen(Default::default()),
    }
  }
}

pub enum SpecificTool {
  Pen(PenTool),
  Eraser,
  Hand,
  Scale,
  PointPlacer,
  LinePlacer,
}

pub struct PenTool {
  pub size: f32,
  pub color: palette::LinSrgba,
}
impl Default for PenTool {
  fn default() -> Self {
    Self {
      size: 1.0,
      color: palette::LinSrgba::new(1.0, 1.0, 1.0, 1.0),
    }
  }
}

//fn generic_tool_sys() {
//}

fn pen_tool_sys(
  mut commands: Commands,
  cursor_tool: Res<CursorTool>,
  ui: Res<CanvasUiInfo>,
  viewport: Res<Viewport>,
  mut current_curve: ResMut<CurrentCurve>,
) {
  let pen = match &cursor_tool.specific {
    SpecificTool::Pen(pen) => pen,
    _ => return,
  };
  let response = match &ui.response {
    Some(response) => response,
    None => return,
  };
  match ui.cursor_canvas_pos {
    Some(pos) => {
      let curve_width = viewport.view_to_canvas(response.rect).scale().length() * pen.size;
      let current = current_curve
        .0
        .get_or_insert(crate::canvas::Curve::new(curve_width, pen.color));

      let last = current.points.last();
      if last != Some(&pos) {
        current.points.push(pos);
        //response.mark_changed();
      }
    }
    None => match current_curve.0.take() {
      Some(c) => {
        commands.spawn().insert(c);
        // response.mark_changed();
      }
      None => {}
    },
  };
}

const ERASER_RADIUS: f32 = 0.01;
fn eraser_tool_sys(
  mut commands: Commands,
  cursor_tool: Res<CursorTool>,
  ui: Res<CanvasUiInfo>,
  curves: Query<(Entity, &Curve)>,
) {
  match &cursor_tool.specific {
    SpecificTool::Eraser => {}
    _ => return,
  }
  match ui.cursor_canvas_pos {
    Some(cursor_pos) => {
      'CURVE_LOOP: for (curve_entity, curve) in curves.iter() {
        for curve_point in &curve.points {
          let diff = *curve_point - cursor_pos;
          let dist = diff.length_squared();
          if dist <= ERASER_RADIUS.powi(2) {
            commands.entity(curve_entity).despawn();
            continue 'CURVE_LOOP;
          }
        }
      }
    }
    None => {}
  };
}

#[derive(Default)]
struct HandToolSys {
  last_pos: Option<Vec2>,
}
fn hand_tool_sys(
  mut local: Local<HandToolSys>,
  cursor_tool: Res<CursorTool>,
  ui: Res<CanvasUiInfo>,
  mut viewport: ResMut<Viewport>,
) {
  match &cursor_tool.specific {
    SpecificTool::Hand => {}
    _ => return,
  };
  match ui.cursor_canvas_pos {
    Some(pos) => {
      match local.last_pos {
        Some(last) => {
          let delta = pos - last;
          viewport.center -= delta;
        }
        _ => {}
      }
      local.last_pos = Some(pos);
    }
    None => {
      local.last_pos = None;
    }
  };
}

#[derive(Default)]
struct ScaleToolSys {
  last_pos: Option<Vec2>,
}
fn scale_tool_sys(
  mut local: Local<ScaleToolSys>,
  cursor_tool: Res<CursorTool>,
  ui: Res<CanvasUiInfo>,
  mut viewport: ResMut<Viewport>,
) {
  match &cursor_tool.specific {
    SpecificTool::Scale => {}
    _ => return,
  };
  match ui.cursor_canvas_pos {
    Some(pos) => {
      match local.last_pos {
        Some(last) => {
          let delta = last - pos;
          viewport.size += delta.y;
        }
        _ => {}
      }
      local.last_pos = Some(pos);
    }
    None => {
      local.last_pos = None;
    }
  };
}

fn point_placer_tool_sys(
  cursor_tool: Res<CursorTool>,
  mut commands: Commands,
  ui: Res<CanvasUiInfo>,
) {
  match &cursor_tool.specific {
    SpecificTool::PointPlacer => {}
    _ => return,
  };
  match ui.cursor_canvas_pos {
    Some(pos) => {
      commands.spawn().insert(GeometryPoint::new(pos));
    }
    _ => {}
  }
}

#[derive(Default)]
struct LinePlacerToolSys {
  p: Option<GeometryPoint>,
}
fn line_placer_tool_sys(
  mut local: Local<LinePlacerToolSys>,
  cursor_tool: Res<CursorTool>,
  mut commands: Commands,
  ui: Res<CanvasUiInfo>,
) {
  match &cursor_tool.specific {
    SpecificTool::LinePlacer => {}
    _ => return,
  };
  match ui.cursor_canvas_pos {
    Some(pos) => match local.p.take() {
      Some(p) => {
        commands
          .spawn()
          .insert(GeometryLine::new(p, GeometryPoint::new(pos)));
      }
      None => local.p = Some(GeometryPoint::new(pos)),
    },
    _ => {}
  }
}
