// Copyright (c) 2021 Joone Hur <joone@chromium.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use wgpu;

/// Shared wgpu rendering context
pub struct WgpuContext {
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub surface: Option<wgpu::Surface<'static>>,
  pub surface_config: Option<wgpu::SurfaceConfiguration>,
}

impl WgpuContext {
  /// Create a new wgpu context without a surface (for library use)
  pub async fn new_offscreen() -> Self {
    // Explicitly specify backends for better compatibility across platforms
    // On macOS, this ensures Metal backend is used
    // On other platforms, PRIMARY includes Vulkan, DX12, or other native backends
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });
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

    WgpuContext {
      device,
      queue,
      surface: None,
      surface_config: None,
    }
  }

  /// Create a new wgpu context with a surface for rendering to a window
  pub async fn new_with_surface(
    window: impl Into<wgpu::SurfaceTarget<'static>>,
    width: u32,
    height: u32,
  ) -> Self {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });

    let surface = instance
      .create_surface(window)
      .expect("Failed to create surface");

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
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

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
      .formats
      .iter()
      .find(|f| f.is_srgb())
      .copied()
      .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width,
      height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    WgpuContext {
      device,
      queue,
      surface: Some(surface),
      surface_config: Some(config),
    }
  }

  /// Resize the surface
  pub fn resize(&mut self, width: u32, height: u32) {
    if let (Some(surface), Some(config)) = (&self.surface, &mut self.surface_config) {
      config.width = width;
      config.height = height;
      surface.configure(&self.device, config);
    }
  }
}
