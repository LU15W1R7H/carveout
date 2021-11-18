use bevy::prelude::*;
use bevy_egui::EguiContext;

pub struct MiscUiPlugin;
impl Plugin for MiscUiPlugin {
  fn build(&self, _app: &mut AppBuilder) {
    //app.init_resource::<WelcomeUiState>();
    //app.add_system(welcome_window_ui.system());
  }
}

#[allow(dead_code)]
struct WelcomeUiState {
  open: bool,
}
impl Default for WelcomeUiState {
  fn default() -> Self {
    Self { open: true }
  }
}

#[allow(dead_code)]
fn welcome_window_ui(mut state: Local<WelcomeUiState>, egui: Res<EguiContext>) {
  egui::Window::new("Welcome")
    .open(&mut state.open)
    .show(egui.ctx(), |ui| {
      ui.label("Welcome to Carveout.");
      ui.separator();
      ui.label("A tool for modern scientific digital pen note taking.");
    });
}
