// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::Arc;
use stretch::{geometry::Rect, geometry::Size, node::Stretch, style::*};
use winit::{
  event::{Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::WindowBuilder,
};

use rust_animation::layer::Layout;
use rust_animation::layer::LayoutMode;
use rust_animation::layer::Layer;
use rust_animation::play::Play;

pub struct FlexLayout {
  name: String,
}

impl FlexLayout {
  pub fn new() -> Self {
    let flex_layout = FlexLayout {
      name: "flex_layout".to_string(),
    };

    println!("new FlexLayout {}", flex_layout.name);

    flex_layout
  }
}

impl Layout for FlexLayout {
  fn layout_sub_layers(
    &mut self,
    layer: &mut Layer,
    parent_layer: Option<&Layer>,
    stretch: &mut Option<Stretch>,
  ) {
    println!("run layout_sub_layer for FlexLayout {}", self.name);
    if let Some(stretch_obj) = stretch {
      if let Some(style_obj) = layer.style {
        layer.node = Some(stretch_obj.new_node(style_obj, vec![]).unwrap());
      } else {
        //println!("default style: {}: {},{}", self.name, self.width, self.height);
        layer.node = Some(
          stretch_obj
            .new_node(
              Style {
                size: Size {
                  width: Dimension::Points(layer.width as f32),
                  height: Dimension::Points(layer.height as f32),
                },
                margin: Rect {
                  start: Dimension::Points(2.0),
                  end: Dimension::Points(2.0),
                  top: Dimension::Points(2.0),
                  bottom: Dimension::Points(2.0),
                  ..Default::default()
                },
                ..Default::default()
              },
              vec![],
            )
            .unwrap(),
        );
      }

      println!("layer name {}", layer.name);

      if let Some(parent_layer) = parent_layer {
        if !parent_layer.node.is_none() && !layer.node.is_none() {
          match stretch_obj.add_child(parent_layer.node.unwrap(), layer.node.unwrap()) {
            Ok(()) => {
              println!(
                " stretch node  is added {} {}",
                parent_layer.name, layer.name
              )
            }
            Err(..) => {}
          }
        }
      }
    }

    //self.update_layout(layer, stretch);
  }

  fn update_layout(&mut self, layer: &mut Layer, stretch: &mut Option<Stretch>) {
    if let Some(stretch_obj) = stretch {
      if !layer.node.is_none() {
        let layout = stretch_obj.layout(layer.node.unwrap()).unwrap();
        layer.x = layout.location.x as i32;
        layer.y = layout.location.y as i32;
        println!(
          "run update_layout for FlexLayout {} = {},{}",
          layer.name, layer.x, layer.y
        );
      }
    }
  }

  fn finalize(&mut self) {
    println!("finalize {}", self.name);
  }
}

fn main() {
  let event_loop = EventLoop::new().unwrap();
  let window = Arc::new(
    WindowBuilder::new()
      .with_title("Flex UI demo")
      .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
      .build(&event_loop)
      .unwrap(),
  );

  // Get the actual window size (may differ from requested due to DPI scaling)
  let window_size = window.inner_size();
  let (width, height) = (window_size.width, window_size.height);

  let mut play = Play::new("Flex UI test".to_string(), width as i32, height as i32, LayoutMode::Flex);

  // Initialize wgpu context with surface using actual window size
  play.init_wgpu_with_surface(window.clone(), width, height);

  let mut stage = Layer::new("stage".to_string(), width, height, None);
  stage.set_style(Style {
    size: Size {
      width: Dimension::Points(width as f32),
      height: Dimension::Points(height as f32),
    },
    justify_content: JustifyContent::Center,
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    margin: Rect {
      start: Dimension::Points(1.0),
      end: Dimension::Points(1.0),
      top: Dimension::Points(1.0),
      bottom: Dimension::Points(1.0),
      ..Default::default()
    },
    ..Default::default()
  });
  stage.set_visible(true);

  let justify_content = vec![
    JustifyContent::FlexStart,
    JustifyContent::FlexEnd,
    JustifyContent::Center,
    JustifyContent::SpaceBetween,
    JustifyContent::SpaceAround,
    JustifyContent::SpaceEvenly,
  ];
  let width = 1500;
  let height = 108;
  for i in 0..6 {
    let layer_name = format!("layer_{}", i + 1);
    let mut layer = Layer::new(layer_name.to_string(), width, height, None);
    layer.set_color(i as f32 / 6.0, i as f32 / 6.0, i as f32 / 6.0);
    layer.set_style(Style {
      size: Size {
        width: Dimension::Points(width as f32),
        height: Dimension::Points(height as f32),
      },
      justify_content: justify_content[i],
      align_items: AlignItems::Center,
      margin: Rect {
        start: Dimension::Points(1.0),
        end: Dimension::Points(1.0),
        top: Dimension::Points(1.0),
        bottom: Dimension::Points(1.0),
        ..Default::default()
      },
      padding: Rect {
        start: Dimension::Points(2.0),
        end: Dimension::Points(2.0),
        ..Default::default()
      },
      ..Default::default()
    });
    for j in 0..10 {
      let mut sub_layer = Layer::new(
        format!("layer_{}_{}", i + 1, j + 1).to_string(),
        100,
        100,
        None,
      );
      sub_layer.set_color(1.0, j as f32 / 10.0, j as f32 / 10.0);
      sub_layer.set_layout(Some(Box::new(FlexLayout::new())));
      layer.add_sub_layer(sub_layer);
    }
    layer.set_layout(Some(Box::new(FlexLayout::new())));
    stage.add_sub_layer(layer);
  }

  stage.set_layout(Some(Box::new(FlexLayout::new())));
  play.add_stage(stage);

  //play.set_stage_needs_layout(&"stage".to_string());

  event_loop
    .run(move |event, elwt| {
      elwt.set_control_flow(ControlFlow::Poll);

      match event {
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::CloseRequested => elwt.exit(),
          WindowEvent::KeyboardInput {
            event:
              KeyEvent {
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                ..
              },
            ..
          } => elwt.exit(),
          WindowEvent::Resized(new_size) => {
            // Update wgpu surface and projection when window is resized
            play.resize(new_size.width, new_size.height);
          }
          WindowEvent::RedrawRequested => {
            play.render();
            window.request_redraw();
          }
          _ => {}
        },
        Event::AboutToWait => {
          window.request_redraw();
        }
        _ => {}
      }
    })
    .unwrap();
}
