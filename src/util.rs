#![allow(dead_code)]

pub fn pos_egui2bevy(egui: egui::Pos2) -> bevy::math::Vec2 {
  bevy::math::Vec2::new(egui.x, egui.y)
}

pub fn pos_bevy2egui(bevy: bevy::math::Vec2) -> egui::Pos2 {
  egui::Pos2::new(bevy.x, bevy.y)
}

pub fn vec_egui2bevy(egui: egui::Vec2) -> bevy::math::Vec2 {
  bevy::math::Vec2::new(egui.x, egui.y)
}

pub fn vec_bevy2egui(bevy: bevy::math::Vec2) -> egui::Vec2 {
  egui::Vec2::new(bevy.x, bevy.y)
}

pub fn color_palette2egui(palette: palette::LinSrgba) -> egui::color::Rgba {
  // TODO: is premultiplied correct?
  egui::color::Rgba::from_rgba_premultiplied(
    palette.red,
    palette.green,
    palette.blue,
    palette.alpha,
  )
}

pub fn color_egui2palette(egui: egui::Rgba) -> palette::LinSrgba {
  palette::LinSrgba::new(egui.r(), egui.g(), egui.b(), egui.a())
}

pub fn color_array2palette(array: [f32; 4]) -> palette::LinSrgba {
  palette::LinSrgba::new(array[0], array[1], array[2], array[3])
}

pub fn color_palette2array(palette: palette::LinSrgba) -> [f32; 4] {
  [palette.red, palette.green, palette.blue, palette.alpha]
}
