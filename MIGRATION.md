# Migration Guide: OpenGL to wgpu

## Overview

Version 0.3.0 migrates rust-animation from OpenGL to wgpu for improved cross-platform support and modern graphics API features.

## Breaking Changes

### API Changes

#### `RALayer::set_text()`
**Before (OpenGL):**
```rust
layer.set_text("Hello World");
```

**After (wgpu):**
```rust
// You now need to provide wgpu device and queue
let wgpu_context = WgpuContext::new_offscreen().await;
layer.set_text("Hello World", &wgpu_context.device, &wgpu_context.queue);
```

#### `RALayer::set_image()` 
The method signature remains the same for backward compatibility, but texture loading is now deferred:

**Before (OpenGL):**
```rust
layer.set_image("path/to/image.png".to_string());
// Texture loaded immediately
```

**After (wgpu):**
```rust
// Set the path
layer.set_image("path/to/image.png".to_string());

// Load texture when wgpu context is available
layer.load_image_texture(&device, &queue);
```

### Initialization Changes

#### `Play::new()`
The Play object no longer automatically initializes graphics resources. You need to explicitly initialize wgpu:

**Before (OpenGL):**
```rust
let play = Play::new(
    "My App".to_string(),
    800,
    600,
    LayoutMode::UserDefine,
);
// OpenGL was initialized automatically
```

**After (wgpu):**
```rust
let mut play = Play::new(
    "My App".to_string(),
    800,
    600,
    LayoutMode::UserDefine,
);
// Initialize wgpu context
play.init_wgpu();
```

### Rendering Changes

The `RALayer::render()` method signature has changed:

**Before (OpenGL):**
```rust
layer.render(shader_program, parent_transform, projection);
```

**After (wgpu):**
```rust
layer.render(parent_transform, projection);
// Actual rendering handled by Play with wgpu
```

## Benefits

The migration to wgpu provides:

- **Cross-platform support**: Vulkan, Metal, D3D12, OpenGL/ES, and WebGPU
- **Modern API**: Type-safe, memory-safe graphics programming
- **WebAssembly support**: Can now target web browsers
- **Better performance**: Access to modern GPU features
- **Future-proof**: Active development and strong ecosystem

## Examples

Examples are being updated to work with wgpu. For now, they may still reference OpenGL patterns. Check the latest examples in the repository for wgpu-based usage.

## Need Help?

If you encounter issues during migration, please:
1. Check the [examples](../examples/) directory for updated usage patterns
2. Review the [API documentation](https://docs.rs/rust-animation/)
3. [Open an issue](https://github.com/joone/rust-animation/issues) on GitHub
