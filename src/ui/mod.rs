mod canvas;
mod misc;
mod sidebar;

use crate::{
  canvas::{CurrentCurve, Curve, Viewport},
  toolbox::Toolbox,
};

use crate::savefile::{LoadFileEvent, SaveFileEvent};

use bevy::prelude::*;

use bevy_egui::EguiContext;

pub struct UiPlugin;
impl Plugin for UiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(ui_sys.system());
  }
}

fn ui_sys(
  commands: Commands,

  egui: Res<EguiContext>,
  viewport: ResMut<Viewport>,

  curves: Query<&Curve>,
  mut current: ResMut<CurrentCurve>,
  mut toolbox: ResMut<Toolbox>,

  load_file_event: EventWriter<LoadFileEvent>,
  save_file_event: EventWriter<SaveFileEvent>,
) {
  sidebar::sidebar_ui(egui.ctx(), &mut toolbox, load_file_event, save_file_event);

  canvas::canvas_ui(
    commands,
    egui.ctx(),
    viewport,
    curves,
    &mut current,
    &mut toolbox,
  );
}
