# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Automated GitHub release process with workflow_dispatch trigger
- CHANGELOG.md for tracking release notes

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

[Unreleased]: https://github.com/joone/rust-animation/compare/v0.2.8...HEAD
[0.2.8]: https://github.com/joone/rust-animation/releases/tag/v0.2.8
