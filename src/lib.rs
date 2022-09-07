#![allow(clippy::single_match)]
#![feature(array_windows)]

extern crate nalgebra as na;

mod camera;
mod content;
mod file;
mod gfx;
mod input;
mod log;
mod spaces;
mod stroke;
mod tools;
mod ui;
mod util;

use camera::Camera;
use content::ContentManager;
use gfx::Gfx;
use input::InputManager;
use stroke::StrokeManager;
use tools::ToolManager;
use ui::Ui;

use instant::{Duration, Instant};
use winit::{
  event::WindowEvent,
  event_loop::ControlFlow,
  window::{Window, WindowId},
};

pub type CustomEvent = ();
pub type Event<'a> = winit::event::Event<'a, CustomEvent>;
pub type EventLoop = winit::event_loop::EventLoop<CustomEvent>;

pub struct Application {
  event_loop: Option<EventLoop>,
  input_manager: InputManager,
  window: Window,
  gfx: Gfx,

  egui_ctx: egui::Context,
  egui_winit: egui_winit::State,
  egui_shapes: Option<Vec<egui::epaint::ClippedShape>>,
  egui_textures_delta: Option<egui::TexturesDelta>,
  ui: Ui,

  content_manager: ContentManager,
  tool_manager: ToolManager,
  stroke_manager: StrokeManager,
  camera: Camera,
}

impl Application {
  pub async fn init() -> Application {
    log::init_log();

    let event_loop = EventLoop::default();
    let window = winit::window::WindowBuilder::default()
      .with_title(util::APP_NAME)
      .build(&event_loop)
      .expect("Fatal error: Failed to create winit window.");
    let input_manager = InputManager::default();
    let gfx = Gfx::init(&window).await;

    #[cfg(target_arch = "wasm32")]
    web_sys::window()
      .and_then(|w| w.document())
      .and_then(|d| d.body())
      .and_then(|b| {
        b.append_child(&winit::platform::web::WindowExtWebSys::canvas(&window))
          .ok()
      })
      .expect("Fatal error: Failed to append winit window to html body.");

    let egui_ctx = egui::Context::default();
    let egui_winit = egui_winit::State::new(&event_loop);
    let egui_shapes = None;
    let egui_textures_delta = None;
    let ui = Ui::default();

    let camera = Camera::default();
    let content_manager = ContentManager::default();
    let tool_manager = ToolManager::default();
    let stroke_manager = StrokeManager::default();

    Self {
      event_loop: Some(event_loop),
      window,
      input_manager,
      gfx,

      egui_ctx,
      egui_winit,
      egui_shapes,
      egui_textures_delta,
      ui,

      content_manager,
      tool_manager,
      stroke_manager,
      camera,
    }
  }

  pub fn run(mut self) {
    self
      .event_loop
      .take()
      .unwrap()
      .run(move |event, _, control_flow| {
        self.handle_event(event, control_flow);
      });
  }

  fn handle_event(&mut self, event: Event<'_>, control_flow: &mut ControlFlow) {
    match event {
      Event::NewEvents(_) => self.reset(),
      Event::WindowEvent { window_id, event } => {
        self.handle_window_event(event, window_id, control_flow)
      }
      Event::MainEventsCleared => self.update(control_flow),
      Event::RedrawRequested(_) => self.render(),
      Event::RedrawEventsCleared => {}
      Event::Suspended => {}
      Event::Resumed => {}
      Event::LoopDestroyed => {}
      _ => {}
    }
  }

  fn handle_window_event(
    &mut self,
    event: WindowEvent,
    window_id: WindowId,
    control_flow: &mut ControlFlow,
  ) {
    assert_eq!(window_id, self.window.id());
    match event {
      WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
      WindowEvent::Resized(new_size) => self.gfx.resize(new_size.width, new_size.height),
      WindowEvent::ScaleFactorChanged {
        ref new_inner_size, ..
      } => {
        self.gfx.resize(new_inner_size.width, new_inner_size.height);
      }
      _ => {}
    }

    let is_exclusive = self.egui_winit.on_event(&self.egui_ctx, &event);
    if is_exclusive && !self.ui.canvas().has_focus() {
      return;
    }

    self
      .input_manager
      .handle_event(&event, &self.window, &self.camera);
  }

  fn reset(&mut self) {
    self.input_manager.reset();
    self.content_manager.reset_delta();
  }

  fn update(&mut self, control_flow: &mut ControlFlow) {
    self.input_manager.update(&self.camera);

    camera::controller::update(&mut self.camera, &self.input_manager);

    self.tool_manager.update(
      &mut self.camera,
      &self.input_manager,
      &mut self.content_manager,
      &self.stroke_manager,
    );

    let egui_input: egui::RawInput = self.egui_winit.take_egui_input(&self.window);
    let egui_output = self.egui_ctx.run(egui_input, |ctx| {
      self.ui.run(
        ctx,
        ui::UiAccess {
          camera: &mut self.camera,
          content_manager: &mut self.content_manager,
          tool_manager: &mut self.tool_manager,
          stroke_manager: &mut self.stroke_manager,
        },
      );
    });
    self.egui_winit.handle_platform_output(
      &self.window,
      &self.egui_ctx,
      egui_output.platform_output,
    );

    self.egui_shapes = Some(egui_output.shapes);
    self.egui_textures_delta = Some(egui_output.textures_delta);

    // TODO: compare to eframe implementation
    let repaint_after = egui_output.repaint_after;
    if repaint_after.is_zero() {
      *control_flow = ControlFlow::Poll;
    } else if repaint_after == Duration::MAX {
      *control_flow = ControlFlow::Wait;
    } else {
      let repaint_at = Instant::now() + repaint_after;
      *control_flow = ControlFlow::WaitUntil(repaint_at);
    }
    self.window.request_redraw();

    let access = self.content_manager.access();
    let delta = self.content_manager.delta();
    self
      .stroke_manager
      .update_strokes(access, &delta.strokes, self.gfx.wgpu().device());
  }

  fn render(&mut self) {
    self.gfx.prepare(
      &self.window,
      &self.egui_ctx,
      self.egui_shapes.take().unwrap(),
      self.egui_textures_delta.take().unwrap(),
      &self.camera,
    );

    self.gfx.render(
      self.window.scale_factor() as f32,
      &self.camera,
      &self.stroke_manager,
    );
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub async fn run() {
  let app = Application::init().await;
  app.run();
}
