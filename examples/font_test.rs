// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use winit::{
  event::{Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::WindowBuilder,
};

use rust_animation::layer::LayoutMode;
use rust_animation::layer::RALayer;
use rust_animation::play::Play;

fn main() {
  let event_loop = EventLoop::new().unwrap();
  let window = WindowBuilder::new()
    .with_title("Font Test")
    .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
    .build(&event_loop)
    .unwrap();

  let mut play = Play::new(
    "Font Test".to_string(),
    1920,
    1080,
    LayoutMode::UserDefine,
  );

  // Initialize wgpu context
  play.init_wgpu();

  let mut stage = RALayer::new("stage".to_string(), 1920, 1080, None);
  stage.set_color(0.5, 0.5, 0.5);
  stage.set_visible(true);

  let mut layer_1 = RALayer::new("layer_1".to_string(), 134, 85, None);
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
