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

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = 225;

pub struct PictureBrowser {
  play: Play,
  image_loaded: bool,
  file_list: Vec<String>,
  cur_file_index: usize,
  main_stage: usize,
  splash_stage: usize,
  cur_col: u32,
  cur_row: u32,
  num_of_col: u32,
  num_of_row: u32,
  sel_actor_index: usize
}

impl PictureBrowser {
  pub fn new(w: u32, h: u32) -> Self {
    PictureBrowser {
      image_loaded: false,
      play: Play::new("Picture Browser".to_string()),
      file_list: Vec::new(),
      cur_file_index: 0,
      main_stage: 0,
      splash_stage: 0,
      cur_col: 0,
      cur_row: 0,
      num_of_col: 0,
      num_of_row: 0,
      sel_actor_index: 0,
    }
  }
  pub fn initialize(&mut self) {
    self.play.initialize();
    let mut splash_stage = Stage::new(1920, 1080);
    splash_stage.set_visible(true);
    splash_stage.stage_actor.set_image("examples/splash.png".to_string());
    self.splash_stage =self.play.add_stage(splash_stage);

    let stage = Stage::new(1920, 1080);
    self.main_stage = self.play.add_stage(stage);
  }

  pub fn load_image_list(&mut self) {
    let paths = fs::read_dir("./examples/images").unwrap();

    for path in paths {
      let file_path = path.unwrap().path().display().to_string();
      println!("Loading {}", file_path.to_string());
      self.file_list.push(file_path);
    }
  }

  pub fn scale_up_first_selected_actor(&mut self, col :u32, row: u32) {
    // Set the initial selected actor.
    self.cur_col = col;
    self.cur_row = row;

    // Count the number of rows and columns using the number of video images.
    self.num_of_col = (self.play.stage_list[self.main_stage].stage_actor.width as f32  / IMAGE_WIDTH as f32) as u32 + 1;
    // Scale up the first selected actor.
    self.apply_selection_animation();
  }

  pub fn load_images(&mut self) {
    if self.file_list.len() == 0 {
      return;
    }

    if self.cur_file_index < self.file_list.len() {
        let mut actor = Actor::new("image".to_string(), IMAGE_WIDTH, IMAGE_HEIGHT);
        actor.x = self.cur_file_index as i32 % 5 * IMAGE_WIDTH as i32;
        let col = self.cur_file_index as i32 / 5;
        actor.y =  col * IMAGE_HEIGHT as i32;
        actor.set_image(self.file_list[self.cur_file_index].to_string());
        self.play.stage_list[self.main_stage].add_actor(actor);
        println!("load a texture {}", &self.file_list[self.cur_file_index].to_string());
        self.cur_file_index += 1;
    } else {
      self.image_loaded = true;
    }

    if self.cur_file_index == self.file_list.len() {
      // Select the movie in column 2 and row 1.
       self.scale_up_first_selected_actor(2, 1);
    }
  }

  pub fn render(&mut self) {
    /*if self.image_loaded == false {
      return;
    }*/

    self.play.render();
  }

  pub fn handle_input(&mut self, key: glfw::Key) {
    // Do not handle events during the animation.
    if  self.play.stage_list[self.main_stage].stage_actor.animated == true {
       return;
    }

    let image_length = self.play.stage_list[self.main_stage].stage_actor.sub_actor_list.len();
 
      // Count the number of rows and columns using the number of video images.
    self.num_of_col = (self.play.stage_list[self.main_stage].stage_actor.width as f32 / IMAGE_WIDTH as f32) as u32 + 1;
    self.num_of_row = image_length as u32 / self.num_of_col;

    let cur_row = self.cur_row;
    let cur_col = self.cur_col;
    let pre_sel_actor = self.sel_actor_index;

    if key == Key::Up {
      if self.cur_row < self.num_of_row {
        self.cur_row += 1;
        self.apply_selection_animation();
      }
    } else if key == Key::Down {
      if self.cur_row > 0 {
        self.cur_row -= 1;
        self.apply_selection_animation();
      }
    } else if key == Key::Left {
      if self.cur_col > 0 {
        self.cur_col -= 1;
        self.apply_selection_animation();
      }
    } else if key == Key::Right {
      if self.cur_col < self.num_of_col - 1 {
        self.cur_col += 1;
        self.apply_selection_animation();
      }
    } /*else if key == Key::Enter || key == Key::Space {
      self.apply_genie_effect_to_view_actor();
    }*/

    if cur_row != self.cur_row || cur_col != self.cur_col {
    
      // Scroll the main_stage
      let stage_y = self.play.stage_list[self.main_stage].stage_actor.y;
      if key == Key::Down {
         self.play.stage_list[self.main_stage].stage_actor.apply_translation_y_animation(stage_y, 
             stage_y + IMAGE_HEIGHT as i32, 9);
      } else if key == Key::Up {
        self.play.stage_list[self.main_stage].stage_actor.apply_translation_y_animation(stage_y,
            stage_y - IMAGE_HEIGHT as i32, 9);
      }
      println!("scroll y={}", stage_y);
    }

    if pre_sel_actor != self.sel_actor_index {
      self.play.stage_list[self.main_stage].stage_actor.sub_actor_list[pre_sel_actor].scale_x = 1.0;
         self.play.stage_list[self.main_stage].stage_actor.sub_actor_list[pre_sel_actor].scale_y = 1.0;
    }
  }

  fn apply_selection_animation(&mut self) {
     let mut i = (self.num_of_col * self.cur_row + self.cur_col) as usize;
     self.sel_actor_index = i;
     // if the i is bigger than the number of actor_list, select the last index and
     // update self.cur_col and self.cur_row
     let image_length = self.play.stage_list[self.main_stage].stage_actor.sub_actor_list.len();
     if i >= image_length {
        i = image_length - 1;
        let row = image_length as u32 / self.num_of_col;
        let col = image_length as u32 % self.num_of_col;
        self.cur_row = row ;
        self.cur_col = col - 1;
      }
      self.play.stage_list[self.main_stage].stage_actor.sub_actor_list[i].apply_scale_animation(1.0, 1.15, 0.01);
      //println!("sel actor =  {} {}", self.actor_list[i].x, self.actor_list[i].y);
  }

  pub fn render_splash_screen(&mut self) {
    if self.image_loaded == true {
      self.play.stage_list[self.splash_stage].set_visible(false);
      self.play.stage_list[self.main_stage].set_visible(true);
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
