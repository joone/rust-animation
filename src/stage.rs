// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use crate::actor::Actor;
use crate::actor::EventHandler;

pub struct Stage<'a> {
  width: u32,
  height: u32,
  viewport_width: u32,
  viewport_height: u32,
  visible: bool,
  pub stage_actor: Actor<'a>,
}

impl<'a>  Stage<'a> {
  pub fn new(vw: u32, vh: u32, event_handler:  Option<Box<dyn EventHandler + 'a>>) -> Self {
    Stage {
      width: 0,
      height: 0,
      viewport_width: vw,
      viewport_height: vh,
      visible: false,
      stage_actor: Actor::new("stage_actor".to_string(), vw, vh, event_handler)
    }
  }

  pub fn initialize(&mut self) {
    self.stage_actor.init_gl(self.viewport_width, self.viewport_height);
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = visible;
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
    self.stage_actor.render(shader_program, None);
  }

  pub fn add_actor(&mut self, mut actor: Actor<'a>) -> usize {
    actor.init_gl(self.viewport_width, self.viewport_height);
    self.stage_actor.sub_actor_list.push(actor);

    self.stage_actor.sub_actor_list.len() - 1
  }
}