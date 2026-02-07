// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This example demonstrates the CoreAnimation-style API

extern crate glfw;

use glfw::{Action, Context, Key};

use rust_animation::layer::RALayer;
use rust_animation::layer::LayoutMode;
use rust_animation::animation::Animation;
use rust_animation::animation::EasingFunction;
use rust_animation::play::Play;
use std::sync::mpsc::Receiver;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    glfw::OpenGlProfileHint::Core,
  ));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  let (mut window, events) = glfw
    .create_window(1920, 1080, "CoreAnimation API Demo", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let mut play = Play::new(
    "CoreAnimation API test".to_string(),
    1920,
    1080,
    LayoutMode::UserDefine,
  );
  let mut stage = RALayer::new("stage".to_string(), 1920, 1080, None);
  stage.set_visible(true);

  // Example 1: Using CoreAnimation-style API for position animation
  let mut layer1 = RALayer::new("layer1".to_string(), 100, 100, None);
  layer1.set_position(100, 100);
  layer1.set_background_color(1.0, 0.0, 0.0); // Red
  layer1.set_opacity(1.0);

  // Create a CoreAnimation-style animation
  let mut position_animation = Animation::with_key_path("position.x");
  position_animation.duration = 3.0;
  position_animation.timing_function = Some(EasingFunction::EaseInOut);
  position_animation.set_from_value_position_x(100);
  position_animation.set_to_value_position_x(800);
  
  // Add animation with a key (CoreAnimation-style)
  layer1.add_animation(position_animation, Some("moveX"));

  // Example 2: Opacity animation
  let mut layer2 = RALayer::new("layer2".to_string(), 120, 120, None);
  layer2.set_position(100, 250);
  layer2.set_background_color(0.0, 1.0, 0.0); // Green
  
  let mut opacity_animation = Animation::with_key_path("opacity");
  opacity_animation.duration = 2.5;
  opacity_animation.timing_function = Some(EasingFunction::Linear);
  opacity_animation.set_from_value_opacity(1.0);
  opacity_animation.set_to_value_opacity(0.2);
  
  layer2.add_animation(opacity_animation, Some("fadeOut"));

  // Example 3: Scale animation
  let mut layer3 = RALayer::new("layer3".to_string(), 80, 80, None);
  layer3.set_position(100, 400);
  layer3.set_background_color(0.0, 0.0, 1.0); // Blue
  
  let mut scale_animation = Animation::with_key_path("transform.scale");
  scale_animation.duration = 3.0;
  scale_animation.timing_function = Some(EasingFunction::EaseInOutCubic);
  scale_animation.set_from_value_scale(1.0);
  scale_animation.set_to_value_scale(2.5);
  
  layer3.add_animation(scale_animation, Some("scaleUp"));

  // Example 4: Rotation animation
  let mut layer4 = RALayer::new("layer4".to_string(), 100, 100, None);
  layer4.set_position(100, 550);
  layer4.set_background_color(1.0, 1.0, 0.0); // Yellow
  
  let mut rotation_animation = Animation::with_key_path("transform.rotation");
  rotation_animation.duration = 4.0;
  rotation_animation.timing_function = Some(EasingFunction::Linear);
  rotation_animation.set_from_value_rotation(0);
  rotation_animation.set_to_value_rotation(360);
  
  layer4.add_animation(rotation_animation, Some("rotate"));

  // Example 5: Multiple animations on one layer
  let mut layer5 = RALayer::new("layer5".to_string(), 150, 150, None);
  layer5.set_position(300, 100);
  layer5.set_background_color(1.0, 0.0, 1.0); // Magenta
  
  // Position Y animation
  let mut pos_y_animation = Animation::with_key_path("position.y");
  pos_y_animation.duration = 2.0;
  pos_y_animation.timing_function = Some(EasingFunction::EaseInOut);
  pos_y_animation.set_from_value_position_y(100);
  pos_y_animation.set_to_value_position_y(600);
  layer5.add_animation(pos_y_animation, Some("moveY"));
  
  // Scale animation
  let mut scale_animation2 = Animation::with_key_path("transform.scale");
  scale_animation2.duration = 2.0;
  scale_animation2.timing_function = Some(EasingFunction::EaseInOut);
  scale_animation2.set_from_value_scale(1.0);
  scale_animation2.set_to_value_scale(0.5);
  layer5.add_animation(scale_animation2, Some("scaleDown"));

  // Example 6: Using sublayers (CoreAnimation-style)
  let mut parent_layer = RALayer::new("parentLayer".to_string(), 200, 200, None);
  parent_layer.set_position(500, 100);
  parent_layer.set_background_color(0.5, 0.5, 0.5); // Gray
  
  let mut child_layer = RALayer::new("childLayer".to_string(), 50, 50, None);
  child_layer.set_position(75, 75);
  child_layer.set_background_color(1.0, 1.0, 1.0); // White
  
  // Add child using CoreAnimation-style API
  parent_layer.add_sublayer(child_layer);

  // Add all layers to stage using CoreAnimation-style API
  stage.add_sublayer(layer1);
  stage.add_sublayer(layer2);
  stage.add_sublayer(layer3);
  stage.add_sublayer(layer4);
  stage.add_sublayer(layer5);
  stage.add_sublayer(parent_layer);

  play.add_stage(stage);

  println!("CoreAnimation API Demo");
  println!("======================");
  println!("Red box: Position X animation (100 -> 800)");
  println!("Green box: Opacity animation (1.0 -> 0.2)");
  println!("Blue box: Scale animation (1.0 -> 2.5)");
  println!("Yellow box: Rotation animation (0 -> 360)");
  println!("Magenta box: Position Y + Scale animations");
  println!("Gray box: Parent layer with white child sublayer");
  println!("\nPress ESC to exit");

  while !window.should_close() {
    process_events(&mut window, &events);
    play.render();
    window.swap_buffers();
    glfw.poll_events();
  }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
  for (_, event) in glfw::flush_messages(events) {
    match event {
      glfw::WindowEvent::FramebufferSize(width, height) => {
        unsafe { gl::Viewport(0, 0, width, height) }
      }
      glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
      _ => {}
    }
  }
}
