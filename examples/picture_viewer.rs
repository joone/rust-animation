// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::Arc;
use stretch::node::Stretch;
use winit::{
  event::{ElementState, Event, KeyEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::{Window, WindowBuilder},
};

use reqwest::Error;
use serde_json::Value;
use std::boxed::Box;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use rust_animation::animation::Animation;
use rust_animation::animation::EasingFunction;
use rust_animation::layer::EventHandler;
use rust_animation::layer::Key as AnimKey;
use rust_animation::layer::Layout;
use rust_animation::layer::LayoutMode;
use rust_animation::layer::Layer;
use rust_animation::play::Play;

type ResultUrl<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn fetch_url(url: String, file_name: String) -> ResultUrl<()> {
  let response = reqwest::blocking::get(&url)?;
  println!("Downloading {}", url.to_string());
  if response.status().as_str() == "200" {
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
  }
  Ok(())
}

fn download_images() -> Result<(), Error> {
  fs::create_dir_all("examples/images").unwrap_or_else(|e| panic!("Error creating dir: {}", e));

  let request_url = format!("https://api.disneyapi.dev/character");
  let response = reqwest::blocking::get(&request_url)?;
  let text_json: String = response.text()?;
  let json_value: Value = serde_json::from_str(&text_json).unwrap();

  for i in 0..49 {
    let image_url = &json_value["data"][i]["imageUrl"];

    let file_name = format!("examples/images/{}.jpg", i);
    println!("{}", file_name);

    if Path::new(&file_name.to_string()).exists() {
      println!("Skip the downloaded file: {}", file_name.to_string());
    } else {
      match image_url.as_str() {
        Some(url) => {
          match fetch_url(url.to_string(), file_name.to_string()) {
            Ok(()) => {
              // Handle the success case
            }
            Err(e) => {
              // Handle the error case, perhaps log the error or retry
              eprintln!("Failed to fetch URL: {:?}", e);
            }
          }
        }
        None => {
          // Handle the case where image_url is None
          eprintln!("image_url was None");
        }
      }
    }
  }
  Ok(())
}

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = 225;

pub struct LayerEvent {
  name: String,
}

impl LayerEvent {
  pub fn new() -> Self {
    LayerEvent {
      name: "layer_event".to_string(),
    }
  }
}

impl EventHandler for LayerEvent {
  fn key_focus_in(&mut self, layer: &mut Layer) {
    println!("key_focus_in: {} {}", self.name, layer.name);
    let mut animation = Animation::new();
    animation.apply_scale(1.0, 1.1, 0.3, EasingFunction::EaseInOut);
    layer.set_animation(Some(animation));
  }

  fn key_focus_out(&mut self, layer: &mut Layer) {
    println!("key_focus_out: {} {}", self.name, layer.name);
    layer.scale_x = 1.0;
    layer.scale_y = 1.0;
  }

  fn key_down(&mut self, key: rust_animation::layer::Key, layer: &mut Layer) {
    println!("key_down: {}  {:?}  {}", self.name, key, layer.name);

    if key == rust_animation::layer::Key::Right {
      // right cursor
      layer.select_next_sub_layer();
    } else if key == rust_animation::layer::Key::Left {
      // left cursor
      layer.select_prev_sub_layer();
    }
  }
}

pub struct ActorLayout {
  name: String,
  cur_x: i32,
}

impl ActorLayout {
  pub fn new() -> Self {
    ActorLayout {
      name: "layer_layout".to_string(),
      cur_x: 0,
    }
  }
}

impl Layout for ActorLayout {
  fn layout_sub_layers(
    &mut self,
    layer: &mut Layer,
    _parent_layer: Option<&Layer>,
    _stretch: &mut Option<Stretch>,
  ) {
    println!("layout_sub_layer {}", self.name);
    let mut index: i32 = 0;
    for sub_layer in layer.sub_layer_list.iter_mut() {
      self.cur_x += sub_layer.width as i32;
      sub_layer.x = index % 5 * IMAGE_WIDTH as i32;
      let col = index / 5;
      sub_layer.y = col * IMAGE_HEIGHT as i32;
      index += 1;
    }
  }

  fn update_layout(&mut self, _actor: &mut Layer, _stretch: &mut Option<Stretch>) {
    println!("update_layout {}", self.name);
  }

  fn finalize(&mut self) {
    println!("finalize {}", self.name);
  }
}

pub struct PictureBrowser {
  play: Play,
  image_loaded: bool,
  file_list: Vec<String>,
  cur_file_index: usize,
  main_stage_name: String,
  splash_stage_name: String,
}

impl PictureBrowser {
  pub fn new(w: u32, h: u32) -> Self {
    PictureBrowser {
      image_loaded: false,
      play: Play::new(
        "Picture Browser".to_string(),
        w as i32,
        h as i32,
        LayoutMode::UserDefine,
      ),
      file_list: Vec::new(),
      cur_file_index: 0,
      main_stage_name: "".to_string(),
      splash_stage_name: "".to_string(),
    }
  }
  pub fn initialize(&mut self, window: Arc<Window>, width: u32, height: u32) {
    // Initialize wgpu context with surface
    self.play.init_wgpu_with_surface(window, width, height);

    let mut splash_stage = Layer::new("splash_stage".to_string(), width, height, None);
    splash_stage.set_image("examples/splash.png".to_string());
    // splash_stage.set_visible(true);
    // splash_stage.set_needs_layout();
    self.splash_stage_name = self.play.add_stage(splash_stage);

    let mut stage = Layer::new(
      "main_stage".to_string(),
      width,
      height,
      Some(Box::new(LayerEvent::new())),
    );
    stage.set_visible(false);
    stage.set_layout(Some(Box::new(ActorLayout::new())));
    //  stage.set_needs_layout();
    self.main_stage_name = self.play.add_stage(stage);
  }

  pub fn load_image_list(&mut self) {
    let paths = fs::read_dir("./examples/images").unwrap();

    for path in paths {
      let file_path = path.unwrap().path().display().to_string();
      println!("Loading {}", file_path.to_string());
      self.file_list.push(file_path);
    }
  }

  pub fn load_images(&mut self) {
    if self.file_list.len() == 0 {
      return;
    }

    if self.cur_file_index < self.file_list.len() {
      let name = format!("image_{}", self.cur_file_index);
      let mut layer = Layer::new(
        name.to_string(),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        Some(Box::new(LayerEvent::new())),
      );
      layer.set_image(self.file_list[self.cur_file_index].to_string());
      self
        .play
        .add_new_layer_to_stage(&self.main_stage_name, layer);
      println!(
        "load a texture {}",
        &self.file_list[self.cur_file_index].to_string()
      );
      self.cur_file_index += 1;
    } else {
      self.image_loaded = true;
      println!("load all textures");
    }
  }

  pub fn render(&mut self) {
    self.play.render();
  }

  pub fn handle_input(&mut self, key: AnimKey) {
    self.play.handle_input(key);
  }

  pub fn render_splash_screen(&mut self) {
    if self.image_loaded == true {
      self.play.set_visible_stage(&self.splash_stage_name, false);
      self.play.set_visible_stage(&self.main_stage_name, true);
      return;
    }
    self.load_images();
  }
}

fn main() {
  let event_loop = EventLoop::new().unwrap();
  let window = Arc::new(
    WindowBuilder::new()
      .with_title("Image Viewer")
      .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
      .build(&event_loop)
      .unwrap(),
  );

  // Get the actual window size (may differ from requested due to DPI scaling)
  let window_size = window.inner_size();
  let (width, height) = (window_size.width, window_size.height);

  let (tx, rx) = mpsc::channel();
  thread::spawn(move || match download_images() {
    Ok(()) => {
      tx.send(true).unwrap();
    }
    Err(..) => {}
  });

  let mut picture_browser = PictureBrowser::new(width, height);
  picture_browser.initialize(window.clone(), width, height);

  event_loop
    .run(move |event, elwt| {
      elwt.set_control_flow(ControlFlow::Poll);

      match event {
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::CloseRequested => elwt.exit(),
          WindowEvent::KeyboardInput {
            event:
              KeyEvent {
                physical_key: PhysicalKey::Code(key_code),
                state: ElementState::Pressed,
                ..
              },
            ..
          } => match key_code {
            KeyCode::Escape => elwt.exit(),
            KeyCode::ArrowUp => picture_browser.handle_input(AnimKey::Up),
            KeyCode::ArrowDown => picture_browser.handle_input(AnimKey::Down),
            KeyCode::ArrowLeft => picture_browser.handle_input(AnimKey::Left),
            KeyCode::ArrowRight => picture_browser.handle_input(AnimKey::Right),
            KeyCode::Enter => picture_browser.handle_input(AnimKey::Enter),
            KeyCode::Space => picture_browser.handle_input(AnimKey::Space),
            _ => {}
          },
          WindowEvent::Resized(new_size) => {
            // Update wgpu surface and projection when window is resized
            picture_browser.play.resize(new_size.width, new_size.height);
          }
          WindowEvent::RedrawRequested => {
            picture_browser.render_splash_screen();

            match rx.try_recv() {
              Ok(true) => {
                picture_browser.load_image_list();
              }
              Ok(false) => {}
              Err(..) => {}
            }

            picture_browser.render();
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
