use crate::canvas::Curve;

use std::{fs, path::Path};

use bevy::prelude::*;
use bevy_egui::EguiContext;

pub struct SavefilePlugin;
impl Plugin for SavefilePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(ui.system());
  }
}

fn ui(egui: Res<EguiContext>, commands: Commands, curves: Query<(Entity, &Curve)>) {
  egui::Window::new("Savefile").show(egui.ctx(), |ui| {
    let path = dirs::cache_dir().unwrap().join("canvas.co");

    if ui.button("Load canvas").clicked() {
      load_file(path, commands, curves);
    } else if ui.button("Save canvas").clicked() {
      save_file(path, curves);
    }
  });
}

fn save_file(path: impl AsRef<Path>, curves: Query<(Entity, &Curve)>) {
  let curves: Vec<Curve> = curves.iter().map(|c| c.1).cloned().collect();
  let data = bincode::serialize(&*curves).unwrap();
  match fs::write(path, data) {
    Ok(()) => (),
    Err(e) => println!("{}", e),
  }
}

fn load_file(path: impl AsRef<Path>, mut commands: Commands, curves: Query<(Entity, &Curve)>) {
  match fs::read(path) {
    Ok(data) => {
      curves
        .iter()
        .for_each(|c| commands.entity(c.0).despawn_recursive());
      let curves: Vec<Curve> = bincode::deserialize(&data).unwrap();
      curves.into_iter().for_each(|c| {
        commands.spawn().insert(c);
      });
    }
    Err(e) => println!("{}", e),
  }
}
