// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate gl;

use self::gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

use crate::stage::Stage;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec4 a_position;
    uniform mat4 transform;
    void main() {
      gl_Position = transform * a_position;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 outColor;
    uniform vec4 color;

    void main() {
       outColor = color;
    }
"#;

pub fn render(name:String) {
   println!("Render {}", name);
}

pub struct Play {
  name : String,
  stage_list: Vec<Stage>,
  shader_program: GLuint,
}

impl Play {
  pub fn new(w: u32, h: u32) -> Self {
    Play {
      name : "Test".to_string(),
      stage_list: Vec::new(),
      shader_program: 0,
    }
  }

pub fn initialize(&mut self) {
  self.compile_shader();
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

  pub fn add_stage(&mut self,  mut stage: Stage) {
    stage.initialize();
    self.stage_list.push(stage);
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
