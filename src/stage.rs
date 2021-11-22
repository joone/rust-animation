// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use crate::actor::Actor;

pub struct Stage {
  width: u32,
  height: u32,
  viewport_width: u32,
  viewport_height: u32,
  visible: bool,
  pub stage_actor: Actor,
}

impl Stage {
  pub fn new(vw: u32, vh: u32) -> Self {
    Stage {
      width: 0,
      height: 0,
      viewport_width: vw,
      viewport_height: vh,
      visible: false,
      stage_actor: Actor::new("stage_actor".to_string(), vw, vh),
    }
  }

  pub fn initialize(&mut self) {
    self.stage_actor.init_gl(self.viewport_width, self.viewport_height);
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = visible;
  }

  pub fn render(&mut self, shader_program: GLuint) {
    if !self.visible {
      return
    }
    self.stage_actor.animate();
    self.stage_actor.render(shader_program, None);
  }

  pub fn add_actor(&mut self, mut actor: Actor) -> usize {
    actor.init_gl(self.viewport_width, self.viewport_height);
    self.stage_actor.sub_actor_list.push(actor);

    self.stage_actor.sub_actor_list.len() - 1
  }
}