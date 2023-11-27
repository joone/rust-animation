// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

use glfw::{Action, Context, Key};

use rust_animation::actor::Actor;
use rust_animation::actor::LayoutMode;
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
    .create_window(1920, 1080, "Image Viewer", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let mut play = Play::new(
    "Animation test".to_string(),
    1920,
    1080,
    LayoutMode::UserDefine,
  );
  let mut stage = Actor::new("stage".to_string(), 1920, 1080, None);
  stage.set_visible(true);

  let mut actor_1 = Actor::new("actor_1".to_string(), 400, 225, None);
  actor_1.x = 100;
  actor_1.y = 100;
  actor_1.set_image("examples/splash.png".to_string());

  let mut animation_1 = Animation::new();

  // 1X -> 2X for 5 sec.
  let time = 5.0;
  animation_1.apply_scale(1.0, 2.0, time, EasingFunction::Linear);
  animation_1.apply_translation_x(100, 1000, time, EasingFunction::EaseInOut);
  animation_1.apply_translation_y(100, 300, time, EasingFunction::EaseInOut);
  animation_1.apply_rotation(0, 360, time, EasingFunction::EaseInOut);
  actor_1.set_animation(Some(animation_1));

  let mut actor_2 = Play::new_actor("actor_2".to_string(), 120, 120, None);
  actor_2.x = 100;
  actor_2.y = 100;
  actor_2.scale_x = 1.5;
  actor_2.scale_y = 1.5;
  actor_2.set_color(0.0, 0.0, 1.0);
  // 0 degree -> 360 degree for 5 sec

  let mut animation_2 = Animation::new();
  animation_2.apply_rotation(0, 360, 5.0, EasingFunction::EaseInOut);
  actor_2.set_animation(Some(animation_2));

  let mut actor_3 = Play::new_actor("actor_3".to_string(), 50, 50, None);
  actor_3.x = 10;
  actor_3.y = 10;
  actor_3.set_color(1.0, 0.0, 0.0);
  actor_2.add_sub_actor(actor_3);

  stage.add_sub_actor(actor_1);
  stage.add_sub_actor(actor_2);

  play.add_stage(stage);

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
        // make sure the viewport matches the new window dimensions; note that width and
        // height will be significantly larger than specified on retina displays.
        unsafe { gl::Viewport(0, 0, width, height) }
      }
      glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
      _ => {}
    }
  }
}
