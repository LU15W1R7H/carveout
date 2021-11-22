use crate::{
  toolbox::ToolMode,
  ui::{LoadFileEvent, SaveFileEvent, Toolbox},
};

use bevy::prelude::*;

use palette::LinSrgba;

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
        let color = toolbox.curve_color;
        let mut color = [color.red, color.green, color.blue, color.alpha];
        // TODO: is it premultiplied?
        ui.color_edit_button_rgba_premultiplied(&mut color);
        let color = LinSrgba::new(color[0], color[1], color[2], color[3]);
        toolbox.curve_color = color;
        ui.label("Pen stroke");
        ui.add(egui::Slider::new(&mut toolbox.curve_width, 0.0..=10.0));
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
