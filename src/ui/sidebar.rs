use crate::{
  savefile::{LoadFileEvent, SaveFileEvent},
  tool::{CursorTool, SpecificTool},
  util,
};

use bevy::prelude::*;
use bevy_egui::EguiContext;

pub(super) fn sidebar_ui_sys(
  mut egui: ResMut<EguiContext>,

  mut cursor_tool: ResMut<CursorTool>,
  mut load_file_event: EventWriter<LoadFileEvent>,
  mut save_file_event: EventWriter<SaveFileEvent>,
) {
  let egui = egui.ctx_mut();

  egui::SidePanel::left("toolbox_panel").show(egui, |ui| {
    ui.add_space(10.0);
    ui.add(egui::Label::new(
      egui::RichText::new("📦 Toolbox").text_style(egui::TextStyle::Heading),
    ));
    ui.add_space(20.0);

    ui.group(|ui| {
      ui.label("Tools");
      ui.horizontal_wrapped(|ui| {
        selectable_tool(
          ui,
          &mut cursor_tool.specific,
          SpecificTool::Pen(Default::default()),
          "✏",
        );
        selectable_tool(ui, &mut cursor_tool.specific, SpecificTool::Hand, "✋");
        selectable_tool(ui, &mut cursor_tool.specific, SpecificTool::Scale, "↕");
        selectable_tool(
          ui,
          &mut cursor_tool.specific,
          SpecificTool::PointPlacer,
          "◎",
        );
        selectable_tool(ui, &mut cursor_tool.specific, SpecificTool::LinePlacer, "∕");
      });
    });

    ui.group(|ui| {
      ui.label("Action");
      // TODO: UndoEventWriter
    });

    match &mut cursor_tool.specific {
      SpecificTool::Pen(pen) => {
        ui.group(|ui| {
          ui.label("Pen color");
          let mut color = util::color_palette2array(pen.color);
          ui.color_edit_button_rgba_premultiplied(&mut color);
          pen.color = util::color_array2palette(color);
          ui.label("Pen stroke");
          ui.add(egui::Slider::new(&mut pen.size, 0.0..=10.0));
        });
      }
      _ => (),
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

fn selectable_tool(
  ui: &mut egui::Ui,
  current: &mut SpecificTool,
  selected: SpecificTool,
  text: &str,
) {
  if ui
    .add(egui::SelectableLabel::new(
      variant_eq(current, &selected),
      text,
    ))
    .clicked()
  {
    *current = selected;
  }
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
  std::mem::discriminant(a) == std::mem::discriminant(b)
}
