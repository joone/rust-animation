// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

use glfw::{Action, Context, Key};
use stretch::node::Stretch;

use reqwest::Error;
use serde_json::Value;
use std::boxed::Box;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use rust_animation::layer::RALayer;
use rust_animation::layer::EventHandler;
use rust_animation::layer::Layout;
use rust_animation::layer::LayoutMode;
use rust_animation::animation::Animation;
use rust_animation::animation::EasingFunction;
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

pub struct RALayerEvent {
  name: String,
}

impl RALayerEvent {
  pub fn new() -> Self {
    RALayerEvent {
      name: "layer_event".to_string(),
    }
  }
}

impl EventHandler for RALayerEvent {
  fn key_focus_in(&mut self, layer: &mut RALayer) {
    println!("key_focus_in: {} {}", self.name, layer.name);
    let mut animation = Animation::new();
    animation.apply_scale(1.0, 1.1, 0.3, EasingFunction::EaseInOut);
    layer.set_animation(Some(animation));
  }

  fn key_focus_out(&mut self, layer: &mut RALayer) {
    println!("key_focus_out: {} {}", self.name, layer.name);
    layer.scale_x = 1.0;
    layer.scale_y = 1.0;
  }

  fn key_down(&mut self, key: rust_animation::layer::Key, layer: &mut RALayer) {
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
    layer: &mut RALayer,
    _parent_layer: Option<&RALayer>,
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

  fn update_layout(&mut self, _actor: &mut RALayer, _stretch: &mut Option<Stretch>) {
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
  pub fn new(_w: u32, _h: u32) -> Self {
    PictureBrowser {
      image_loaded: false,
      play: Play::new(
        "Picture Browser".to_string(),
        1920,
        1080,
        LayoutMode::UserDefine,
      ),
      file_list: Vec::new(),
      cur_file_index: 0,
      main_stage_name: "".to_string(),
      splash_stage_name: "".to_string(),
    }
  }
  pub fn initialize(&mut self) {
    let mut splash_stage = RALayer::new("splash_stage".to_string(), 1920, 1080, None);
    splash_stage.set_image("examples/splash.png".to_string());
    // splash_stage.set_visible(true);
    // splash_stage.set_needs_layout();
    self.splash_stage_name = self.play.add_stage(splash_stage);

    let mut stage = RALayer::new(
      "main_stage".to_string(),
      1920,
      1080,
      Some(Box::new(RALayerEvent::new())),
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
      let mut layer = RALayer::new(
        name.to_string(),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        Some(Box::new(RALayerEvent::new())),
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

  pub fn handle_input(&mut self, key: glfw::Key) {
    self
      .play
      .handle_input(unsafe { ::std::mem::transmute(key) });
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
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    glfw::OpenGlProfileHint::Core,
  ));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  let (mut window, events) = glfw
    .create_window(1920, 1080, "Image Viewer", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let (tx, rx) = mpsc::channel();
  thread::spawn(move || match download_images() {
    Ok(()) => {
      tx.send(true).unwrap();
    }
    Err(..) => {}
  });

  let mut picture_browser = PictureBrowser::new(1920, 1080);
  picture_browser.initialize();

  while !window.should_close() {
    for event in glfw::flush_messages(&events) {
      handle_window_event(&mut window, event, &mut picture_browser);
    }

    picture_browser.render_splash_screen();

    match rx.try_recv() {
      Ok(true) => {
        picture_browser.load_image_list();
      }
      Ok(false) => {}
      Err(..) => {}
    }

    picture_browser.render();

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    window.swap_buffers();
    glfw.poll_events();
  }
}

fn handle_window_event(
  window: &mut glfw::Window,
  (_time, event): (f64, glfw::WindowEvent),
  browser: &mut PictureBrowser,
) {
  match event {
    glfw::WindowEvent::FramebufferSize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
    glfw::WindowEvent::Key(key, scancode, action, mods) => {
      println!(
        "Time: {:?}, Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]",
        _time, key, scancode, action, mods
      );
      match (key, action) {
        (Key::Escape, Action::Press) => window.set_should_close(true),
        (Key::Up, Action::Press) => browser.handle_input(Key::Up),
        (Key::Down, Action::Press) => browser.handle_input(Key::Down),
        (Key::Left, Action::Press) => browser.handle_input(Key::Left),
        (Key::Right, Action::Press) => browser.handle_input(Key::Right),
        (Key::Enter, Action::Press) => browser.handle_input(Key::Enter),
        (Key::Space, Action::Press) => browser.handle_input(Key::Space),
        _ => {}
      }
    }
    _ => {}
  }
}
