use crate::{
  savefile::{LoadFileEvent, SaveFileEvent},
  toolbox::{ToolMode, Toolbox},
};

use bevy::prelude::*;
use bevy_egui::EguiContext;

pub struct SidebarUiPlugin;
impl Plugin for SidebarUiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(sidebar_ui.system());
  }
}

fn sidebar_ui(
  egui: Res<EguiContext>,
  mut toolbox: ResMut<Toolbox>,
  mut load_file_event: EventWriter<LoadFileEvent>,
  mut save_file_event: EventWriter<SaveFileEvent>,
) {
  egui::SidePanel::left("toolbox_panel").show(egui.ctx(), |ui| {
    ui.add(egui::Label::new("📦 Toolbox").strong());
    ui.separator();
    ui.add(egui::Separator::default().spacing(0.01));

    ui.label("Tools");
    ui.selectable_value(&mut toolbox.mode, ToolMode::Pen, "✏");
    ui.selectable_value(&mut toolbox.mode, ToolMode::Hand, "✋");
    ui.selectable_value(&mut toolbox.mode, ToolMode::Scale, "↕");

    ui.separator();

    ui.label("Action");
    toolbox.undo = ui.button("↩").clicked();

    if toolbox.mode == ToolMode::Pen {
      ui.separator();
      ui.label("Pen color");
      let color = &mut toolbox.stroke.color;
      ui.color_edit_button_srgba(color);
      ui.label("Pen stroke");
      let width = &mut toolbox.stroke.width;
      ui.add(egui::Slider::new(width, 0.0..=10.0));
    }

    ui.separator();

    ui.label("Save and Load");
    if ui.button("📂").clicked() {
      load_file_event.send(LoadFileEvent);
    } else if ui.button("📝").clicked() {
      save_file_event.send(SaveFileEvent);
    }
  });
}
