// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate image;
extern crate keyframe;

use cgmath::{Deg, Matrix4, SquareMatrix, Vector3};
use image::DynamicImage;
use std::path::Path;

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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  pub position: [f32; 3],
  pub tex_coords: [f32; 2],
}

impl Vertex {
  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x2,
        },
      ],
    }
  }
}

pub struct Layer {
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
  pub opacity: f32, // CoreAnimation-style property
  pub image_path: String,
  pub sub_layer_list: Vec<Layer>,
  pub(crate) vertex_buffer: Option<wgpu::Buffer>,
  pub(crate) index_buffer: Option<wgpu::Buffer>,
  pub(crate) texture: Option<wgpu::Texture>,
  pub(crate) texture_view: Option<wgpu::TextureView>,
  pub(crate) bind_group: Option<wgpu::BindGroup>,
  pub animated: bool,
  pub animation: Option<Animation>,
  animations: std::collections::HashMap<String, Animation>, // CoreAnimation-style animations by key
  event_handler: Option<Box<dyn EventHandler>>,
  layout: Option<Box<dyn Layout>>,
  pub(crate) focused_sub_layer: usize,
  pub(crate) focused: bool,
  pub needs_update: bool,
  pub node: Option<Node>,   // for stretch only
  pub style: Option<Style>, // for stretch only
}

pub trait EventHandler {
  fn key_focus_in(&mut self, layer: &mut Layer);
  fn key_focus_out(&mut self, layer: &mut Layer);
  fn key_down(&mut self, key: Key, layer: &mut Layer);
}

pub trait Layout {
  fn layout_sub_layers(
    &mut self,
    layer: &mut Layer,
    parent_layer: Option<&Layer>,
    stretch: &mut Option<Stretch>,
  );
  fn update_layout(&mut self, layer: &mut Layer, stretch: &mut Option<Stretch>);
  fn finalize(&mut self);
}

impl Layer {
  pub fn new(name: String, w: u32, h: u32, event_handler: Option<Box<dyn EventHandler>>) -> Self {
    let layer = Layer {
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
      opacity: 1.0,
      image_path: "".to_string(),
      sub_layer_list: Vec::new(),
      vertex_buffer: None,
      index_buffer: None,
      texture: None,
      texture_view: None,
      bind_group: None,
      animated: false,
      animation: None,
      animations: std::collections::HashMap::new(),
      event_handler: event_handler,
      layout: None,
      focused_sub_layer: 0,
      focused: false,
      needs_update: true,
      node: None,
      style: None,
    };

    layer
  }

  #[allow(unused_variables)]
  pub fn init_buffers(&mut self, device: &wgpu::Device) {
    // Skip buffer initialization during tests
    #[cfg(test)]
    {
      return;
    }

    #[cfg(not(test))]
    {
      let vertices = [
        Vertex {
          position: [self.width as f32, self.height as f32, 0.0],
          tex_coords: [1.0, 1.0],
        }, // top right
        Vertex {
          position: [self.width as f32, 0.0, 0.0],
          tex_coords: [1.0, 0.0],
        }, // bottom right
        Vertex {
          position: [0.0, 0.0, 0.0],
          tex_coords: [0.0, 0.0],
        }, // bottom left
        Vertex {
          position: [0.0, self.height as f32, 0.0],
          tex_coords: [0.0, 1.0],
        }, // top left
      ];

      let indices: [u16; 6] = [0, 1, 3, 1, 2, 3];

      use wgpu::util::DeviceExt;
      self.vertex_buffer = Some(
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Vertex Buffer"),
          contents: bytemuck::cast_slice(&vertices),
          usage: wgpu::BufferUsages::VERTEX,
        }),
      );

      self.index_buffer = Some(
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Index Buffer"),
          contents: bytemuck::cast_slice(&indices),
          usage: wgpu::BufferUsages::INDEX,
        }),
      );
    }
  }

  pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
    self.color[0] = r;
    self.color[1] = g;
    self.color[2] = b;
  }

  pub fn set_text(&mut self, text: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
    let mut font_renderer: FontRenderer = FontRenderer::new("fonts/DejaVuSans.ttf".to_string());
    let image = font_renderer.render(text);
    let dynamic_image = DynamicImage::ImageRgba8(image);

    dynamic_image.save("temp.png").unwrap();

    self.image_path = "temp".to_string();
    self.width = dynamic_image.width();
    self.height = dynamic_image.height();

    println!("width: {}, height: {}", self.width, self.height);

    let rgba = dynamic_image.to_rgba8();
    let dimensions = rgba.dimensions();

    let texture_size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some("Text Texture"),
      size: texture_size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      view_formats: &[],
    });

    queue.write_texture(
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      &rgba,
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1),
      },
      texture_size,
    );

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    self.texture = Some(texture);
    self.texture_view = Some(texture_view);
  }

  /// Set image path (for backward compatibility - actual texture loading requires wgpu context)
  pub fn set_image(&mut self, path: String) {
    self.image_path = path;
  }

  /// Load image with wgpu context
  pub fn load_image_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
    if !self.image_path.is_empty() {
      // Use format auto-detection to handle images with incorrect extensions
      let result = image::ImageReader::open(&Path::new(&self.image_path))
        .and_then(|reader| reader.with_guessed_format())
        .map_err(|e| image::ImageError::IoError(e))
        .and_then(|reader| reader.decode());
      
      match result {
        Ok(img) => {
          let rgba = img.to_rgba8();
          let dimensions = rgba.dimensions();

          let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
          };

          let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Image Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
          });

          queue.write_texture(
            wgpu::ImageCopyTexture {
              texture: &texture,
              mip_level: 0,
              origin: wgpu::Origin3d::ZERO,
              aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
              offset: 0,
              bytes_per_row: Some(4 * dimensions.0),
              rows_per_image: Some(dimensions.1),
            },
            texture_size,
          );

          let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
          self.texture = Some(texture);
          self.texture_view = Some(texture_view);
        }
        Err(err) => println!("Fail to load a image {:?}", err),
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
    self.sub_layer_list.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
  }*/

  pub fn animate(&mut self) {
    // Run legacy animation if present
    if let Some(mut animation) = self.animation.take() {
      animation.run(self);
      self.animation = Some(animation);
    }

    // Run CoreAnimation-style animations
    // Take the animations HashMap out temporarily
    let mut animations = std::mem::take(&mut self.animations);
    for (_key, animation) in animations.iter_mut() {
      animation.run(self);
    }
    // Put it back
    self.animations = animations;

    for sub_layer in self.sub_layer_list.iter_mut() {
      sub_layer.animate();
    }
  }

  pub fn select_next_sub_layer(&mut self) {
    if self.sub_layer_list.is_empty() {
      return;
    }
    // no more next layer.
    if self.focused_sub_layer < self.sub_layer_list.len() - 1 {
      let prev_focused_sub_layer = self.focused_sub_layer;
      self.focused_sub_layer += 1;
      self.sub_layer_list[self.focused_sub_layer].set_focus(true);
      self.sub_layer_list[prev_focused_sub_layer].set_focus(false);
    }
  }

  pub fn select_prev_sub_layer(&mut self) {
    if self.sub_layer_list.is_empty() {
      return;
    }
    // ne more previous layer.
    if self.focused_sub_layer == 0 {
      return;
    }
    let prev_focused_sub_layer = self.focused_sub_layer;
    self.focused_sub_layer -= 1;
    self.sub_layer_list[self.focused_sub_layer].set_focus(true);
    self.sub_layer_list[prev_focused_sub_layer].set_focus(false);
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
    for sub_layer in self.sub_layer_list.iter_mut() {
      if sub_layer.focused {
        sub_layer.handle_input(key);
      }
    }
    if let Some(mut event_handler) = self.event_handler.take() {
      event_handler.key_down(key, self);
      self.event_handler = Some(event_handler);
    }
  }

  pub fn layout_sub_layers(
    &mut self,
    parent_layer: Option<&Layer>,
    stretch: &mut Option<Stretch>,
  ) {
    if let Some(mut layout) = self.layout.take() {
      layout.layout_sub_layers(self, parent_layer, stretch);
      self.layout = Some(layout); // Put back the layout
    }

    // Replace the sub_layer_list with an empty vector and take the original vector out
    let mut sub_layer_list = std::mem::replace(&mut self.sub_layer_list, Vec::new());

    // Iterate over the vector outside of the self structure
    for sub_layer in &mut sub_layer_list {
      // As we are outside of the self structure, we can now borrow self as immutable
      sub_layer.layout_sub_layers(Some(self), stretch);
    }

    // Put back the original sub_layer_list
    self.sub_layer_list = sub_layer_list;
  }

  pub fn update_layout(&mut self, stretch: &mut Option<Stretch>) {
    if let Some(mut layout) = self.layout.take() {
      layout.update_layout(self, stretch);
      self.layout = Some(layout); // Put back the layout
    }

    for sub_layer in self.sub_layer_list.iter_mut() {
      sub_layer.update_layout(stretch);
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

  pub fn render(&mut self, parent_model_matrix: Option<&Matrix4<f32>>, projection: &Matrix4<f32>) {
    if !self.visible {
      return;
    }

    let mut transform: Matrix4<f32> = self.model_matrix();
    if let Some(parent_model_matrix) = parent_model_matrix {
      transform = transform * parent_model_matrix;
    }

    // Rendering will be handled by the Play struct with wgpu
    // This method now just updates the transform hierarchy

    for sub_layer in self.sub_layer_list.iter_mut() {
      if sub_layer.focused == false {
        sub_layer.render(Some(&transform), projection);
      }
    }

    // render the focused sub_layer at the end.
    if !self.sub_layer_list.is_empty() {
      self.sub_layer_list[self.focused_sub_layer].render(Some(&transform), projection);
    }
  }

  pub fn add_sub_layer(&mut self, layer: Layer) {
    self.sub_layer_list.push(layer);
  }

  // CoreAnimation-style API methods

  /// Set position (CoreAnimation-style API)
  pub fn set_position(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }

  /// Get position as tuple (CoreAnimation-style API)
  pub fn position(&self) -> (i32, i32) {
    (self.x, self.y)
  }

  /// Set bounds (CoreAnimation-style API)
  pub fn set_bounds(&mut self, width: u32, height: u32) {
    self.width = width;
    self.height = height;
  }

  /// Get bounds as tuple (CoreAnimation-style API)
  pub fn bounds(&self) -> (u32, u32) {
    (self.width, self.height)
  }

  /// Set opacity (CoreAnimation-style API)
  pub fn set_opacity(&mut self, opacity: f32) {
    self.opacity = opacity.max(0.0).min(1.0);
  }

  /// Set background color (CoreAnimation-style API)
  pub fn set_background_color(&mut self, r: f32, g: f32, b: f32) {
    self.set_color(r, g, b);
  }

  /// Get background color (CoreAnimation-style API)
  pub fn background_color(&self) -> (f32, f32, f32) {
    (self.color[0], self.color[1], self.color[2])
  }

  /// Add an animation for a specific key (CoreAnimation-style API)
  pub fn add_animation(&mut self, animation: Animation, key: Option<&str>) {
    if let Some(key_str) = key {
      self.animations.insert(key_str.to_string(), animation);
    } else {
      // If no key provided, use the legacy animation field
      self.animation = Some(animation);
    }
  }

  /// Remove all animations (CoreAnimation-style API)
  pub fn remove_all_animations(&mut self) {
    self.animations.clear();
    self.animation = None;
  }

  /// Remove animation for a specific key (CoreAnimation-style API)
  pub fn remove_animation(&mut self, key: &str) {
    self.animations.remove(key);
  }

  /// Add a sublayer (CoreAnimation-style API, alias for add_sub_layer)
  pub fn add_sublayer(&mut self, layer: Layer) {
    self.add_sub_layer(layer);
  }

  /// Get sublayers (CoreAnimation-style API)
  pub fn sublayers(&self) -> &Vec<Layer> {
    &self.sub_layer_list
  }

  /// Get mutable sublayers (CoreAnimation-style API)
  pub fn sublayers_mut(&mut self) -> &mut Vec<Layer> {
    &mut self.sub_layer_list
  }

  /// Create uniform buffer with transform matrix and color
  pub fn create_uniform_buffer(
    &self,
    device: &wgpu::Device,
    transform: &Matrix4<f32>,
    projection: &Matrix4<f32>,
  ) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;

    #[repr(C)]
    #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
    struct Uniforms {
      transform: [[f32; 4]; 4],
      projection: [[f32; 4]; 4],
      color: [f32; 4],
      use_texture: u32,
      _padding: [u32; 3],
    }

    let use_texture = if self.texture.is_some() { 1 } else { 0 };
    
    let uniforms = Uniforms {
      transform: (*transform).into(),
      projection: (*projection).into(),
      color: [self.color[0], self.color[1], self.color[2], self.opacity],
      use_texture,
      _padding: [0; 3],
    };

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Uniform Buffer"),
      contents: bytemuck::cast_slice(&[uniforms]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
  }

  /// Get or create texture bind group
  pub fn get_or_create_bind_group(
    &mut self,
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    default_texture_view: &wgpu::TextureView,
  ) -> &wgpu::BindGroup {
    if self.bind_group.is_none() {
      let texture_view = self.texture_view.as_ref().unwrap_or(default_texture_view);
      
      self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(texture_view),
          },
          wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(sampler),
          },
        ],
      }));
    }
    
    self.bind_group.as_ref().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::animation::{Animation, EasingFunction};

  #[test]
  fn test_position_api() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);
    layer.set_position(50, 75);
    let (x, y) = layer.position();
    assert_eq!(x, 50);
    assert_eq!(y, 75);
  }

  #[test]
  fn test_bounds_api() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);
    layer.set_bounds(200, 150);
    let (w, h) = layer.bounds();
    assert_eq!(w, 200);
    assert_eq!(h, 150);
  }

  #[test]
  fn test_opacity_api() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);
    assert_eq!(layer.opacity, 1.0);

    layer.set_opacity(0.5);
    assert_eq!(layer.opacity, 0.5);

    // Test clamping
    layer.set_opacity(1.5);
    assert_eq!(layer.opacity, 1.0);

    layer.set_opacity(-0.5);
    assert_eq!(layer.opacity, 0.0);
  }

  #[test]
  fn test_background_color_api() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);
    layer.set_background_color(0.5, 0.6, 0.7);
    let (r, g, b) = layer.background_color();
    assert_eq!(r, 0.5);
    assert_eq!(g, 0.6);
    assert_eq!(b, 0.7);
  }

  #[test]
  fn test_add_animation_with_key() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);
    let mut animation = Animation::with_key_path("position.x");
    animation.duration = 2.0;
    animation.timing_function = Some(EasingFunction::Linear);

    layer.add_animation(animation, Some("moveX"));
    assert_eq!(layer.animations.len(), 1);
    assert!(layer.animations.contains_key("moveX"));
  }

  #[test]
  fn test_remove_animation() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);
    let animation1 = Animation::with_key_path("position.x");
    let animation2 = Animation::with_key_path("opacity");

    layer.add_animation(animation1, Some("anim1"));
    layer.add_animation(animation2, Some("anim2"));
    assert_eq!(layer.animations.len(), 2);

    layer.remove_animation("anim1");
    assert_eq!(layer.animations.len(), 1);
    assert!(!layer.animations.contains_key("anim1"));
    assert!(layer.animations.contains_key("anim2"));
  }

  #[test]
  fn test_remove_all_animations() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);
    let animation1 = Animation::with_key_path("position.x");
    let animation2 = Animation::with_key_path("opacity");
    let animation3 = Animation::new();

    layer.add_animation(animation1, Some("anim1"));
    layer.add_animation(animation2, Some("anim2"));
    layer.set_animation(Some(animation3));

    assert_eq!(layer.animations.len(), 2);
    assert!(layer.animation.is_some());

    layer.remove_all_animations();
    assert_eq!(layer.animations.len(), 0);
    assert!(layer.animation.is_none());
  }

  #[test]
  fn test_sublayers_api() {
    let mut parent = Layer::new("parent".to_string(), 200, 200, None);
    let child1 = Layer::new("child1".to_string(), 50, 50, None);
    let child2 = Layer::new("child2".to_string(), 50, 50, None);

    parent.add_sublayer(child1);
    parent.add_sublayer(child2);

    let sublayers = parent.sublayers();
    assert_eq!(sublayers.len(), 2);
    assert_eq!(sublayers[0].name, "child1");
    assert_eq!(sublayers[1].name, "child2");
  }

  #[test]
  fn test_backward_compatibility() {
    let mut layer = Layer::new("test".to_string(), 100, 100, None);

    // Old way of setting position
    layer.x = 50;
    layer.y = 75;
    assert_eq!(layer.x, 50);
    assert_eq!(layer.y, 75);

    // Old way of creating animation
    let mut animation = Animation::new();
    animation.apply_translation_x(0, 100, 1.0, EasingFunction::Linear);
    layer.set_animation(Some(animation));

    assert!(layer.animation.is_some());
  }
}
