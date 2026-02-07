// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use cgmath::{Matrix4, SquareMatrix};
use std::collections::HashMap;
use stretch::{geometry::Size, node::Stretch};

use crate::layer::EventHandler;
use crate::layer::Key;
use crate::layer::LayoutMode;
use crate::layer::Layer;
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
  stage_list: Vec<Layer>,
  stage_map: HashMap<String, usize>,
  projection: Matrix4<f32>,
  pub stretch: Option<Stretch>,
  pub wgpu_context: Option<WgpuContext>,
  render_pipeline: Option<wgpu::RenderPipeline>,
  bind_group_layout: Option<wgpu::BindGroupLayout>,
  texture_bind_group_layout: Option<wgpu::BindGroupLayout>,
  default_texture: Option<wgpu::Texture>,
  default_texture_view: Option<wgpu::TextureView>,
  sampler: Option<wgpu::Sampler>,
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
      render_pipeline: None,
      bind_group_layout: None,
      texture_bind_group_layout: None,
      default_texture: None,
      default_texture_view: None,
      sampler: None,
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

  /// Initialize wgpu with a window surface for rendering to screen
  pub fn init_wgpu_with_surface(
    &mut self,
    window: impl Into<wgpu::SurfaceTarget<'static>>,
    width: u32,
    height: u32,
  ) {
    self.wgpu_context = Some(pollster::block_on(WgpuContext::new_with_surface(
      window, width, height,
    )));
    
    // Set up the render pipeline after wgpu context is created
    self.setup_render_pipeline();
    self.create_default_texture();
  }

  /// Set up the render pipeline for drawing
  fn setup_render_pipeline(&mut self) {
    let Some(ref context) = self.wgpu_context else {
      return;
    };

    let device = &context.device;

    // Create shader module
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Shader"),
      source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
    });

    // Create bind group layout for uniforms
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("Uniform Bind Group Layout"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    // Create bind group layout for texture
    let texture_bind_group_layout =
      device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
              multisampled: false,
              view_dimension: wgpu::TextureViewDimension::D2,
              sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
          },
        ],
      });

    // Create pipeline layout
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[&bind_group_layout, &texture_bind_group_layout],
      push_constant_ranges: &[],
    });

    // Get surface format
    let surface_format = context
      .surface_config
      .as_ref()
      .map(|c| c.format)
      .unwrap_or(wgpu::TextureFormat::Bgra8UnormSrgb);

    // Create render pipeline
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[crate::layer::Vertex::desc()],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(wgpu::ColorTargetState {
          format: surface_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      multiview: None,
      cache: None,
    });

    self.render_pipeline = Some(render_pipeline);
    self.bind_group_layout = Some(bind_group_layout);
    self.texture_bind_group_layout = Some(texture_bind_group_layout);

    // Create sampler
    self.sampler = Some(device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Linear,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    }));
  }

  /// Create a default 1x1 white texture for layers without textures
  fn create_default_texture(&mut self) {
    let Some(ref context) = self.wgpu_context else {
      return;
    };

    let device = &context.device;
    let queue = &context.queue;

    let texture_size = wgpu::Extent3d {
      width: 1,
      height: 1,
      depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some("Default White Texture"),
      size: texture_size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      view_formats: &[],
    });

    // Write white color to the texture
    queue.write_texture(
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      &[255, 255, 255, 255],
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4),
        rows_per_image: Some(1),
      },
      texture_size,
    );

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    self.default_texture = Some(texture);
    self.default_texture_view = Some(texture_view);
  }

  /// Resize the rendering surface and update projection matrix
  pub fn resize(&mut self, width: u32, height: u32) {
    if let Some(ref mut context) = self.wgpu_context {
      context.resize(width, height);
    }
    
    // Update projection matrix for new viewport size
    let orth_matrix = cgmath::ortho(
      0.0,
      width as f32,
      height as f32,
      0.0,
      1.0,
      -1.0,
    );
    self.projection = orth_matrix;
  }

  /// Recursively render a layer and its sublayers
  fn render_layer(
    layer: &mut Layer,
    render_pass: &mut wgpu::RenderPass,
    parent_transform: Option<&Matrix4<f32>>,
    context: &WgpuContext,
    projection: &Matrix4<f32>,
    render_pipeline: &wgpu::RenderPipeline,
    bind_group_layout: &wgpu::BindGroupLayout,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    default_texture_view: &wgpu::TextureView,
  ) {
    if !layer.visible {
      return;
    }

    // Initialize buffers if needed
    if layer.vertex_buffer.is_none() || layer.index_buffer.is_none() {
      layer.init_buffers(&context.device);
    }

    // Load texture if image path is set and texture not loaded
    if !layer.image_path.is_empty() && layer.texture.is_none() {
      layer.load_image_texture(&context.device, &context.queue);
    }

    // Early return if buffers don't exist
    if layer.vertex_buffer.is_none() || layer.index_buffer.is_none() {
      return;
    }

    // Calculate transform
    let mut transform = layer.model_matrix();
    if let Some(parent) = parent_transform {
      transform = transform * parent;
    }

    // Create uniform buffer
    let uniform_buffer = layer.create_uniform_buffer(&context.device, &transform, projection);

    // Create bind group for uniforms
    let uniform_bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("Uniform Bind Group"),
      layout: bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: uniform_buffer.as_entire_binding(),
      }],
    });

    // Get or create texture bind group
    let texture_bind_group = layer.get_or_create_bind_group(
      &context.device,
      texture_bind_group_layout,
      sampler,
      default_texture_view,
    );

    // Set pipeline and bindings
    render_pass.set_pipeline(render_pipeline);
    render_pass.set_bind_group(0, &uniform_bind_group, &[]);
    render_pass.set_bind_group(1, texture_bind_group, &[]);
    render_pass.set_vertex_buffer(0, layer.vertex_buffer.as_ref().unwrap().slice(..));
    render_pass.set_index_buffer(
      layer.index_buffer.as_ref().unwrap().slice(..),
      wgpu::IndexFormat::Uint16,
    );
    render_pass.draw_indexed(0..6, 0, 0..1);

    // Render sublayers (non-focused first, then focused)
    for (i, sub_layer) in layer.sub_layer_list.iter_mut().enumerate() {
      if !sub_layer.focused && i != layer.focused_sub_layer {
        Self::render_layer(
          sub_layer,
          render_pass,
          Some(&transform),
          context,
          projection,
          render_pipeline,
          bind_group_layout,
          texture_bind_group_layout,
          sampler,
          default_texture_view,
        );
      }
    }

    // Render focused sublayer last
    if !layer.sub_layer_list.is_empty() && layer.focused_sub_layer < layer.sub_layer_list.len() {
      Self::render_layer(
        &mut layer.sub_layer_list[layer.focused_sub_layer],
        render_pass,
        Some(&transform),
        context,
        projection,
        render_pipeline,
        bind_group_layout,
        texture_bind_group_layout,
        sampler,
        default_texture_view,
      );
    }
  }

  pub fn new_layer(
    name: String,
    w: u32,
    h: u32,
    event_handler: Option<Box<dyn EventHandler>>,
  ) -> Layer {
    Layer::new(name, w, h, event_handler)
  }

  pub fn add_new_layer_to_stage(&mut self, stage_name: &String, layer: Layer) {
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

  pub fn add_stage(&mut self, stage: Layer) -> String {
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
    // Update animations and layout
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

    // Perform actual wgpu rendering if surface is available
    if let Some(ref context) = self.wgpu_context {
      if let Some(ref surface) = context.surface {
        let output = match surface.get_current_texture() {
          Ok(output) => output,
          Err(e) => {
            eprintln!("Failed to get current texture: {:?}", e);
            return;
          }
        };

        let view = output
          .texture
          .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
          .device
          .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
          });

        {
          let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
              view: &view,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                  r: 0.1,
                  g: 0.2,
                  b: 0.3,
                  a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
              },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
          });
          
          // Render all stages
          for stage in self.stage_list.iter_mut() {
            Self::render_layer(
              stage,
              &mut render_pass,
              None,
              context,
              &self.projection,
              self.render_pipeline.as_ref().unwrap(),
              self.bind_group_layout.as_ref().unwrap(),
              self.texture_bind_group_layout.as_ref().unwrap(),
              self.sampler.as_ref().unwrap(),
              self.default_texture_view.as_ref().unwrap(),
            );
          }
        }

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
      }
    }
  }
}
