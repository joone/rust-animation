// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

use glfw::{Action, Context, Key};

use rust_animation::actor::Actor;
use rust_animation::actor::EasingFunction;
use rust_animation::actor::LayoutMode;
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
    .create_window(
      1920,
      1080,
      "Easing functions demo",
      glfw::WindowMode::Windowed,
    )
    .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let mut play = Play::new("Easing functions demo".to_string(), LayoutMode::UserDefine);
  play.initialize();
  let mut stage = Actor::new(
    "stage".to_string(),
    1920,
    1080,
    None,
  );
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
    let actor_name = format!("actor_{}", i + 1);
    let mut actor = Actor::new(actor_name.to_string(), width, height, None);
    actor.x = 0;
    actor.y = y;
    y += height as i32;
    actor.set_color(i as f32 / 18.0, i as f32 / 18.0, i as f32 / 18.0);
    actor.apply_translation_x_animation(0, (1920 - width) as i32, time, easing_functions[i]);
    actor.apply_rotation_animation(0, 360, time, EasingFunction::Linear);
    stage.add_sub_actor(actor);
  }
;
  play.add_stage(stage);

  while !window.should_close() {
    // events
    process_events(&mut window, &events);

    play.render();

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
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
