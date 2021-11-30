// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;
extern crate image;
extern crate keyframe;

use self::gl::types::*;
use cgmath::{Matrix, Matrix4, Deg, SquareMatrix, Vector3};
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;
use std::time::{Instant};

use keyframe::{ease, functions::*};
use stretch::{style::*, node::{Node, Stretch}, geometry::Size, geometry::Rect};

use crate::actor::image::GenericImage;

#[derive(Copy, Clone, Debug)]
pub enum EasingFunction {
    EaseIn,
    EaseInCubic,
    EaseInOut,
    EaseInOutCubic,
    EaseInOutQuad,
    EaseInOutQuart,
    EaseInOutQuint,
    EaseInQuad,
    EaseInQuart,
    EaseInQuint,
    EaseOut,
    EaseOutCubic,
    EaseOutQuad,
    EaseOutQuart,
    EaseOutQuint,
    Linear,
    Step,
}

macro_rules! c_str {
  ($literal:expr) => {
      CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
  }
}

pub struct Actor<'a> {
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
  color: [f32; 3],
  viewport_width: u32,
  viewport_height: u32,
  pub image_path: String,
  pub sub_actor_list: Vec<Actor<'a>>,
  vertex_array_obj: gl::types::GLuint,
  texture: gl::types::GLuint,
  pub animated: bool,
  animation_time_instance: Instant,

  translation_x_animation_running: bool,
  translation_x_animation_starting_time: u128,
  translation_x_animation_time_duration: f32,
  translation_x_animation_from_value: i32,
  translation_x_animation_to_value: i32,
  translation_x_animation_ease: EasingFunction,
 
  translation_y_animation_running: bool,
  translation_y_animation_starting_time: u128,
  translation_y_animation_time_duration: f32,
  translation_y_animation_from_value: i32,
  translation_y_animation_to_value: i32,
  translation_y_animation_ease: EasingFunction,

  scale_animation_running: bool,
  scale_animation_starting_time: u128,
  scale_animation_time_duration: u128,
  scale_animation_from_value: f32,
  scale_animation_to_value: f32,
  scale_animation_ease: EasingFunction, 

  rotation_animation_running: bool,
  rotation_animation_starting_time: u128,
  rotation_animation_time_duration: u128,
  rotation_animation_from_value: i32,
  rotation_animation_to_value: i32,
  rotation_animation_ease: EasingFunction,

  event_handelr: Option<Box<dyn EventHandler + 'a>>,
  layout: Option<Box<dyn Layout + 'a>>,
  focused_sub_actor: usize,
  focused: bool,
  needsUpdate: bool,
  pub node: Option<Node>
}

pub trait EventHandler {
  fn key_focus_in(&mut self, val: u32, actor: &mut Actor);
  fn key_focus_out(&mut self, val: u32, actor: &mut Actor);
}

pub trait Layout {
  fn layout_sub_actors(&mut self, actor: &mut Vec<Actor>);
}

impl<'a> Actor<'a> {
  pub fn new(name: String, w: u32, h: u32, event_handler: Option<Box<dyn EventHandler + 'a>>) -> Self {
    Actor {
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
      color: [1.0, 1.0, 1.0],
      viewport_width: 0,
      viewport_height: 0,
      image_path: "".to_string(),
      sub_actor_list: Vec::new(),
      vertex_array_obj: gl::types::GLuint::default(),
      texture: gl::types::GLuint::default(),
      animated: false,
      animation_time_instance: Instant::now(),
      translation_x_animation_running: false,
      translation_x_animation_starting_time: 0,
      translation_x_animation_time_duration: 0.0,
      translation_x_animation_from_value: 0,
      translation_x_animation_ease: EasingFunction::Linear,
      translation_x_animation_to_value: 0,
      translation_y_animation_running: false,
      translation_y_animation_starting_time: 0,
      translation_y_animation_time_duration: 0.0,
      translation_y_animation_from_value: 0,
      translation_y_animation_to_value: 0,
      translation_y_animation_ease: EasingFunction::Linear,
      scale_animation_running: false,
      scale_animation_starting_time: 0,
      scale_animation_time_duration: 0,
      scale_animation_from_value: 0.0,
      scale_animation_to_value: 0.0,
      scale_animation_ease: EasingFunction::Linear,
      rotation_animation_running: false,
      rotation_animation_starting_time: 0,
      rotation_animation_time_duration: 0,
      rotation_animation_from_value: 0,
      rotation_animation_to_value: 0,
      rotation_animation_ease: EasingFunction::Linear,
      event_handelr: event_handler,
      layout: None,
      focused_sub_actor: 0,
      focused: false,
      needsUpdate: false,
      node: None
    }
  }

  pub fn init_gl(&mut self, viewport_width: u32, viewport_height: u32,
      stretch: &mut Stretch) {
    self.viewport_width = viewport_width;
    self.viewport_height = viewport_height;
    println!("actor::init_gl");
    self.node = Some(stretch.new_node(Style {
      size: Size { 
          width: Dimension::Points(self.width as f32), 
          height: Dimension::Points(self.height as f32),
      }, justify_content: JustifyContent::SpaceEvenly,
       margin: Rect {
                    start: Dimension::Points(2.0),
                    end: Dimension::Points(2.0),
                    top: Dimension::Points(2.0),
                    bottom: Dimension::Points(2.0),
                    ..Default::default()
      },
      ..Default::default()
  }, vec![]).unwrap());

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
          Ok(img) => {
            let to_rgba = img.to_rgba();
            let data = to_rgba.into_vec();
            gl::TexImage2D(gl::TEXTURE_2D,
                      0,
                      gl::RGB as i32,
                      img.width() as i32,
                      img.height() as i32,
                      0,
                      gl::RGBA,
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

  pub fn set_layout(&mut self, layout: Option<Box<dyn Layout + 'a>>) {
    self.layout = layout;
  }

  /*pub fn update(&mut self) {
    // Sort sub actors by z-axis
    self.sub_actor_list.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
  }*/

  fn easing_function(easing: EasingFunction, from: f32, to: f32, duration: f32) -> f32 {
     match easing {
        EasingFunction::EaseIn => ease(EaseIn, from, to, duration),
        EasingFunction::EaseInCubic  => ease(EaseInCubic, from, to, duration),
        EasingFunction::EaseInOut => ease(EaseInOut, from, to, duration),
        EasingFunction::EaseInOutCubic => ease(EaseInOutCubic, from, to, duration),
        EasingFunction::EaseInOutQuad => ease(EaseInOutQuad , from, to, duration),
        EasingFunction::EaseInOutQuart => ease(EaseInOutQuart, from, to, duration),
        EasingFunction::EaseInOutQuint => ease(EaseInOutQuint, from, to, duration),
        EasingFunction::EaseInQuad => ease(EaseInQuad, from, to, duration),
        EasingFunction::EaseInQuart => ease(EaseInQuart, from, to, duration),
        EasingFunction::EaseInQuint => ease(EaseInQuint, from, to, duration),
        EasingFunction::EaseOut => ease(EaseOut, from, to, duration),
        EasingFunction::EaseOutCubic => ease(EaseOutCubic, from, to, duration),
        EasingFunction::EaseOutQuad => ease(EaseOutQuad, from, to, duration),
        EasingFunction::EaseOutQuart => ease(EaseOutQuart, from, to, duration),
        EasingFunction::EaseOutQuint => ease(EaseOutQuint, from, to, duration),
        EasingFunction::Linear => ease(Linear, from, to, duration),
        EasingFunction::Step => ease(Step, from, to, duration)
     }
  }
  pub fn animate(&mut self) {
    if self.needsUpdate {
        self.layout_sub_actors();
      self.needsUpdate = false;
    }

    if self.translation_x_animation_running == true {
      if self.translation_x_animation_starting_time == 0 {
        self.translation_x_animation_starting_time = self.animation_time_instance.elapsed().as_millis();
      }
      let cur_time = (self.animation_time_instance.elapsed().as_millis() -
          self.translation_x_animation_starting_time) as f32 / self.translation_x_animation_time_duration;
      if cur_time <= 1.0 {
        self.x = Actor::easing_function(self.translation_x_animation_ease, self.translation_x_animation_from_value as f32, 
          self.translation_x_animation_to_value as f32, cur_time) as i32;
      } else {
        self.translation_x_animation_running = false;
        self.translation_x_animation_starting_time = 0;
        self.x = self.translation_x_animation_to_value;
      }
    }

    if self.translation_y_animation_running == true {
      if self.translation_y_animation_starting_time == 0 {
        self.translation_y_animation_starting_time = self.animation_time_instance.elapsed().as_millis();
      }
      let cur_time = (self.animation_time_instance.elapsed().as_millis() -
          self.translation_y_animation_starting_time) as f32 / self.translation_y_animation_time_duration;
      if cur_time <= 1.0 {
        self.y = Actor::easing_function(self.translation_y_animation_ease, self.translation_y_animation_from_value as f32, 
          self.translation_y_animation_to_value as f32, cur_time) as i32;
      } else {
        self.translation_y_animation_running = false;
        self.translation_y_animation_starting_time = 0;
        self.y = self.translation_y_animation_to_value;
      }
    }

    if self.rotation_animation_running == true {
      if self.rotation_animation_starting_time == 0 {
        self.rotation_animation_starting_time = self.animation_time_instance.elapsed().as_millis();
      }
 
      let cur_time = (self.animation_time_instance.elapsed().as_millis() -
          self.rotation_animation_starting_time) as f32 / self.rotation_animation_time_duration as f32;
      if cur_time <= 1.0 {
        self.rotation = Actor::easing_function(self.rotation_animation_ease, self.rotation_animation_from_value as f32, 
            self.rotation_animation_to_value as f32, cur_time) as i32;
      } else {
        self.rotation_animation_running = false;
        self.rotation_animation_starting_time = 0;
        self.rotation = self.rotation_animation_to_value;
      }
    }

    if self.scale_animation_running == true {
      if self.scale_animation_starting_time == 0 {
        self.scale_animation_starting_time = self.animation_time_instance.elapsed().as_millis();
      }

      let cur_time = (self.animation_time_instance.elapsed().as_millis() -
          self.scale_animation_starting_time) as f32 / self.scale_animation_time_duration as f32;
      if cur_time <= 1.0 {
        self.scale_x = Actor::easing_function(self.scale_animation_ease, self.scale_animation_from_value, 
            self.scale_animation_to_value, cur_time) as f32;
        self.scale_y = Actor::easing_function(self.scale_animation_ease, self.scale_animation_from_value, 
            self.scale_animation_to_value, cur_time) as f32;
      } else {
        self.scale_animation_running = false;
        self.scale_animation_starting_time = 0;
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

  pub fn apply_translation_x_animation(&mut self, from_value: i32, to_value: i32, time: f32, easing: EasingFunction) {
    self.translation_x_animation_running = true;
    self.translation_x_animation_ease = easing;
    self.translation_x_animation_from_value = from_value;
    self.translation_x_animation_to_value = to_value;
    self.translation_x_animation_time_duration = time * 1000.0; // msec.
    self.x = self.translation_x_animation_from_value;
  }

  pub fn apply_translation_y_animation(&mut self, from_value: i32, to_value: i32, time: f32, easing: EasingFunction) {
    self.translation_y_animation_running = true;
    self.translation_y_animation_ease = easing;
    self.translation_y_animation_from_value = from_value;
    self.translation_y_animation_to_value = to_value;
    self.translation_y_animation_time_duration = time * 1000.0; // msec.
    self.x = self.translation_y_animation_from_value;
  }

  pub fn apply_rotation_animation(&mut self, from_value: i32, to_value: i32, time: f32, easing: EasingFunction) {
    self.rotation_animation_running = true;
    self.rotation_animation_ease = easing;
    self.rotation_animation_from_value = from_value;
    self.rotation_animation_to_value = to_value;
    self.rotation_animation_time_duration = time as u128 * 1000; // msec.
    self.rotation = self.rotation_animation_from_value;
  }

  pub fn apply_scale_animation(&mut self, from_value: f32, to_value: f32, time: f32, easing: EasingFunction) {
    self.scale_animation_running = true;
    self.scale_animation_ease = easing;
    self.scale_animation_from_value = from_value;
    self.scale_animation_to_value = to_value;
    self.scale_animation_time_duration = time as u128 * 1000; // msec.
    self.scale_x = self.scale_animation_from_value;
    self.scale_y = self.scale_animation_from_value;
  }

  pub fn handle_input(&mut self, key: usize) {
    if self.sub_actor_list.len() <= 0 {
        return;
    }

    if let Some(ref mut event_handler) = self.event_handelr {
      if key == 262 {     // right cursor
        if self.focused_sub_actor < self.sub_actor_list.len() - 1 {
          let prev_focused_sub_actor = self.focused_sub_actor;  
          self.focused_sub_actor += 1;

          self.sub_actor_list[self.focused_sub_actor].focused = true;
          event_handler.key_focus_in(key as u32, 
              &mut self.sub_actor_list[self.focused_sub_actor]);

          self.sub_actor_list[prev_focused_sub_actor].focused = false;
            event_handler.key_focus_out(key as u32, 
              &mut self.sub_actor_list[prev_focused_sub_actor]);
        }
      } else if key == 263 { // left cursor 
        if self.focused_sub_actor > 0 {
          let prev_focused_sub_actor = self.focused_sub_actor;
          self.focused_sub_actor -= 1;

          self.sub_actor_list[self.focused_sub_actor].focused = true;

          event_handler.key_focus_in(key as u32, 
            &mut self.sub_actor_list[self.focused_sub_actor]);
      
          self.sub_actor_list[prev_focused_sub_actor].focused = false;

          event_handler.key_focus_out(key as u32, 
              &mut self.sub_actor_list[prev_focused_sub_actor]);
        }
      }
    }
  }

  // Marks the layer’s contents as needing to be updated.
  pub fn set_needs_layout(&mut self) {
     self.needsUpdate = true;
  }

  // layout sub-actors.
  pub fn layout_sub_actors(&mut self) {
     if let Some(ref mut layout) = self.layout {
        layout.layout_sub_actors(&mut self.sub_actor_list);
    }
  }

  pub fn render(&self, shader_program: GLuint, stretch: &Stretch, actor: Option<&Actor>) {
    let layout = stretch.layout(self.node.unwrap()).unwrap();
    let mut x = layout.location.x;
    let mut y = layout.location.y;

    println!("node: {:#?}", stretch.layout(self.node.unwrap()));
    if let Some(main_actor) = actor {
      x += main_actor.x as f32;
      y += main_actor.y as f32;
    }
    
    println!("{}: x,y = {}, {}", self.name, x, y);

    let mut transform: Matrix4<f32> = Matrix4::identity();

    // Apply orthographic projection matrix: left, right, bottom, top, near, far
    transform = transform * cgmath::ortho(0.0, self.viewport_width as f32,
        self.viewport_height as f32, 0.0, 1.0, -1.0);

    //println!("{} {}", self.name, self.z);
    transform = transform *
        Matrix4::<f32>::from_translation(Vector3::new(
        x as f32, y as f32, self.z));

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

    for sub_actor in self.sub_actor_list.iter() {
      if sub_actor.focused == false {
        sub_actor.render(shader_program, stretch, Some(&self));
      }
    }

    // render the focused sub_actor at the end.
    if self.sub_actor_list.len() > 0 {
      self.sub_actor_list[self.focused_sub_actor].render(shader_program, stretch, Some(&self));
    }
  }

  pub fn add_sub_actor(&mut self, actor: Actor<'a>) {
    
    self.sub_actor_list.push(actor);
  }
}
