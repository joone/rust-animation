// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

use glfw::{Action, Context, Key};
use stretch::{geometry::Rect, geometry::Size, node::Stretch, style::*};

use rust_animation::actor::Actor;
use rust_animation::actor::EasingFunction;
use rust_animation::actor::Layout;
use rust_animation::actor::LayoutMode;
use rust_animation::play::Play;
use std::sync::mpsc::Receiver;

pub struct FlexLayout {
  name: String,
}

impl FlexLayout {
  pub fn new() -> Self {
    let mut flex_layout = FlexLayout {
      name: "flex_layout".to_string(),
    };

    println!("new FlexLayout {}", flex_layout.name);

    flex_layout
  }
}

impl Layout for FlexLayout {
  fn layout_sub_actors(
    &mut self,
    actor: &mut Actor,
    parent_actor: Option<&Actor>,
    stretch: &mut Option<Stretch>,
  ) {
    println!("run layout_sub_layer for FlexLayout {}", self.name);
    if let Some(stretch_obj) = stretch {
      if let Some(style_obj) = actor.style {
        actor.node = Some(stretch_obj.new_node(style_obj, vec![]).unwrap());
      } else {
        //println!("default style: {}: {},{}", self.name, self.width, self.height);
        actor.node = Some(
          stretch_obj
            .new_node(
              Style {
                size: Size {
                  width: Dimension::Points(actor.width as f32),
                  height: Dimension::Points(actor.height as f32),
                },
                margin: Rect {
                  start: Dimension::Points(2.0),
                  end: Dimension::Points(2.0),
                  top: Dimension::Points(2.0),
                  bottom: Dimension::Points(2.0),
                  ..Default::default()
                },
                ..Default::default()
              },
              vec![],
            )
            .unwrap(),
        );
      }

      println!("actor name {}", actor.name);

      if let Some(parent_actor) = parent_actor {
        if !parent_actor.node.is_none() && !actor.node.is_none() {
          match stretch_obj.add_child(parent_actor.node.unwrap(), actor.node.unwrap()) {
            Ok(()) => {
              println!(
                " stretch node  is added {} {}",
                parent_actor.name, actor.name
              )
            }
            Err(..) => {}
          }
        }
      }
    }

    //self.update_layout(actor, stretch);
  }

  fn update_layout(&mut self, actor: &mut Actor, stretch: &mut Option<Stretch>) {
    if let Some(stretch_obj) = stretch {
      if !actor.node.is_none() {
        let layout = stretch_obj.layout(actor.node.unwrap()).unwrap();
        actor.x = layout.location.x as i32;
        actor.y = layout.location.y as i32;
        println!(
          "run update_layout for FlexLayout {} = {},{}",
          actor.name, actor.x, actor.y
        );
      }
    }
  }

  fn finalize(&mut self) {
    println!("finalize {}", self.name);
  }
}

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    glfw::OpenGlProfileHint::Core,
  ));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  let (mut window, events) = glfw
    .create_window(1920, 1080, "Flex UI demo", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let mut play = Play::new("Flex UI test".to_string(), 1920, 1080, LayoutMode::Flex);
  let mut stage = Actor::new("stage".to_string(), 1920, 1080, None);
  stage.set_style(Style {
    size: Size {
      width: Dimension::Points(1920.0),
      height: Dimension::Points(1080.0),
    },
    justify_content: JustifyContent::Center,
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
  });
  stage.set_visible(true);

  let justify_content = vec![
    JustifyContent::FlexStart,
    JustifyContent::FlexEnd,
    JustifyContent::Center,
    JustifyContent::SpaceBetween,
    JustifyContent::SpaceAround,
    JustifyContent::SpaceEvenly,
  ];
  let width = 1500;
  let height = 108;
  for i in 0..6 {
    let actor_name = format!("actor_{}", i + 1);
    let mut actor = Actor::new(actor_name.to_string(), width, height, None);
    actor.set_color(i as f32 / 6.0, i as f32 / 6.0, i as f32 / 6.0);
    actor.set_style(Style {
      size: Size {
        width: Dimension::Points(width as f32),
        height: Dimension::Points(height as f32),
      },
      justify_content: justify_content[i],
      align_items: AlignItems::Center,
      margin: Rect {
        start: Dimension::Points(1.0),
        end: Dimension::Points(1.0),
        top: Dimension::Points(1.0),
        bottom: Dimension::Points(1.0),
        ..Default::default()
      },
      padding: Rect {
        start: Dimension::Points(2.0),
        end: Dimension::Points(2.0),
        ..Default::default()
      },
      ..Default::default()
    });
    for j in 0..10 {
      let mut sub_actor = Actor::new(
        format!("actor_{}_{}", i + 1, j + 1).to_string(),
        100,
        100,
        None,
      );
      sub_actor.set_color(1.0, j as f32 / 10.0, j as f32 / 10.0);
      sub_actor.set_layout(Some(Box::new(FlexLayout::new())));
      actor.add_sub_actor(sub_actor);
    }
    actor.set_layout(Some(Box::new(FlexLayout::new())));
    stage.add_sub_actor(actor);
  }

  stage.set_layout(Some(Box::new(FlexLayout::new())));
  play.add_stage(stage);

  //play.set_stage_needs_layout(&"stage".to_string());

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
