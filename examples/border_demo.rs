// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This example demonstrates border rendering with CoreAnimation-style API

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
      .with_title("Border Rendering Demo")
      .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
      .build(&event_loop)
      .unwrap(),
  );

  // Get the actual window size (may differ from requested due to DPI scaling)
  let window_size = window.inner_size();
  let (width, height) = (window_size.width, window_size.height);

  let mut play = Play::new(
    "Border Rendering Demo".to_string(),
    width as i32,
    height as i32,
    LayoutMode::UserDefine,
  );

  // Initialize wgpu context with surface using actual window size
  play.init_wgpu_with_surface(window.clone(), width, height);

  let mut stage = Layer::new("stage".to_string(), width, height, None);
  stage.set_visible(true);
  stage.set_background_color(0.95, 0.95, 0.95); // Light gray background

  // Example 1: Red layer with black border
  let mut layer1 = Layer::new("layer1".to_string(), 150, 150, None);
  layer1.set_position(100, 100);
  layer1.set_background_color(1.0, 0.0, 0.0); // Red
  layer1.set_border(5.0, 0.0, 0.0, 0.0, 1.0); // 5px black border

  // Example 2: Green layer with white border
  let mut layer2 = Layer::new("layer2".to_string(), 150, 150, None);
  layer2.set_position(300, 100);
  layer2.set_background_color(0.0, 1.0, 0.0); // Green
  layer2.set_border(3.0, 1.0, 1.0, 1.0, 1.0); // 3px white border

  // Example 3: Blue layer with yellow border
  let mut layer3 = Layer::new("layer3".to_string(), 150, 150, None);
  layer3.set_position(500, 100);
  layer3.set_background_color(0.0, 0.0, 1.0); // Blue
  layer3.set_border(10.0, 1.0, 1.0, 0.0, 1.0); // 10px yellow border

  // Example 4: White layer with semi-transparent red border
  let mut layer4 = Layer::new("layer4".to_string(), 150, 150, None);
  layer4.set_position(700, 100);
  layer4.set_background_color(1.0, 1.0, 1.0); // White
  layer4.set_border(8.0, 1.0, 0.0, 0.0, 0.5); // 8px semi-transparent red border

  // Example 5: Nested layers with borders
  let mut parent_layer = Layer::new("parent".to_string(), 200, 200, None);
  parent_layer.set_position(100, 300);
  parent_layer.set_background_color(0.8, 0.8, 0.8); // Light gray
  parent_layer.set_border(5.0, 0.2, 0.2, 0.2, 1.0); // 5px dark gray border

  let mut child_layer = Layer::new("child".to_string(), 100, 100, None);
  child_layer.set_position(50, 50);
  child_layer.set_background_color(1.0, 0.5, 0.0); // Orange
  child_layer.set_border(3.0, 0.5, 0.0, 0.5, 1.0); // 3px purple border

  parent_layer.add_sublayer(child_layer);

  // Example 6: Various border widths
  let border_widths = vec![1.0, 2.0, 5.0, 10.0, 15.0];
  for (i, width) in border_widths.iter().enumerate() {
    let mut layer = Layer::new(format!("border_{}", i), 80, 80, None);
    layer.set_position(350 + (i as i32 * 100), 300);
    layer.set_background_color(0.5, 0.5, 1.0); // Light blue
    layer.set_border(*width, 0.0, 0.0, 0.5, 1.0); // Dark blue border
    stage.add_sublayer(layer);
  }

  // Example 7: Layer with border and image (if you have an image)
  let mut image_layer = Layer::new("image_layer".to_string(), 200, 200, None);
  image_layer.set_position(100, 520);
  image_layer.set_image("examples/splash.png".to_string());
  image_layer.set_border(5.0, 1.0, 0.0, 1.0, 1.0); // 5px magenta border

  // Add all layers to stage
  stage.add_sublayer(layer1);
  stage.add_sublayer(layer2);
  stage.add_sublayer(layer3);
  stage.add_sublayer(layer4);
  stage.add_sublayer(parent_layer);
  stage.add_sublayer(image_layer);

  play.add_stage(stage);

  println!("Border Rendering Demo");
  println!("=====================");
  println!("Demonstrates various border styles:");
  println!("- Top row: Different colors and widths");
  println!("- Middle left: Nested layers with borders");
  println!("- Middle: Progressive border widths (1px to 15px)");
  println!("- Bottom: Image with border");
  println!("\nPress ESC to exit");

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
