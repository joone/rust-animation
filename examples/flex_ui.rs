// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

use glfw::{Action, Context, Key};
use stretch::{style::*, geometry::Size, geometry::Rect};

use std::sync::mpsc::Receiver;
use rust_animation::play::Play;
use rust_animation::stage::Stage;
use rust_animation::actor::Actor;
use rust_animation::actor::LayoutMode;
use rust_animation::actor::EasingFunction;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  let (mut window, events) = glfw.create_window(1920, 1080,
      "Image Viewer", glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let mut play = Play::new("Animation test".to_string());
  play.initialize();
  let mut stage = Stage::new("stage".to_string(), 1920, 1080, LayoutMode::Flex, None);
  stage.set_style(Style {
          size: Size { 
              width: Dimension::Points(1920.0), 
              height: Dimension::Points(1080.0),
          }, justify_content: JustifyContent::Center,
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::Center,
          margin: Rect {
              start: Dimension::Points(1.0),
              end: Dimension::Points(1.0),
              top: Dimension::Points(1.0),
              bottom: Dimension::Points(1.0),
              ..Default::default()
          },
          ..Default::default()
        }
  );
  stage.set_visible(true);   

  let justify_content = vec![
      JustifyContent::FlexStart,
      JustifyContent::FlexEnd,
      JustifyContent::Center,
      JustifyContent::SpaceBetween,
      JustifyContent::SpaceAround,
      JustifyContent::SpaceEvenly,
  ];
  let width  = 1500;
  let height = 100;
  for i in 0..6 {
    let actor_name = format!("actor_{}", i+1);
    let mut actor = Actor::new(actor_name.to_string(), width, height, None);
    actor.set_color(i as f32 / 18.0, i as f32 / 18.0, i as f32 / 18.0);
    actor.set_style(Style {
           size: Size { 
              width: Dimension::Points(width as f32), 
              height: Dimension::Points(height as f32),
          },
          justify_content: justify_content[i],
          margin: Rect {
              start: Dimension::Points(1.0),
              end: Dimension::Points(1.0),
              top: Dimension::Points(1.0),
              bottom: Dimension::Points(1.0),
              ..Default::default()
          },
          ..Default::default()
        }
    );
    for j in 0..10 {
      let mut sub_actor = Actor::new(format!("actor_{}_{}", i+1, j+1).to_string(),
          100, 98, None);
      sub_actor.set_color(1.0, j as f32 / 10.0, j as f32 / 10.0);
      actor.add_sub_actor(sub_actor);
    }
    stage.add_actor(actor);
   
  }

  stage.set_needs_layout();
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
