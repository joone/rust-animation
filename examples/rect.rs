// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

use glfw::{Action, Context, Key};
use rust_animation::play::Play;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  let (mut window, events) = glfw.create_window(1920, 1080,
      "Image Viewer", glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();

  let play = Play::new(1920, 1080);

  while !window.should_close() {
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      handle_window_event(&mut window, event);
    }
    play.render();
  }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
  match event {
    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
      window.set_should_close(true)
    }
    _ => {}
  }
}
