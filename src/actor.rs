// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub struct Actor {
  pub width: u32,
  pub height: u32,
}

impl Actor {
  pub fn new(w: u32, h: u32) -> Self {
    Actor {
        width: w,
        height: h,
    }
  }
}