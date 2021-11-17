use crate::{
  canvas::Curve,
  savefile,
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
  commands: Commands,
  egui: Res<EguiContext>,
  mut toolbox: ResMut<Toolbox>,
  curves: Query<(Entity, &Curve)>,
) {
  egui::SidePanel::left("toolbox_panel").show(egui.ctx(), |ui| {
    ui.add(egui::Label::new("📦 Toolbox").strong());
    ui.separator();
    ui.separator();

    ui.label("Tools");
    ui.selectable_value(&mut toolbox.mode, ToolMode::Hand, "✋");
    ui.selectable_value(&mut toolbox.mode, ToolMode::Scale, "↕");
    ui.selectable_value(&mut toolbox.mode, ToolMode::Pen, "✏");

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

    if ui.button("📝").clicked() {
      savefile::save_file(curves);
    } else if ui.button("📂").clicked() {
      savefile::load_file(commands, curves);
    }
  });
}
