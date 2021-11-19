// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use crate::actor::Actor;

pub struct Stage {
  width: u32,
  height: u32,
  stage_actor: Actor,
}

impl Stage {
  pub fn new(w: u32, h: u32) -> Self {
    Stage {
      width: w,
      height: h,
      stage_actor: Actor::new(w, h),
    }
  }

  pub fn initialize(&mut self) {

  }

  pub fn render(&mut self, shader_program: GLuint) {
    println!("Stage::render");
    self.stage_actor.render(shader_program);
  }

  pub fn add_actor(&mut self, mut actor: Actor) {
    actor.init_gl();
    self.stage_actor.sub_actor_list.push(actor);
  }
}