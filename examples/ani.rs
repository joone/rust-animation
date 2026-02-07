// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use winit::{
  event::{Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::WindowBuilder,
};

use rust_animation::animation::Animation;
use rust_animation::animation::EasingFunction;
use rust_animation::layer::LayoutMode;
use rust_animation::layer::RALayer;
use rust_animation::play::Play;

fn main() {
  let event_loop = EventLoop::new().unwrap();
  let window = WindowBuilder::new()
    .with_title("Animation test")
    .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
    .build(&event_loop)
    .unwrap();

  let mut play = Play::new(
    "Animation test".to_string(),
    1920,
    1080,
    LayoutMode::UserDefine,
  );

  // Initialize wgpu context
  play.init_wgpu();

  let mut stage = RALayer::new("stage".to_string(), 1920, 1080, None);
  stage.set_visible(true);

  let mut layer_1 = RALayer::new("layer_1".to_string(), 400, 225, None);
  layer_1.x = 100;
  layer_1.y = 100;
  layer_1.set_image("examples/splash.png".to_string());

  let mut animation_1 = Animation::new();

  // 1X -> 2X for 5 sec.
  let time = 5.0;
  animation_1.apply_scale(1.0, 2.0, time, EasingFunction::Linear);
  animation_1.apply_translation_x(100, 1000, time, EasingFunction::EaseInOut);
  animation_1.apply_translation_y(100, 300, time, EasingFunction::EaseInOut);
  animation_1.apply_rotation(0, 360, time, EasingFunction::EaseInOut);
  layer_1.set_animation(Some(animation_1));

  let mut layer_2 = Play::new_layer("layer_2".to_string(), 120, 120, None);
  layer_2.x = 100;
  layer_2.y = 100;
  layer_2.scale_x = 1.5;
  layer_2.scale_y = 1.5;
  layer_2.set_color(0.0, 0.0, 1.0);
  // 0 degree -> 360 degree for 5 sec

  let mut animation_2 = Animation::new();
  animation_2.apply_rotation(0, 360, 5.0, EasingFunction::EaseInOut);
  layer_2.set_animation(Some(animation_2));

  let mut layer_3 = Play::new_layer("layer_3".to_string(), 50, 50, None);
  layer_3.x = 10;
  layer_3.y = 10;
  layer_3.set_color(1.0, 0.0, 0.0);
  layer_2.add_sub_layer(layer_3);

  stage.add_sub_layer(layer_1);
  stage.add_sub_layer(layer_2);

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
