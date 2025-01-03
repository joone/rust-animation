// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;
extern crate image;
extern crate keyframe;

use self::gl::types::*;
use cgmath::{Deg, Matrix, Matrix4, SquareMatrix, Vector3};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

use stretch::{
  node::{Node, Stretch},
  style::*,
};

use crate::animation::Animation;
use crate::font::FontRenderer;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
  Space = 32,
  Enter = 36,
  Tab = 48,
  Backspace = 51,
  Escape = 53,
  Right = 262,
  Left = 263,
  Down = 264,
  Up = 265,
}

#[derive(Copy, Clone, Debug)]
pub enum LayoutMode {
  UserDefine,
  Flex,
}

macro_rules! c_str {
  ($literal:expr) => {
    CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
  };
}

pub struct Actor {
  pub name: String,
  pub x: i32,
  pub y: i32,
  pub z: f32,
  pub width: u32,
  pub height: u32,
  pub anchor_x: f32,
  pub anchor_y: f32,
  pub scale_x: f32,
  pub scale_y: f32,
  pub rotation: i32,
  pub visible: bool,
  color: [f32; 3],
  pub image_path: String,
  pub sub_actor_list: Vec<Actor>,
  vertex_array_obj: gl::types::GLuint,
  texture: gl::types::GLuint,
  pub animated: bool,
  pub animation: Option<Animation>,
  event_handler: Option<Box<dyn EventHandler>>,
  layout: Option<Box<dyn Layout>>,
  focused_sub_actor: usize,
  focused: bool,
  pub needs_update: bool,
  pub node: Option<Node>,   // for stretch only
  pub style: Option<Style>, // for stretch only
}

pub trait EventHandler {
  fn key_focus_in(&mut self, actor: &mut Actor);
  fn key_focus_out(&mut self, actor: &mut Actor);
  fn key_down(&mut self, key: Key, actor: &mut Actor);
}

pub trait Layout {
  fn layout_sub_actors(
    &mut self,
    actor: &mut Actor,
    parent_actor: Option<&Actor>,
    stretch: &mut Option<Stretch>,
  );
  fn update_layout(&mut self, actor: &mut Actor, stretch: &mut Option<Stretch>);
  fn finalize(&mut self);
}

impl Actor {
  pub fn new(name: String, w: u32, h: u32, event_handler: Option<Box<dyn EventHandler>>) -> Self {
    let mut actor = Actor {
      name: name,
      x: 0,
      y: 0,
      z: 0.0,
      width: w,
      height: h,
      anchor_x: 0.5,
      anchor_y: 0.5,
      scale_x: 1.0,
      scale_y: 1.0,
      rotation: 0,
      visible: true,
      color: [1.0, 1.0, 1.0],
      image_path: "".to_string(),
      sub_actor_list: Vec::new(),
      vertex_array_obj: gl::types::GLuint::default(),
      texture: gl::types::GLuint::default(),
      animated: false,
      animation: None,
      event_handler: event_handler,
      layout: None,
      focused_sub_actor: 0,
      focused: false,
      needs_update: true,
      node: None,
      style: None,
    };
    actor.init_gl();

    actor
  }

  pub fn init_gl(&mut self) {
    unsafe {
      let (mut vertex_array_buffer, mut elem_array_buffer) = (0, 0);
      let vertices: [f32; 20] = [
        // positions                   texture coords
        self.width as f32,
        self.height as f32,
        0.0,
        1.0,
        1.0, // top right
        self.width as f32,
        0.0,
        0.0,
        1.0,
        0.0, // bottom right
        0.0,
        0.0,
        0.0,
        0.0,
        0.0, // bottom left
        0.0,
        self.height as f32,
        0.0,
        0.0,
        1.0, // top left
      ];
      let indices = [
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
      ];

      gl::GenVertexArrays(1, &mut self.vertex_array_obj);
      gl::BindVertexArray(self.vertex_array_obj);

      // position data
      gl::GenBuffers(1, &mut vertex_array_buffer);
      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_array_buffer);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
      );
      // index data
      gl::GenBuffers(1, &mut elem_array_buffer);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, elem_array_buffer);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &indices[0] as *const i32 as *const c_void,
        gl::STATIC_DRAW,
      );

      let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
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

  pub fn set_text(&mut self, text: &str) {
    let mut font_renderer: FontRenderer = FontRenderer::new("fonts/DejaVuSans.ttf".to_string());
    let image = font_renderer.render(text);
    let dynamic_image = DynamicImage::ImageRgba8(image);

    dynamic_image.save("temp.png").unwrap();

    self.image_path = "temp".to_string();
    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

    unsafe {
      // texture coord attribute
      gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
      );
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

      self.width = dynamic_image.width();
      self.height = dynamic_image.height();

      println!("width: {}, height: {}", self.width, self.height);

      //   let data = image.into_raw();
      //let data = image.into_vec();
      let to_rgba = dynamic_image.to_rgba8();
      let data = to_rgba.into_raw();
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        self.width as i32,
        self.height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const c_void,
      );
      gl::GenerateMipmap(gl::TEXTURE_2D);
      // Unbind the texture
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
  }

  pub fn set_image(&mut self, path: String) {
    self.image_path = path;

    if self.image_path.len() > 0 {
      let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
      unsafe {
        // texture coord attribute
        gl::VertexAttribPointer(
          1,
          2,
          gl::FLOAT,
          gl::FALSE,
          stride,
          (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
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
          Ok(img) => {
            let to_rgba = img.to_rgba8();
            let data = to_rgba.into_vec();
            gl::TexImage2D(
              gl::TEXTURE_2D,
              0,
              gl::RGB as i32,
              img.width() as i32,
              img.height() as i32,
              0,
              gl::RGBA,
              gl::UNSIGNED_BYTE,
              &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
          }
          Err(err) => println!("Fail to load a image {:?}", err),
        }
      }
    }
  }

  pub fn set_layout(&mut self, layout: Option<Box<dyn Layout>>) {
    self.layout = layout;
  }

  pub fn set_animation(&mut self, animation: Option<Animation>) {
    self.animation = animation;
  }

  pub fn set_style(&mut self, style: Style) {
    self.style = Some(style);
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = visible;
  }

  /*pub fn update(&mut self) {
    // Sort sub actors by z-axis
    self.sub_actor_list.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
  }*/

  pub fn animate(&mut self) {
    if let Some(mut animation) = self.animation.take() {
      animation.run(self);
      self.animation = Some(animation);
    }

    for sub_actor in self.sub_actor_list.iter_mut() {
      sub_actor.animate();
    }
  }

  pub fn select_next_sub_actor(&mut self) {
    if self.sub_actor_list.len() <= 0 {
      return;
    }
    // no more next actor.
    if self.focused_sub_actor < self.sub_actor_list.len() - 1 {
      let prev_focused_sub_actor = self.focused_sub_actor;
      self.focused_sub_actor += 1;
      self.sub_actor_list[self.focused_sub_actor].set_focus(true);
      self.sub_actor_list[prev_focused_sub_actor].set_focus(false);
    }
  }

  pub fn select_prev_sub_actor(&mut self) {
    if self.sub_actor_list.len() <= 0 {
      return;
    }
    // ne more previous actor.
    if self.focused_sub_actor == 0 {
      return;
    }
    let prev_focused_sub_actor = self.focused_sub_actor;
    self.focused_sub_actor -= 1;
    self.sub_actor_list[self.focused_sub_actor].set_focus(true);
    self.sub_actor_list[prev_focused_sub_actor].set_focus(false);
  }

  pub fn set_focus(&mut self, focused: bool) {
    self.focused = focused;
    if let Some(mut event_handler) = self.event_handler.take() {
      //println!("set_focus {} {} ", self.name, focused);

      if self.focused {
        event_handler.key_focus_in(self);
      } else {
        event_handler.key_focus_out(self);
      }
      self.event_handler = Some(event_handler);
    }
  }

  pub fn handle_input(&mut self, key: Key) {
    for sub_actor in self.sub_actor_list.iter_mut() {
      if sub_actor.focused {
        sub_actor.handle_input(key);
      }
    }
    if let Some(mut event_handler) = self.event_handler.take() {
      event_handler.key_down(key, self);
      self.event_handler = Some(event_handler);
    }
  }

  pub fn layout_sub_actors(&mut self, parent_actor: Option<&Actor>, stretch: &mut Option<Stretch>) {
    if let Some(mut layout) = self.layout.take() {
      layout.layout_sub_actors(self, parent_actor, stretch);
      self.layout = Some(layout); // Put back the layout
    }

    // Replace the sub_actor_list with an empty vector and take the original vector out
    let mut sub_actor_list = std::mem::replace(&mut self.sub_actor_list, Vec::new());

    // Iterate over the vector outside of the self structure
    for sub_actor in &mut sub_actor_list {
      // As we are outside of the self structure, we can now borrow self as immutable
      sub_actor.layout_sub_actors(Some(self), stretch);
    }

    // Put back the original sub_actor_list
    self.sub_actor_list = sub_actor_list;
  }

  pub fn update_layout(&mut self, stretch: &mut Option<Stretch>) {
    if let Some(mut layout) = self.layout.take() {
      layout.update_layout(self, stretch);
      self.layout = Some(layout); // Put back the layout
    }

    for sub_actor in self.sub_actor_list.iter_mut() {
      sub_actor.update_layout(stretch);
    }
  }

  pub fn finalize_layout(&mut self) {
    if let Some(ref mut layout) = self.layout {
      layout.finalize();
    }
  }

  pub fn model_matrix(&self) -> Matrix4<f32> {
    let mut transform: Matrix4<f32> = Matrix4::identity();
    transform = transform
      * Matrix4::<f32>::from_translation(Vector3::new(self.x as f32, self.y as f32, self.z as f32));

    // Handle rotation and scale.
    // Move back to the original position.
    transform = transform
      * Matrix4::<f32>::from_translation(Vector3::new(
        self.width as f32 * self.anchor_x,
        self.height as f32 * self.anchor_y,
        0.0,
      ));

    if self.rotation != 0 {
      transform = transform * Matrix4::<f32>::from_angle_z(Deg(self.rotation as f32));
    }

    transform = transform * Matrix4::from_nonuniform_scale(self.scale_x, self.scale_y, 0.0);

    // Move to the origin of coordinate.
    transform = transform
      * Matrix4::<f32>::from_translation(Vector3::new(
        -(self.width as f32 * self.anchor_x),
        -(self.height as f32 * self.anchor_y),
        0.0,
      ));

    transform
  }

  pub fn render(
    &mut self,
    shader_program: GLuint,
    parent_model_matrix: Option<&Matrix4<f32>>,
    projection: &Matrix4<f32>,
  ) {
    if !self.visible {
      return;
    }

    let mut transform: Matrix4<f32> = self.model_matrix();
    if let Some(parent_model_matrix) = parent_model_matrix {
      transform = transform * parent_model_matrix;
    }

    unsafe {
      gl::UseProgram(shader_program);
      let loc_color = gl::GetUniformLocation(shader_program, c_str!("color").as_ptr());
      let loc_transform = gl::GetUniformLocation(shader_program, c_str!("transform").as_ptr());
      let loc_projection = gl::GetUniformLocation(shader_program, c_str!("projection").as_ptr());
      let loc_use_texture = gl::GetUniformLocation(shader_program, c_str!("useTexture").as_ptr());

      gl::Uniform4f(loc_color, self.color[0], self.color[1], self.color[2], 1.0);
      gl::UniformMatrix4fv(loc_transform, 1, gl::FALSE, transform.as_ptr());
      gl::UniformMatrix4fv(loc_projection, 1, gl::FALSE, projection.as_ptr());

      if self.image_path.len() > 0 {
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
        gl::Uniform1i(loc_use_texture, 1);
      } else {
        gl::Uniform1i(loc_use_texture, 0);
      }

      gl::BindVertexArray(self.vertex_array_obj);
      gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }

    for sub_actor in self.sub_actor_list.iter_mut() {
      if sub_actor.focused == false {
        sub_actor.render(shader_program, Some(&transform), projection);
      }
    }

    // render the focused sub_actor at the end.
    if self.sub_actor_list.len() > 0 {
      self.sub_actor_list[self.focused_sub_actor].render(
        shader_program,
        Some(&transform),
        projection,
      );
    }
  }

  pub fn add_sub_actor(&mut self, actor: Actor) {
    self.sub_actor_list.push(actor);
  }
}
