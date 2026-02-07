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

use rust_animation::animation::Animation;
use rust_animation::animation::EasingFunction;
use rust_animation::layer::LayoutMode;
use rust_animation::layer::Layer;
use rust_animation::play::Play;

fn main() {
  let event_loop = EventLoop::new().unwrap();
  let window = Arc::new(
    WindowBuilder::new()
      .with_title("Easing functions demo")
      .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
      .build(&event_loop)
      .unwrap(),
  );

  let mut play = Play::new(
    "Easing functions demo".to_string(),
    1920,
    1080,
    LayoutMode::UserDefine,
  );

  // Initialize wgpu context with surface
  play.init_wgpu_with_surface(window.clone(), 1920, 1080);

  let mut stage = Layer::new("stage".to_string(), 1920, 1080, None);
  stage.set_visible(true);

  let easing_functions = vec![
    EasingFunction::EaseIn,
    EasingFunction::EaseInCubic,
    EasingFunction::EaseInOut,
    EasingFunction::EaseInOutCubic,
    EasingFunction::EaseInOutQuad,
    EasingFunction::EaseInOutQuart,
    EasingFunction::EaseInOutQuint,
    EasingFunction::EaseInQuad,
    EasingFunction::EaseInQuart,
    EasingFunction::EaseInQuint,
    EasingFunction::EaseOut,
    EasingFunction::EaseOutCubic,
    EasingFunction::EaseOutQuad,
    EasingFunction::EaseOutQuart,
    EasingFunction::EaseOutQuint,
    EasingFunction::Linear,
    EasingFunction::Step,
  ];
  let mut y = 0;
  let time = 5.0;
  let width = 63;
  let height = width;
  for i in 0..17 {
    let layer_name = format!("layer_{}", i + 1);
    let mut layer = Layer::new(layer_name.to_string(), width, height, None);
    layer.x = 0;
    layer.y = y;
    y += height as i32;
    layer.set_color(i as f32 / 18.0, i as f32 / 18.0, i as f32 / 18.0);

    let mut animation = Animation::new();
    animation.apply_translation_x(0, (1920 - width) as i32, time, easing_functions[i]);
    animation.apply_rotation(0, 360, time, EasingFunction::Linear);
    layer.set_animation(Some(animation));
    stage.add_sub_layer(layer);
  }
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
