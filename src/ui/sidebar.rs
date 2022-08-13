use palette::{FromColor, Hsv, IntoColor};

use crate::canvas::{tool::ToolEnum, Canvas};

pub struct SidebarUi {
  rainbow_mode: bool,
}

impl SidebarUi {
  pub fn init() -> Self {
    Self {
      rainbow_mode: false,
    }
  }

  pub fn ui(&mut self, ctx: &egui::Context, canvas: &mut Canvas) {
    egui::SidePanel::left("toolbox_panel").show(ctx, |ui| {
      ui.add_space(10.0);
      ui.add(egui::Label::new(
        egui::RichText::new("📦 Toolbox").text_style(egui::TextStyle::Heading),
      ));
      ui.add_space(20.0);

      ui.group(|ui| {
        ui.label("Tools");
        let selected = &mut canvas.tool_config_mut().selected;

        ui.horizontal_wrapped(|ui| {
          selectable_tool(ui, selected, ToolEnum::Pen, "✏");
          selectable_tool(ui, selected, ToolEnum::Translate, "✋");
          selectable_tool(ui, selected, ToolEnum::Rotate, "🔄");
          selectable_tool(ui, selected, ToolEnum::Scale, "🔍");
        });

        ui.separator();

        match selected {
          ToolEnum::Pen => {
            let mut pen = &mut canvas.tool_config_mut().pen;

            ui.label("Pen color");
            let color = pen.color.into_components();
            let mut color = [color.0, color.1, color.2];
            ui.color_edit_button_rgb(&mut color);
            pen.color = palette::LinSrgb::new(color[0], color[1], color[2]);

            ui.checkbox(&mut self.rainbow_mode, "Rainbow mode");
            if self.rainbow_mode {
              let mut hsv = Hsv::from_color(pen.color);
              hsv.hue += 2.0;
              hsv.saturation = 1.0;
              hsv.value = 1.0;
              pen.color = hsv.into_color();
            }

            ui.label("Pen width");
            ui.add(egui::Slider::new(&mut pen.width, 0.1..=10.0));
          }
          ToolEnum::Translate => {
            ui.label("Translate option");
            let position = &mut canvas.camera_mut().position;
            ui.horizontal(|ui| {
              const SPEED: f32 = 0.001;
              ui.colored_label(egui::Color32::RED, "X:");
              ui.add(egui::DragValue::new(&mut position.x.0).speed(SPEED));
              ui.colored_label(egui::Color32::BLUE, "Y:");
              ui.add(egui::DragValue::new(&mut position.y.0).speed(SPEED));
            });
          }
          ToolEnum::Rotate => {
            ui.label("Rotate option");
            let rotation = &mut canvas.camera_mut().angle;
            ui.add(egui::Slider::new(rotation, 0.0..=std::f32::consts::TAU));
          }
          ToolEnum::Scale => {
            ui.label("Scale options");
            let scale = &mut canvas.camera_mut().scale;
            const SPEED_MUL: f32 = 0.003;
            let speed = *scale * SPEED_MUL;
            ui.add(
              egui::DragValue::new(scale)
                .clamp_range(0.1..=10.0)
                .speed(speed),
            );
          }
        }
      });
    });
  }
}

fn selectable_tool(ui: &mut egui::Ui, selected: &mut ToolEnum, selectable: ToolEnum, text: &str) {
  if ui
    .add(egui::SelectableLabel::new(*selected == selectable, text))
    .clicked()
  {
    *selected = selectable;
  }
}
