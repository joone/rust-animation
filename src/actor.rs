// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;
extern crate image;

use self::gl::types::*;
use cgmath::{Matrix, Matrix4, Deg, SquareMatrix, Vector3};
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

use crate::actor::image::GenericImage;

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
  pub rotation: i32,
  color: [f32; 3],
  viewport_width: u32,
  viewport_height: u32,
  pub image_path: String,
  pub sub_actor_list: Vec<Actor>,
  vertex_array_obj: gl::types::GLuint,
  texture: gl::types::GLuint,
  pub animated: bool,
  translation_x_animation_running: bool,
  translation_x_animation_from_value: i32,
  translation_x_animation_to_value: i32,
  translation_x_animation_by_value: i32,
  translation_y_animation_running: bool,
  translation_y_animation_from_value: i32,
  translation_y_animation_to_value: i32,
  translation_y_animation_by_value: i32,
  scale_animation_running: bool,
  scale_animation_from_value: f32,
  scale_animation_to_value: f32,
  scale_animation_by_value: f32,
  rotation_animation_running: bool,
  rotation_animation_from_value: i32,
  rotation_animation_to_value: i32,
  rotation_animation_by_value: i32
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
      rotation: 0,
      color: [1.0, 1.0, 1.0],
      viewport_width: 0,
      viewport_height: 0,
      image_path: "".to_string(),
      sub_actor_list: Vec::new(),
      vertex_array_obj: gl::types::GLuint::default(),
      texture: gl::types::GLuint::default(),
      animated: false,
      translation_x_animation_running: false,
      translation_x_animation_from_value: 0,
      translation_x_animation_to_value: 0,
      translation_x_animation_by_value: 0,
      translation_y_animation_running: false,
      translation_y_animation_from_value: 0,
      translation_y_animation_to_value: 0,
      translation_y_animation_by_value: 0,
      scale_animation_running: false,
      scale_animation_from_value: 0.0,
      scale_animation_to_value: 0.0,
      scale_animation_by_value: 0.0,
      rotation_animation_running: false,
      rotation_animation_from_value: 0,
      rotation_animation_to_value: 0,
      rotation_animation_by_value: 0
    }
  }

  pub fn init_gl(&mut self, viewport_width: u32, viewport_height: u32) {
    self.viewport_width = viewport_width;
    self.viewport_height = viewport_height;

    unsafe {
      let (mut vertex_array_buffer, mut elem_array_buffer) = (0, 0);
      let vertices: [f32; 20] = [
          // positions                   texture coords
          self.width as f32, self.height as f32, 0.0,  1.0, 1.0, // top right
          self.width as f32, 0.0,                0.0,  1.0, 0.0, // bottom right
          0.0,               0.0,                0.0,  0.0, 0.0, // bottom left
          0.0,               self.height as f32, 0.0,  0.0, 1.0  // top left
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

      let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
      // position attribute
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
      gl::EnableVertexAttribArray(0);

      if self.image_path.len() > 0 {
        // texture coord attribute
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        // Create a texture
        gl::GenTextures(1, &mut self.texture);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        match image::open(&Path::new(&self.image_path)) {
          Ok(mut img) => {
            let data = img.raw_pixels();
            gl::TexImage2D(gl::TEXTURE_2D,
                      0,
                      gl::RGB as i32,
                      img.width() as i32,
                      img.height() as i32,
                      0,
                      gl::RGB,
                      gl::UNSIGNED_BYTE,
                      &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
          }
          Err(err) => println!("Fail to load a image")
        }

      }
    }
  }

  pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
    self.color[0] = r;
    self.color[1] = g;
    self.color[2] = b;
  }

  pub fn set_image(&mut self, path: String) {
    self.image_path = path;
  }

  pub fn animate(&mut self) {
    if self.translation_x_animation_running == true {
      if self.translation_x_animation_to_value >
          self.translation_x_animation_from_value {
        if self.x < self.translation_x_animation_to_value {
          self.x += self.translation_x_animation_by_value;
        } else {
          self.translation_x_animation_running = false;
          self.x = self.translation_x_animation_to_value;
        }
      } else {
        if self.x > self.translation_x_animation_to_value {
          self.x -= self.translation_x_animation_by_value;
        } else {
          self.translation_x_animation_running = false;
          self.x = self.translation_x_animation_to_value;
        }
      }
    }

    if self.translation_y_animation_running == true {
      if self.translation_y_animation_to_value >
          self.translation_y_animation_from_value {
        if self.y < self.translation_y_animation_to_value {
          self.y += self.translation_y_animation_by_value;
        } else {
          self.translation_y_animation_running = false;
          self.y = self.translation_y_animation_to_value;
        }
      } else {
        if self.y > self.translation_y_animation_to_value {
          self.y -= self.translation_y_animation_by_value;
        } else {
          self.translation_y_animation_running = false;
          self.y = self.translation_y_animation_to_value;
        }
      }
    }

    if self.rotation_animation_running == true {
      if self.rotation < self.rotation_animation_to_value {
        self.rotation += self.rotation_animation_by_value;
      } else {
        self.rotation_animation_running = false;
        self.rotation = self.rotation_animation_to_value;
      }
    }

    if self.scale_animation_running == true {
      if self.scale_x < self.scale_animation_to_value {
        self.scale_x += self.scale_animation_by_value;
        self.scale_y += self.scale_animation_by_value;
      } else {
        self.scale_animation_running = false;
        self.scale_x = self.scale_animation_to_value;
        self.scale_y = self.scale_animation_to_value;
      }
    }

    if self.translation_x_animation_running == true || self.translation_y_animation_running == true
        || self.rotation_animation_running == true ||
        self.scale_animation_running == true {
      self.animated = true;
    } else {
      self.animated = false;
    }

    for sub_actor in self.sub_actor_list.iter_mut() {
      sub_actor.animate();
    }
  }

  pub fn apply_translation_x_animation(&mut self, from_value: i32, to_value: i32, by_value: i32) {
    self.translation_x_animation_running = true;
    self.translation_x_animation_from_value = from_value;
    self.translation_x_animation_to_value = to_value;
    self.translation_x_animation_by_value = by_value;
    self.x = self.translation_x_animation_from_value;
  }
 
  pub fn apply_translation_y_animation(&mut self, from_value: i32, to_value: i32, by_value: i32) {
    self.translation_y_animation_running = true;
    self.translation_y_animation_from_value = from_value;
    self.translation_y_animation_to_value = to_value;
    self.translation_y_animation_by_value = by_value;
    self.y = self.translation_y_animation_from_value;
  }

  pub fn apply_rotation_animation(&mut self, from_value: i32, to_value: i32, by_value: i32) {
    self.rotation_animation_running = true;
    self.rotation_animation_from_value = from_value;
    self.rotation_animation_to_value = to_value;
    self.rotation_animation_by_value = by_value;
    self.rotation = self.rotation_animation_from_value;
  }

  pub fn apply_scale_animation(&mut self, from_value: f32, to_value: f32, by_value: f32) {
    self.scale_animation_running = true;
    self.scale_animation_from_value = from_value;
    self.scale_animation_to_value = to_value;
    self.scale_animation_by_value = by_value;
    self.scale_x = self.scale_animation_from_value;
    self.scale_y = self.scale_animation_from_value;
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

    if self.rotation != 0 {
      transform = transform * Matrix4::<f32>::from_angle_z(Deg(self.rotation as f32));
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

      if self.image_path.len() > 0 {
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
      }

      gl::BindVertexArray(self.vertex_array_obj);
      gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
  }

  pub fn add_sub_actor(&mut self, actor: Actor) {
    self.sub_actor_list.push(actor);
  }
}