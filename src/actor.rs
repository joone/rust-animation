// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use cgmath::{Matrix, Matrix4, Deg, SquareMatrix, Vector3};
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

macro_rules! c_str {
  ($literal:expr) => {
      CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
  }
}

pub struct Actor {
  pub name: String,
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub anchor_x: f32,
  pub anchor_y: f32,
  pub scale_x: f32,
  pub scale_y: f32,
  pub rotation: f32,
  color: [f32; 3],
  viewport_width: u32,
  viewport_height: u32,
  pub sub_actor_list: Vec<Actor>,
  vertex_array_obj: gl::types::GLuint,
}

impl Actor {
  pub fn new(name: String, w: u32, h: u32) -> Self {
    Actor {
      name: name,
      x: 0,
      y: 0,
      width: w,
      height: h,
      anchor_x: 0.5,
      anchor_y: 0.5,
      scale_x: 1.0,
      scale_y: 1.0,
      rotation: 0.0,
      color: [1.0, 1.0, 1.0],
      viewport_width: 0,
      viewport_height: 0,
      sub_actor_list: Vec::new(),
      vertex_array_obj: gl::types::GLuint::default(),
    }
  }

  pub fn init_gl(&mut self, viewport_width: u32, viewport_height: u32) {
    self.viewport_width = viewport_width;
    self.viewport_height = viewport_height;
    unsafe {
      let (mut vertex_array_buffer, mut elem_array_buffer) = (0, 0);
      let vertices: [f32; 12] = [
          self.width as f32, self.height as f32, 0.0,  // top right
          self.width as f32, 0.0,                0.0,  // bottom right
          0.0,               0.0,                0.0,  // bottom left
          0.0,               self.height as f32, 0.0   // top left
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

  pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
    self.color[0] = r;
    self.color[1] = g;
    self.color[2] = b;
  }

  pub fn render(&self, shader_program: GLuint, actor: Option<&Actor>) {
    let mut x : f32 = self.x as f32;
    let mut y : f32 = self.y as f32;

    if let Some(main_actor) = actor {
      x += main_actor.x as f32;
      y += main_actor.y as f32;
    }

    for sub_actor in self.sub_actor_list.iter() {
      sub_actor.render(shader_program, Some(&self));
    }

    let mut transform: Matrix4<f32> = Matrix4::identity();

    // Apply orthographic projection matrix: left, right, bottom, top, near, far
    transform = transform * cgmath::ortho(0.0, self.viewport_width as f32,
        self.viewport_height as f32, 0.0, 0.5, -0.5);

    transform = transform *
        Matrix4::<f32>::from_translation(Vector3::new(
        x as f32, y as f32, 0.0));

    // Handle rotation and scale.
    // Move back to the original position.
    transform = transform *
        Matrix4::<f32>::from_translation(Vector3::new(self.width as f32 * self.anchor_x,
        self.height as f32 * self.anchor_y, 0.0));

    if self.rotation != 0.0 {
      transform = transform * Matrix4::<f32>::from_angle_z(Deg(self.rotation));
    }

    transform = transform * Matrix4::from_nonuniform_scale(self.scale_x,
        self.scale_y, 0.0);

    // Move to the origin of coordinate.
    transform = transform *
        Matrix4::<f32>::from_translation(Vector3::new(-(self.width as f32 * self.anchor_x),
          -(self.height as f32 * self.anchor_y), 0.0));

    unsafe {
      gl::UseProgram(shader_program);
      let loc_color = gl::GetUniformLocation(shader_program, c_str!("color").as_ptr());
      let loc_transform = gl::GetUniformLocation(shader_program, c_str!("transform").as_ptr());

      gl::Uniform4f(loc_color, self.color[0], self.color[1], self.color[2], 1.0);
      gl::UniformMatrix4fv(loc_transform, 1, gl::FALSE, transform.as_ptr());

      gl::BindVertexArray(self.vertex_array_obj);
      gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
  }

  pub fn add_sub_actor(&mut self, actor: Actor) {
    self.sub_actor_list.push(actor);
  }
}