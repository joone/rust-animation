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

use std::sync::mpsc::Receiver;
use rust_animation::play::Play;
use rust_animation::stage::Stage;
use rust_animation::actor::Actor;

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

pub struct PictureBrowser {
  play: Play,
  image_loaded: bool,
  file_list: Vec<String>,
  cur_file_index: usize,
}

impl PictureBrowser {
  pub fn new(w: u32, h: u32) -> Self {
    PictureBrowser {
      image_loaded: false,
      play: Play::new("Picture Browser".to_string()),
      file_list: Vec::new(),
      cur_file_index: 0
    }
  }
  pub fn initialize(&mut self) {
    self.play.initialize();
    let stage = Stage::new(1920, 1080);
    self.play.add_stage(stage);
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
        let mut actor = Actor::new("image".to_string(), 400, 300);
        actor.x = self.cur_file_index as i32 % 5 * 400;
        let col = self.cur_file_index as i32 / 5;
        actor.y =  col * 320;
        actor.set_image(self.file_list[self.cur_file_index].to_string());
        self.play.stage_list[0].add_actor(actor);
        println!("load a texture {}", &self.file_list[self.cur_file_index].to_string());
        self.cur_file_index += 1;
    } else {
      self.image_loaded = true;
    }
  }

  pub fn render(&mut self) {
    if self.image_loaded == false {
      return;
    }

    self.play.render();
  }

  pub fn handle_input(&mut self, key: glfw::Key) {

  }

  pub fn render_splash_screen(&mut self) {
    if self.image_loaded == true {
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
    // events
    process_events(&mut window, &events);

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

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
  for (_, event) in glfw::flush_messages(events) {
    match event {
      glfw::WindowEvent::FramebufferSize(width, height) => {
        // make sure the viewport matches the new window dimensions; note that width and
        // height will be significantly larger than specified on retina displays.
        unsafe { gl::Viewport(0, 0, width, height) }
      }
      glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
      _ => {}
    }
  }
}
