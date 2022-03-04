use crate::canvas::Curve;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

pub struct SavefilePlugin;
impl Plugin for SavefilePlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<LoadFileEvent>();
    app.add_event::<SaveFileEvent>();
    app.add_system(load_sys.system());
    app.add_system(save_sys.system());
  }
}

pub struct LoadFileEvent;
pub struct SaveFileEvent;

fn load_sys(
  mut event: EventReader<LoadFileEvent>,
  mut commands: Commands,
  curves: Query<(Entity, &Curve)>,
) {
  if event.iter().next().is_none() {
    return;
  }

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
      let reader = flexbuffers::Reader::get_root(data.as_ref()).unwrap();
      let curves: Vec<Curve> = Deserialize::deserialize(reader).unwrap();
      curves.into_iter().for_each(|c| {
        commands.spawn().insert(c);
      });
    }
    Err(e) => println!("{}", e),
  }
}

fn save_sys(mut event: EventReader<SaveFileEvent>, curves: Query<&Curve>) {
  if event.iter().next().is_none() {
    return;
  }

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

  let mut serializer = flexbuffers::FlexbufferSerializer::new();
  let curves: Vec<Curve> = curves.iter().cloned().collect();
  curves.serialize(&mut serializer).unwrap();
  let data = serializer.take_buffer();
  match fs::write(path, data) {
    Ok(()) => (),
    Err(e) => println!("{}", e),
  }
}

const EXTENSION: &str = "co";
const DEFAULT_FILE_NAME: &str = "carveout.co";
fn default_dir_path() -> Option<PathBuf> {
  dirs::cache_dir()
}
