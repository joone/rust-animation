// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use cgmath::{Matrix4, SquareMatrix};
use std::collections::HashMap;
use stretch::{geometry::Size, node::Stretch};

use crate::layer::EventHandler;
use crate::layer::Key;
use crate::layer::LayoutMode;
use crate::layer::RALayer;
use crate::wgpu_context::WgpuContext;

// WGSL shader source
const SHADER_SOURCE: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Uniforms {
    transform: mat4x4<f32>,
    projection: mat4x4<f32>,
    color: vec4<f32>,
    use_texture: u32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var t_texture: texture_2d<f32>;
@group(1) @binding(1)
var t_sampler: sampler;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.projection * uniforms.transform * vec4<f32>(vertex.position, 1.0);
    out.tex_coords = vertex.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (uniforms.use_texture > 0u) {
        return textureSample(t_texture, t_sampler, in.tex_coords);
    } else {
        return uniforms.color;
    }
}
"#;

pub fn render(name: String) {
  println!("Render {}", name);
}

pub struct Play {
  _name: String,
  stage_list: Vec<RALayer>,
  stage_map: HashMap<String, usize>,
  projection: Matrix4<f32>,
  pub stretch: Option<Stretch>,
  pub wgpu_context: Option<WgpuContext>,
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
      stage_map: HashMap::new(),
      projection: Matrix4::identity(),
      stretch: stretch,
      wgpu_context: None,
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

    play
  }

  pub fn init_wgpu(&mut self) {
    // Initialize wgpu context (offscreen for library use)
    self.wgpu_context = Some(pollster::block_on(WgpuContext::new_offscreen()));
  }

  pub fn new_layer(
    name: String,
    w: u32,
    h: u32,
    event_handler: Option<Box<dyn EventHandler>>,
  ) -> RALayer {
    RALayer::new(name, w, h, event_handler)
  }

  pub fn add_new_layer_to_stage(&mut self, stage_name: &String, layer: RALayer) {
    match self.stage_map.get(stage_name) {
      Some(&index) => {
        self.stage_list[index].add_sub_layer(layer);
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

  pub fn add_stage(&mut self, stage: RALayer) -> String {
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
    // wgpu rendering would happen here in a full implementation
    // For now, we just update animations and layout

    for stage in self.stage_list.iter_mut() {
      if stage.needs_update {
        stage.layout_sub_layers(None, &mut self.stretch);

        if let Some(stretch_obj) = &mut self.stretch {
          stretch_obj
            .compute_layout(stage.node.unwrap(), Size::undefined())
            .unwrap();
        }

        stage.update_layout(&mut self.stretch);
        stage.needs_update = false;
      }

      stage.animate();
      stage.render(None, &self.projection);
    }
  }
}
