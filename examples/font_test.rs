// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::Arc;
use winit::{
  event::{Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::WindowBuilder,
};

use rust_animation::layer::LayoutMode;
use rust_animation::layer::Layer;
use rust_animation::play::Play;

fn main() {
  let event_loop = EventLoop::new().unwrap();
  let window = Arc::new(
    WindowBuilder::new()
      .with_title("Font Test")
      .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
      .build(&event_loop)
      .unwrap(),
  );

  // Get the actual window size (may differ from requested due to DPI scaling)
  let window_size = window.inner_size();
  let (width, height) = (window_size.width, window_size.height);

  let mut play = Play::new(
    "Font Test".to_string(),
    width as i32,
    height as i32,
    LayoutMode::UserDefine,
  );

  // Initialize wgpu context with surface using actual window size
  play.init_wgpu_with_surface(window.clone(), width, height);

  let mut stage = Layer::new("stage".to_string(), width, height, None);
  stage.set_color(0.5, 0.5, 0.5);
  stage.set_visible(true);

  let mut layer_1 = Layer::new("layer_1".to_string(), 134, 85, None);
  layer_1.x = 100;
  layer_1.y = 100;

  // Get wgpu context to set text
  if let Some(wgpu_ctx) = &play.wgpu_context {
    layer_1.set_text("hello", &wgpu_ctx.device, &wgpu_ctx.queue);
  }

  stage.add_sub_layer(layer_1);

  play.add_stage(stage);

  event_loop
    .run(move |event, elwt| {
      elwt.set_control_flow(ControlFlow::Poll);

      match event {
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::CloseRequested => elwt.exit(),
          WindowEvent::KeyboardInput {
            event:
              KeyEvent {
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                ..
              },
            ..
          } => elwt.exit(),
          WindowEvent::Resized(new_size) => {
            // Update wgpu surface and projection when window is resized
            play.resize(new_size.width, new_size.height);
          }
          WindowEvent::RedrawRequested => {
            play.render();
            window.request_redraw();
          }
          _ => {}
        },
        Event::AboutToWait => {
          window.request_redraw();
        }
        _ => {}
      }
    })
    .unwrap();
}
