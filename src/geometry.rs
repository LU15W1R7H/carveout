use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct GeometryPoint {
  /// canvas space pos
  pub pos: Vec2,
}
impl GeometryPoint {
  pub fn new(pos: Vec2) -> Self {
    Self { pos }
  }
}

#[derive(Default, Clone)]
pub struct GeometryLine {
  pub p: GeometryPoint,
  pub q: GeometryPoint,
}
impl GeometryLine {
  pub fn new(p: GeometryPoint, q: GeometryPoint) -> Self {
    Self { p, q }
  }
}
