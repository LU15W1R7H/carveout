use crate::canvas::Curve;

use std::{fs, path::PathBuf};

use bevy::prelude::*;
use bevy_egui::EguiContext;

pub struct SavefilePlugin;
impl Plugin for SavefilePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(ui.system());
  }
}

const EXTENSION: &str = "co";
const DEFAULT_FILE_NAME: &str = "carveout.co";
fn default_dir_path() -> Option<PathBuf> {
  dirs::cache_dir()
}

fn ui(egui: Res<EguiContext>, commands: Commands, curves: Query<(Entity, &Curve)>) {
  egui::Window::new("Savefile").show(egui.ctx(), |ui| {
    if ui.button("📝").clicked() {
      save_file(curves);
    } else if ui.button("📂").clicked() {
      load_file(commands, curves);
    }
  });
}

fn save_file(curves: Query<(Entity, &Curve)>) {
  let mut dialog = rfd::FileDialog::new()
    .set_title("Save carveout file")
    .set_file_name(DEFAULT_FILE_NAME)
    .add_filter("carveout", &["co"]);

  match default_dir_path() {
    Some(dir) => dialog = dialog.set_directory(dir),
    None => {}
  }

  let mut path = match dialog.save_file() {
    Some(p) => p,
    None => return,
  };

  path.set_extension(EXTENSION);

  let curves: Vec<Curve> = curves.iter().map(|c| c.1).cloned().collect();
  let data = bincode::serialize(&*curves).unwrap();
  match fs::write(path, data) {
    Ok(()) => (),
    Err(e) => println!("{}", e),
  }
}

fn load_file(mut commands: Commands, curves: Query<(Entity, &Curve)>) {
  let mut dialog = rfd::FileDialog::new()
    .set_title("Load carveout file")
    .add_filter("carveout", &["co"]);

  match default_dir_path() {
    Some(dir) => dialog = dialog.set_directory(dir),
    None => {}
  }

  let path = match dialog.pick_file() {
    Some(p) => p,
    None => return,
  };

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
