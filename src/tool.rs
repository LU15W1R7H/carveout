use bevy::prelude::*;

use crate::{
  canvas::{CurrentCurve, Curve, Viewport},
  geometry::{GeometryLine, GeometryPoint},
  ui::canvas::CanvasUiInfo,
  util,
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

#[derive(Default)]
struct EraserToolSys {
  last_pos: Option<Vec2>,
}
fn eraser_tool_sys(
  mut local: Local<EraserToolSys>,
  mut commands: Commands,
  cursor_tool: Res<CursorTool>,
  ui: Res<CanvasUiInfo>,
  curves: Query<(Entity, &Curve)>,
) {
  match &cursor_tool.specific {
    SpecificTool::Eraser => {}
    _ => return,
  }
  match [local.last_pos, ui.cursor_canvas_pos] {
    [Some(e0), Some(e1)] => {
      let e01 = e1 - e0;
      'CURVE_LOOP: for (curve_entity, curve) in curves.iter() {
        for [c0, c1] in curve.points.array_windows::<2>().copied() {
          let c01 = c1 - c0;

          if are_segments_intersecting(e0, e01, c0, c01) {
            commands.entity(curve_entity).despawn();
            continue 'CURVE_LOOP;
          }
        }
      }
    }
    _ => {}
  };
  local.last_pos = ui.cursor_canvas_pos;
}
fn are_segments_intersecting(p0: Vec2, pv: Vec2, q0: Vec2, qv: Vec2) -> bool {
  let pvqv_cross = pv.perp_dot(qv);
  if pvqv_cross.abs() > f32::EPSILON {
    let range01 = 0.0..=1.0;
    let p0q0 = q0 - p0;
    let t = p0q0.perp_dot(pv) / pvqv_cross;
    let s = p0q0.perp_dot(qv) / pvqv_cross;
    range01.contains(&t) && range01.contains(&s)
  } else {
    false
  }
}

#[derive(Default)]
struct HandToolSys {
  last_pos: Option<egui::Pos2>,
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
  match (
    ui.response.as_ref().and_then(|r| r.interact_pointer_pos()),
    local.last_pos,
    ui.view_to_canvas,
  ) {
    (Some(pos), Some(last), Some(view_to_canvas)) => {
      let mut delta = last - pos;
      delta = view_to_canvas.scale() * delta;
      viewport.center += util::vec_egui2bevy(delta);
    }
    _ => {}
  };
  local.last_pos = ui.response.as_ref().and_then(|r| r.interact_pointer_pos());
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
  match [ui.cursor_canvas_pos, local.last_pos] {
    [Some(pos), Some(last)] => {
      let delta = pos - last;
      viewport.center -= delta;
    }
    _ => {}
  };
  local.last_pos = ui.cursor_canvas_pos;
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
