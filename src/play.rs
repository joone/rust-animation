// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use cgmath::{Deg, Matrix, Matrix4, SquareMatrix, Vector3};
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::str;
use stretch::{geometry::Size, node::Stretch, style::*};

use crate::actor::Actor;
use crate::actor::EventHandler;
use crate::actor::Key;
use crate::actor::LayoutMode;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec2 a_texCoord;

    uniform mat4 transform;
    uniform mat4 projection;   
    out vec2 v_texCoord;

    void main() {
      gl_Position = projection * transform * a_position;
      v_texCoord = a_texCoord;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 outColor;
    uniform vec4 color;
    uniform int useTexture; // Flag to determine whether to use the texture
    in vec2 v_texCoord;
    uniform sampler2D s_texture;

    void main() {
      if (useTexture > 0)
        outColor = texture(s_texture, v_texCoord);
      else
        outColor = color;
    }
"#;

pub fn render(name: String) {
  println!("Render {}", name);
}

pub struct Play {
  _name: String,
  // `Play` holds a list of `Stage`s, each of which will share the same lifetime `'a`
  stage_list: Vec<Actor>,
  shader_program: GLuint,
  stage_map: HashMap<String, usize>,
  projection: Matrix4<f32>,
  pub stretch: Option<Stretch>,
}

impl Play {
  pub fn new(
    name: String,
    viewport_width: i32,
    viewport_height: i32,
    layout_mode: LayoutMode,
  ) -> Self {
    let mut stretch = None;
    match layout_mode {
      LayoutMode::Flex => {
        stretch = Some(Stretch::new());
      }
      LayoutMode::UserDefine => {
        print!("UserDefine");
      }
    }

    let mut play = Play {
      _name: name,
      stage_list: Vec::new(),
      shader_program: 0,
      stage_map: HashMap::new(),
      projection: Matrix4::identity(),
      stretch: stretch,
    };

    // Apply orthographic projection matrix: left, right, bottom, top, near, far
    let orth_matrix = cgmath::ortho(
      0.0,
      viewport_width as f32,
      viewport_height as f32,
      0.0,
      1.0,
      -1.0,
    );
    play.projection = orth_matrix;
    //self.stretch = Some(Stretch::new());
    play.compile_shader();

    play
  }

  pub fn new_actor(
    name: String,
    w: u32,
    h: u32,
    event_handler: Option<Box<dyn EventHandler>>,
  ) -> Actor {
    Actor::new(name, w, h, event_handler)
  }

  pub fn add_new_actor_to_stage(&mut self, stage_name: &String, actor: Actor) {
    match self.stage_map.get(stage_name) {
      Some(&index) => {
        self.stage_list[index].add_sub_actor(actor);
      }
      _ => println!("Can't find the stage with the given name: {}", stage_name),
    }
  }

  pub fn set_visible_stage(&mut self, name: &String, visible: bool) {
    match self.stage_map.get(name) {
      Some(&index) => {
        self.stage_list[index].set_visible(visible);
        self.stage_list[index].needs_update = true;
      }
      _ => println!("Can't find the stage with the given name: {}", name),
    }
  }

  // https://github.com/bwasty/learn-opengl-rs/blob/master/src/_1_getting_started/_2_1_hello_triangle.rs
  fn compile_shader(&mut self) {
    unsafe {
      // build and compile our shader program
      // vertex shader
      let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
      let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
      gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
      gl::CompileShader(vertex_shader);

      // check for shader compile errors
      let mut success = gl::FALSE as GLint;
      let mut info_log = Vec::with_capacity(512);
      info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
      gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
      if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
          vertex_shader,
          512,
          ptr::null_mut(),
          info_log.as_mut_ptr() as *mut GLchar,
        );
        //println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        let s = str::from_utf8(&info_log);
        match s {
          Err(_) => {
            println!("Failed to decode using");
          }
          Ok(s) => {
            println!("Decoded with  to '{}'", s);
          }
        }
      }

      // fragment shader
      let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
      let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
      gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
      gl::CompileShader(fragment_shader);
      // check for shader compile errors
      gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
      if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
          fragment_shader,
          512,
          ptr::null_mut(),
          info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
          "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
          str::from_utf8(&info_log).unwrap()
        );
      }

      // link shaders
      self.shader_program = gl::CreateProgram();
      gl::AttachShader(self.shader_program, vertex_shader);
      gl::AttachShader(self.shader_program, fragment_shader);
      gl::LinkProgram(self.shader_program);
      // check for linking errors
      gl::GetProgramiv(self.shader_program, gl::LINK_STATUS, &mut success);
      if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(
          self.shader_program,
          512,
          ptr::null_mut(),
          info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
          "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
          str::from_utf8(&info_log).unwrap()
        );
      }
      gl::DeleteShader(vertex_shader);
      gl::DeleteShader(fragment_shader);
    }
  }

  pub fn add_stage(&mut self, stage: Actor) -> String {
    let stage_name = stage.name.to_string();
    self.stage_list.push(stage);
    self
      .stage_map
      .insert(stage_name.to_string(), self.stage_list.len() - 1);

    stage_name
  }

  pub fn handle_input(&mut self, key: Key) {
    // println!("key: {}", key);
    for stage in self.stage_list.iter_mut() {
      stage.handle_input(key);
    }
  }

  pub fn render(&mut self) {
    unsafe {
      gl::ClearColor(0.2, 0.3, 0.3, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      for stage in self.stage_list.iter_mut() {
        if stage.needs_update {
          stage.layout_sub_actors(None, &mut self.stretch);

          if let Some(stretch_obj) = &mut self.stretch {
            stretch_obj
              .compute_layout(stage.node.unwrap(), Size::undefined())
              .unwrap();

            //let layout = stretch_obj.layout(self.stage_actor.node.unwrap()).unwrap();
            //println!("set_needs_layout {}, {}", layout.size.width, layout.size.height);
          }

          stage.update_layout(&mut self.stretch);
          stage.needs_update = false;
        }

        stage.animate();
        stage.render(self.shader_program, None, &self.projection);
      }
    }
  }
}
