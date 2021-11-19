// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub fn render(name:String) {
   println!("Render {}", name);
}

pub struct Play {
  name : String,
}

impl Play {
  pub fn new(w: u32, h: u32) -> Self {
    Play {
      name : "Test".to_string(),
    }
  }

  pub fn render(&self) {
    println!("{}", self.name);
  }
}
