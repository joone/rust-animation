// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use crate::actor::Actor;
use crate::actor::EventHandler;
use crate::actor::Layout;
use crate::actor::LayoutMode;

use stretch::{style::*, node::{Node, Stretch}, geometry::Size};

pub struct Stage<'a> {
  pub name: String,
  width: u32,
  height: u32,
  viewport_width: u32,
  viewport_height: u32,
  visible: bool,
  stage_actor: Actor<'a>,
  pub stretch: Option<Stretch>
}

impl<'a> Stage<'a> {
  pub fn new(name: String, vw: u32, vh: u32, layout_mode: LayoutMode,
      event_handler:  Option<Box<dyn EventHandler + 'a>>) -> Self {

    let mut stretch = None;
    let mut actor;

     match layout_mode {
       LayoutMode::Flex => {
         stretch = Some(Stretch::new());
         actor = Actor::new("stage_actor".to_string(), vw, vh, event_handler);

       }
       LayoutMode::UserDefine => {
          actor = Actor::new("stage_actor".to_string(), vw, vh, event_handler);
          actor.init_gl(vw, vh);
       }
     }

    Self {
      name: name,
      width: 0,
      height: 0,
      viewport_width: vw,
      viewport_height: vh,
      visible: false,
      stage_actor: actor,
      stretch: stretch,
    }
  }

  pub fn set_image(&mut self, path: String) {
    self.stage_actor.set_image(path);
  }

  pub fn set_style(&mut self, style: Style) {
    self.stage_actor.set_style(style);
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = visible;
    self.stage_actor.init_gl(self.viewport_width, self.viewport_height);
  }

  pub fn set_needs_layout(&mut self) {
    self.stage_actor.set_needs_layout(&mut self.stretch);
    if let Some(stretch_obj) = &mut self.stretch {
      stretch_obj.compute_layout(self.stage_actor.node.unwrap(),
          Size::undefined()).unwrap();

      let layout = stretch_obj.layout(self.stage_actor.node.unwrap()).unwrap();
      println!("set_needs_layout {}, {}", layout.size.width, layout.size.height);
    }
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
    self.stage_actor.update_layout(&mut self.stretch);
    self.stage_actor.render(shader_program, &mut self.stretch, None);
  }

  pub fn add_actor(&mut self, actor: Actor<'a>) -> usize {
    self.stage_actor.add_sub_actor(actor);
    self.stage_actor.sub_actor_list.len() - 1
  }

  pub fn add_sub_actor(&mut self, actor: &mut Actor<'a>, sub_actor: Actor<'a>) {
    actor.add_sub_actor(sub_actor);
  }
}
