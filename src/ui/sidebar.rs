use crate::{
  toolbox::ToolMode,
  ui::{LoadFileEvent, SaveFileEvent, Toolbox},
};

use bevy::prelude::*;

pub(super) fn sidebar_ui(
  egui: &egui::CtxRef,

  toolbox: &mut Toolbox,
  mut load_file_event: EventWriter<LoadFileEvent>,
  mut save_file_event: EventWriter<SaveFileEvent>,
) {
  egui::SidePanel::left("toolbox_panel").show(egui, |ui| {
    ui.add_space(10.0);
    ui.add(egui::Label::new("📦 Toolbox").text_style(egui::TextStyle::Heading));
    ui.add_space(20.0);

    ui.group(|ui| {
      ui.label("Tools");
      ui.horizontal_wrapped(|ui| {
        ui.selectable_value(&mut toolbox.mode, ToolMode::Pen, "✏");
        ui.selectable_value(&mut toolbox.mode, ToolMode::Hand, "✋");
        ui.selectable_value(&mut toolbox.mode, ToolMode::Scale, "↕");
      });
    });

    ui.group(|ui| {
      ui.label("Action");
      toolbox.undo = ui.button("↩").clicked();
    });

    if toolbox.mode == ToolMode::Pen {
      ui.group(|ui| {
        ui.label("Pen color");
        let color = &mut toolbox.stroke.color;
        ui.color_edit_button_srgba(color);
        ui.label("Pen stroke");
        let width = &mut toolbox.stroke.width;
        ui.add(egui::Slider::new(width, 0.0..=10.0));
      });
    }

    ui.group(|ui| {
      ui.label("Save and Load");
      ui.horizontal_wrapped(|ui| {
        if ui.button("📂").clicked() {
          load_file_event.send(LoadFileEvent);
        } else if ui.button("📝").clicked() {
          save_file_event.send(SaveFileEvent);
        }
      });
    });
  });
}
