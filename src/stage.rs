// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use crate::actor::Actor;
use crate::actor::EventHandler;
use crate::actor::Layout;

use stretch::{style::*, node::{Node, Stretch}, geometry::Size};

pub struct Stage<'a> {
  pub name: String,
  width: u32,
  height: u32,
  viewport_width: u32,
  viewport_height: u32,
  visible: bool,
  pub stage_actor: Actor<'a>,
  pub stretch: Stretch
}

impl<'a> Stage<'a> {
  pub fn new(name: String, vw: u32, vh: u32, event_handler:  Option<Box<dyn EventHandler + 'a>>) -> Self {
    Stage {
      name: name,
      width: 0,
      height: 0,
      viewport_width: vw,
      viewport_height: vh,
      visible: false,
      stage_actor: Actor::new("stage_actor".to_string(), vw, vh, event_handler),
      stretch: Stretch::new()
    }
  }

  pub fn initialize(&mut self) {
    println!("stage::initialize");
    self.stage_actor.init_gl(self.viewport_width, self.viewport_height, &mut self.stretch);
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = visible;
  }

  pub fn set_needs_layout(&mut self) {
     self.stage_actor.set_needs_layout();
     self.stretch.compute_layout(self.stage_actor.node.unwrap(),
         Size::undefined()).unwrap();

    let layout = self.stretch.layout(self.stage_actor.node.unwrap()).unwrap();
    println!("{}, {}", layout.size.width, layout.size.height);
  }

  pub fn set_layout(&mut self, layout: Option<Box<dyn Layout + 'a>>) {
    self.stage_actor.set_layout(layout);
  }

  pub fn handle_input(&mut self, key: usize) {
     println!("stage key: {}", key);

      self.stage_actor.handle_input(key);

  }

  pub fn render(&mut self, shader_program: GLuint) {
    if !self.visible {
      return
    }
    self.stage_actor.animate();
    self.stage_actor.render(shader_program, &self.stretch, None);
  }

  pub fn add_actor(&mut self, mut actor: Actor<'a>) -> usize {
        println!("stage::add_actor");
    actor.init_gl(self.viewport_width, self.viewport_height,
        &mut self.stretch);

    match self.stretch.add_child(self.stage_actor.node.unwrap(),
        actor.node.unwrap()) {
       Ok(()) => {}
       Err(..) => {}
    }
    self.stage_actor.sub_actor_list.push(actor);

    self.stage_actor.sub_actor_list.len() - 1
  }
}
