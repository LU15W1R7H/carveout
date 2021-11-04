use bevy::prelude::*;

use bevy_egui::EguiContext;

pub struct ToolboxPlugin;
impl Plugin for ToolboxPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_startup_system(start_up.system());
    app.add_system(ui.system());
  }
}

pub struct Toolbox {
  pub stroke: egui::Stroke,
}
impl Default for Toolbox {
  fn default() -> Self {
    Self {
      stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
    }
  }
}

fn start_up(mut commands: Commands) {
  commands.insert_resource(Toolbox::default());
}

fn ui(egui: Res<EguiContext>, mut toolbox: ResMut<Toolbox>) {
  egui::Window::new("📦 Toolbox").show(egui.ctx(), |ui| {
    ui.label("Pen color");
    let color = &mut toolbox.stroke.color;
    ui.color_edit_button_srgba(color);
    ui.label("Pen stroke");
    let width = &mut toolbox.stroke.width;
    ui.add(egui::Slider::new(width, 0.0..=10.0));
  });
}
