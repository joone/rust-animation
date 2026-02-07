# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.10] - 2026-02-07

### Added
- Initial public release of rust-animation
- wgpu-based rendering engine (22.1.0) for cross-platform hardware acceleration
- Core animation system with keyframe support and multiple easing functions
- Layer hierarchy system with parent-child relationships
- Flex layout system using Stretch library for CSS Flexbox-like layouts
- Text rendering using ab_glyph with TrueType font support
- Image loading and texture rendering with format auto-detection
- CoreAnimation-style API for iOS/macOS developer familiarity
- Multiple examples: ani, coreanimation_api, easing_functions, flex_ui, font_test, picture_viewer
- Comprehensive documentation (README, MIGRATION guide, RELEASING guide)
- Automated GitHub workflows for CI, releases, and crates.io publishing
- Version management script (bump-version.sh) for automated releases

## [0.2.8] - 2024-XX-XX

### Changed
- Migrated from OpenGL to wgpu 22.1.0 for rendering
- Updated all examples to use winit 0.29.15 with wgpu
- Breaking API changes: set_text now requires wgpu device and queue parameters

### Added
- CoreAnimation-style API for better iOS/macOS developer familiarity
- WGSL shaders for wgpu backend
- Comprehensive migration guide in MIGRATION.md

### Fixed
- Improved macOS compatibility with explicit wgpu backend configuration
- Removed all unsafe code blocks for safer API

### Removed
- OpenGL/GLSL rendering backend
- glfw dependency in favor of winit

[Unreleased]: https://github.com/joone/rust-animation/compare/v0.2.10...HEAD
[0.2.10]: https://github.com/joone/rust-animation/releases/tag/v0.2.10
[0.2.8]: https://github.com/joone/rust-animation/releases/tag/v0.2.8
