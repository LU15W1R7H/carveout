use crate::canvas::Canvas;

use std::fs;

use bevy::prelude::*;
use bevy_egui::EguiContext;

pub struct SavefilePlugin;
impl Plugin for SavefilePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(ui.system());
  }
}

fn ui(egui: Res<EguiContext>, mut canvas: ResMut<Canvas>) {
  egui::Window::new("Savefile").show(egui.ctx(), |ui| {
    let path = dirs::cache_dir().unwrap().join("canvas.co");

    if ui.button("Load canvas").clicked() {
      match fs::read(path.clone()) {
        Ok(data) => *canvas = bincode::deserialize(&data).unwrap(),
        Err(e) => println!("{}", e),
      }
    }
    if ui.button("Save canvas").clicked() {
      let data = bincode::serialize(&*canvas).unwrap();
      match fs::write(path, data) {
        Ok(()) => (),
        Err(e) => println!("{}", e),
      }
    }
  });
}
