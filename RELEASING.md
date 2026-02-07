# Release Process

This document describes how to publish a new version of `rust-animation` to crates.io.

## Automated Release Process

The project uses GitHub Actions to automate the publishing process. When you push a git tag, the workflow automatically publishes the crate to crates.io.

### Prerequisites

1. **Set up CRATES_IO_TOKEN secret**: The repository must have a `CRATES_IO_TOKEN` secret configured.
   - Go to https://crates.io/me/tokens and create a new API token
   - Add the token as a repository secret:
     - Go to repository Settings → Secrets and variables → Actions
     - Click "New repository secret"
     - Name: `CRATES_IO_TOKEN`
     - Value: Your crates.io API token

### Steps to Release

1. **Update the version number** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.8"  # Update this
   ```

2. **Update CHANGELOG** (if you have one):
   - Document all changes since the last release
   - Include breaking changes, new features, and bug fixes

3. **Commit the version change**:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.8"
   git push origin main
   ```

4. **Create and push a git tag**:
   ```bash
   git tag v0.2.8
   git push origin v0.2.8
   ```

5. **Monitor the workflow**:
   - Go to the Actions tab in GitHub
   - Watch the "Publish to crates.io" workflow
   - The workflow will:
     - Verify the version in Cargo.toml matches the tag
     - Run tests
     - Build the project
     - Publish to crates.io

6. **Verify the release**:
   - Check https://crates.io/crates/rust-animation
   - Verify the new version appears

## Troubleshooting

### Version Mismatch Error

If you get a version mismatch error, make sure:
- The version in `Cargo.toml` matches the git tag (without the 'v' prefix)
- Example: `Cargo.toml` has `version = "0.2.8"` and tag is `v0.2.8`

### Publishing Fails

If publishing fails:
- Check that `CRATES_IO_TOKEN` is set correctly in repository secrets
- Verify the token has publish permissions
- Check the workflow logs for specific error messages
- Ensure all tests pass: `cargo test --lib`

### Manual Publishing (Fallback)

If automated publishing fails, you can publish manually:

```bash
# Make sure you're on the tagged commit
git checkout v0.2.8

# Run tests
cargo test --lib

# Build
cargo build --release

# Publish (you'll need to log in with your crates.io token)
cargo publish
```

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):
- MAJOR version for incompatible API changes
- MINOR version for backwards-compatible functionality additions
- PATCH version for backwards-compatible bug fixes

Example: `0.2.8` → `0.2.9` (patch), `0.3.0` (minor), `1.0.0` (major)
