// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use wgpu;

/// Shared wgpu rendering context
pub struct WgpuContext {
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
}

impl WgpuContext {
  /// Create a new wgpu context without a surface (for library use)
  pub async fn new_offscreen() -> Self {
    let instance = wgpu::Instance::default();
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
      })
      .await
      .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: Some("Device"),
          required_features: wgpu::Features::empty(),
          required_limits: wgpu::Limits::default(),
          memory_hints: wgpu::MemoryHints::default(),
        },
        None,
      )
      .await
      .expect("Failed to create device");

    WgpuContext { device, queue }
  }
}
