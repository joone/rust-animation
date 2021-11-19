// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use rust_animation::play::render;
use rust_animation::play::Play;

fn main() {
   render("rectangle".to_string());

   let play = Play::new(1920, 1080);

   play.render();
}


