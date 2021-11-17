use crate::canvas::Curve;

use std::{fs, path::PathBuf};

use bevy::prelude::*;

const EXTENSION: &str = "co";
const DEFAULT_FILE_NAME: &str = "carveout.co";
fn default_dir_path() -> Option<PathBuf> {
  dirs::cache_dir()
}

pub fn save_file(curves: Query<(Entity, &Curve)>) {
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

pub fn load_file(mut commands: Commands, curves: Query<(Entity, &Curve)>) {
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
