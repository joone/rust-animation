// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::str;

use crate::stage::Stage;
use crate::actor::Actor;
use crate::actor::EventHandler;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec2 a_texCoord;

    uniform mat4 transform;
    out vec2 v_texCoord;

    void main() {
      gl_Position = transform * a_position;
      v_texCoord = a_texCoord;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 outColor;
    uniform vec4 color;

    in vec2 v_texCoord;
    uniform sampler2D s_texture;

    void main() {
      if (v_texCoord[0] == 0.0f)
        outColor = color;
      else {
        outColor = texture(s_texture, v_texCoord);
      }
    }
"#;

pub fn render(name:String) {
   println!("Render {}", name);
}

pub struct Play<'a>  {
  name : String,
  pub stage_list: Vec<Stage<'a>>,
  shader_program: GLuint,
  stage_map: HashMap<String, usize>
}

impl<'a> Play<'a>  {
  pub fn new(name: String) -> Self {
    Play {
      name : name,
      stage_list: Vec::new(),
      shader_program: 0,
      stage_map: HashMap::new()
    }
  }

pub fn initialize(&mut self) {
  self.compile_shader();
}

pub fn new_actor(name: String, w: u32, h: u32,
    event_handler: Option<Box<dyn EventHandler + 'a>>) -> Actor {
  Actor::new(name, w, h, event_handler)
}

pub fn add_new_actor_to_stage(&mut self, stage_name: &String, actor: Actor<'a>) {
  match self.stage_map.get(stage_name) {
      Some(&index) => {
        self.stage_list[index].add_actor(actor);
      },
      _ => println!("Can't find the stage with the given name: {}", stage_name)
  }
}

pub fn set_stage_needs_layout(&mut self, stage_name: &String) {
  match self.stage_map.get(stage_name) {
      Some(&index) => {
        self.stage_list[index].set_needs_layout();
      },
      _ => println!("Can't find the stage with the given name: {}", stage_name)
  }
}

pub fn set_visible_stage(&mut self, name: &String, visible: bool) {
    match self.stage_map.get(name) {
      Some(&index) => {
        self.stage_list[index].set_visible(visible);
      },
      _ => println!("Can't find the stage with the given name: {}", name)
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
          gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
          //println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
          let s = str::from_utf8(&info_log);
            match s {
            Err(_) => { println!("Failed to decode using"); }
            Ok(s) => { println!("Decoded with  to '{}'", s); }
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
          gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
          println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
      }

      // link shaders
      self.shader_program = gl::CreateProgram();
      gl::AttachShader(self.shader_program, vertex_shader);
      gl::AttachShader(self.shader_program, fragment_shader);
      gl::LinkProgram(self.shader_program);
      // check for linking errors
      gl::GetProgramiv(self.shader_program, gl::LINK_STATUS, &mut success);
      if success != gl::TRUE as GLint {
          gl::GetProgramInfoLog(self.shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
          println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
      }
      gl::DeleteShader(vertex_shader);
      gl::DeleteShader(fragment_shader);
    }
  }

  pub fn add_stage(&mut self, mut stage: Stage<'a>) -> String {
    stage.initialize();
    let stage_name = stage.name.to_string();
    self.stage_list.push(stage);
    self.stage_map.insert(stage_name.to_string(), self.stage_list.len() - 1);

    stage_name
  }

  pub fn handle_input(&mut self, key: usize) {
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
        stage.render(self.shader_program);
      }
    }
  }
}
