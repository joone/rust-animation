// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

use glfw::{Action, Context, Key};

use std::thread;
use reqwest::Error;
use serde_json::Value;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::mpsc;
use std::boxed::Box;

use rust_animation::play::Play;
use rust_animation::stage::Stage;
use rust_animation::actor::Actor;
use rust_animation::actor::EventHandler;
use rust_animation::actor::Layout;
use rust_animation::actor::EasingFunction;
use rust_animation::actor::LayoutMode;

type ResultUrl<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn fetch_url(url: String, file_name: String) -> ResultUrl<()> {
  let response = reqwest::blocking::get(&url)?;
  println!("Downloading {}", url.to_string());
  if response.status().as_str() == "200" {
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
  }
  Ok(())
}

fn download_images() -> Result<(), Error> {
  fs::create_dir_all("examples/images").unwrap_or_else(|e| panic!("Error creating dir: {}", e));

  let request_url = format!("https://api.disneyapi.dev/characters");
  let response = reqwest::blocking::get(&request_url)?;
  let text_json :String = response.text()?;
  let json_value  : Value = serde_json::from_str(&text_json).unwrap();

  for i in 0..49 {
      let image_url = &json_value["data"][i]["imageUrl"];
     
      let file_name = format!("examples/images/{}.jpg", i);
      println!("{}", file_name);

      if Path::new(&file_name.to_string()).exists() {
        println!("Skip the downloaded file: {}", file_name.to_string());
      } else {
        match fetch_url(image_url.as_str().unwrap().to_string(), file_name.to_string()) {
          Ok(()) => {}
          Err(..) => {}
        }
      }
  }
  Ok(())
}

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = 225;

pub struct ActorEvent {
  name: String,
}

impl ActorEvent {
 pub fn new() -> Self {
    ActorEvent {
      name: "actor_event".to_string()
    }
 }
}

impl EventHandler for ActorEvent {
  fn key_focus_in(&mut self, actor: &mut Actor) {
     println!("key_focus_in: {} {}", self.name, actor.name);
     actor.apply_scale_animation(1.0, 1.1, 0.3, EasingFunction::EaseInOut);
  }

  fn key_focus_out(&mut self, actor: &mut Actor) {
    println!("key_focus_out: {} {}", self.name, actor.name);
    actor.scale_x = 1.0;
    actor.scale_y = 1.0;
  }

  fn key_down(&mut self, key: usize, actor: &mut Actor) {
     println!("key_down: {}  {}  {}", self.name, key, actor.name);

    if key == 262 {     // right cursor
       actor.select_next_sub_actor();
    } else if key == 263 { // left cursor 
       actor.select_prev_sub_actor();
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
      name: "actor_layout".to_string(),
      cur_x: 0
    }
 }
}

impl Layout for ActorLayout {
  fn layout_sub_actors(&mut self, sub_actor_list: &mut Vec<Actor>) {
    println!("layout_sub_layer {}", self.name);
    let mut index : i32 = 0;
    for sub_actor in sub_actor_list.iter_mut() {
      self.cur_x += sub_actor.width as i32;
      sub_actor.x = index % 5 * IMAGE_WIDTH as i32;
      let col = index  / 5;
      sub_actor.y =  col * IMAGE_HEIGHT as i32;
      index +=1;
    }
  }
}

pub struct PictureBrowser<'a> {
  play: Play<'a>,
  image_loaded: bool,
  file_list: Vec<String>,
  cur_file_index: usize,
  main_stage_name: String,
  splash_stage_name: String,
}

impl<'a> PictureBrowser<'a> {
  pub fn new(w: u32, h: u32) -> Self {
    PictureBrowser {
      image_loaded: false,
      play: Play::new("Picture Browser".to_string()),
      file_list: Vec::new(),
      cur_file_index: 0,
      main_stage_name: "".to_string(),
      splash_stage_name: "".to_string()
    }
  }
  pub fn initialize(&mut self) {
    self.play.initialize();
    let mut splash_stage = Stage::new("splash_stage".to_string(), 1920, 1080,
        LayoutMode::UserDefine, None);
    splash_stage.set_image("examples/splash.png".to_string());
    splash_stage.set_visible(true);
    splash_stage.set_needs_layout();
    self.splash_stage_name = self.play.add_stage(splash_stage);

    let mut stage = Stage::new("main_stage".to_string(), 1920, 1080,
        LayoutMode::UserDefine, Some(Box::new(ActorEvent::new())));
    stage.set_layout(Some(Box::new(ActorLayout::new())));
    stage.set_needs_layout();
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
        let mut actor = Actor::new(name.to_string(), IMAGE_WIDTH, IMAGE_HEIGHT,
            Some(Box::new(ActorEvent::new())));
        actor.set_image(self.file_list[self.cur_file_index].to_string());
        self.play.add_new_actor_to_stage(&self.main_stage_name, actor);
        println!("load a texture {}", &self.file_list[self.cur_file_index].to_string());
        self.cur_file_index += 1;
    } else {
      self.image_loaded = true;
      self.play.set_stage_needs_layout(&self.main_stage_name);
    }
  }

  pub fn render(&mut self) {
    self.play.render();
  }

  pub fn handle_input(&mut self, key: glfw::Key) {
    self.play.handle_input(key as usize);
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
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  let (mut window, events) = glfw.create_window(1920, 1080,
      "Image Viewer", glfw::WindowMode::Windowed)
      .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    match download_images() {
      Ok(()) => {
        tx.send(true).unwrap();
      }
      Err(..) => {}
    }
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

fn handle_window_event(window: &mut glfw::Window, (_time, event): (f64, glfw::WindowEvent),
    browser: &mut PictureBrowser) {
  match event {
    glfw::WindowEvent::FramebufferSize(w, h) => {
        unsafe { gl::Viewport(0, 0, w, h) }
    }
    glfw::WindowEvent::Key(key, _scancode, action, _mods) => {
      /*println!(
          "Time: {:?}, Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]",
          time, key, scancode, action, mods
      );*/
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
