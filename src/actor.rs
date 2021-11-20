// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

pub struct Actor {
  pub width: u32,
  pub height: u32,
  pub sub_actor_list: Vec<Actor>,
  vertex_array_obj: gl::types::GLuint,
}

impl Actor {
  pub fn new(w: u32, h: u32) -> Self {
    Actor {
      width: w,
      height: h,
      sub_actor_list: Vec::new(),
      vertex_array_obj: gl::types::GLuint::default(),
    }
  }

   pub fn init_gl(&mut self, viewport_width: u32, viewport_height: u32) {
      unsafe {
      let (mut vertex_array_buffer, mut elem_array_buffer) = (0, 0);
      let vertices: [f32; 12] = [
         self.width as f32 / viewport_width as f32,  self.height as f32 / viewport_height as f32, 0.0,  // top right
         self.width as f32 / viewport_width as f32, -(self.height as f32 / viewport_height as f32), 0.0,  // bottom right
       -(self.width as f32 / viewport_width as f32), -(self.height as f32 / viewport_height as f32), 0.0,  // bottom left
       -(self.width as f32 / viewport_width as f32), self.height as f32 / viewport_height as f32, 0.0   // top left
      ];
      let indices = [
          0, 1, 3,  // first Triangle
          1, 2, 3   // second Triangle
      ];

      gl::GenVertexArrays(1, &mut self.vertex_array_obj);
      gl::BindVertexArray(self.vertex_array_obj);

      // position data
      gl::GenBuffers(1, &mut vertex_array_buffer);
      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_array_buffer);
      gl::BufferData(gl::ARRAY_BUFFER,
                    (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &vertices[0] as *const f32 as *const c_void,
                    gl::STATIC_DRAW);
      // index data
      gl::GenBuffers(1, &mut elem_array_buffer);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, elem_array_buffer);
      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &indices[0] as *const i32 as *const c_void,
                    gl::STATIC_DRAW);

      let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
      // position attribute
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
      gl::EnableVertexAttribArray(0);
    }
  }

  pub fn render(&mut self, shader_program: GLuint) {
    println!("actor::render");
    for actor in self.sub_actor_list.iter_mut() {
      actor.render(shader_program);
    }

    unsafe {
      gl::UseProgram(shader_program);
      gl::BindVertexArray(self.vertex_array_obj);
      gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
  }

  pub fn add_sub_actor(&mut self, actor: Actor) {
    self.sub_actor_list.push(actor);
  }
}