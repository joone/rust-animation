# Release Process

This document describes how to publish a new version of `rust-animation` to crates.io and create a GitHub release.

## Fully Automated Release Process

The project uses GitHub Actions to fully automate releases, including:
- Creating GitHub Releases with auto-generated release notes
- Publishing to crates.io
- Building and attaching example binaries for multiple platforms
- Version validation and testing

### Prerequisites

1. **Set up CRATES_IO_TOKEN secret**: The repository must have a `CRATES_IO_TOKEN` secret configured.
   - Go to https://crates.io/me/tokens and create a new API token
   - Add the token as a repository secret:
     - Go to repository Settings → Secrets and variables → Actions
     - Click "New repository secret"
     - Name: `CRATES_IO_TOKEN`
     - Value: Your crates.io API token

## Release Methods

### Method 1: Automated via Version Bump Script (Recommended)

Use the included `bump-version.sh` script to automate version management:

#### Option A: Fully Automatic (One Command)

Run the version bump script with the `--auto` flag to handle everything automatically:

```bash
# Bump patch version (0.2.8 -> 0.2.9) and trigger release
./bump-version.sh patch --auto

# Bump minor version (0.2.8 -> 0.3.0) and trigger release
./bump-version.sh minor --auto

# Bump major version (0.2.8 -> 1.0.0) and trigger release
./bump-version.sh major --auto

# Set specific version and trigger release
./bump-version.sh 0.3.5 --auto
```

The script will:
1. Update version in `Cargo.toml`
2. Create a new version section in `CHANGELOG.md`
3. Commit the changes with message "Bump version to X.Y.Z"
4. Push to the main branch
5. Create and push a git tag `vX.Y.Z`
6. Trigger the automated release workflow

**Note:** You should still update the `[Unreleased]` section in CHANGELOG.md with your changes before running the script, or manually update the generated version section after the script runs but before it commits.

#### Option B: Semi-Automatic (Review Before Release)

1. **Run the version bump script** (without --auto):
   ```bash
   # Bump patch version (0.2.8 -> 0.2.9)
   ./bump-version.sh patch
   
   # Bump minor version (0.2.8 -> 0.3.0)
   ./bump-version.sh minor
   
   # Bump major version (0.2.8 -> 1.0.0)
   ./bump-version.sh major
   
   # Set specific version
   ./bump-version.sh 0.3.5
   ```

2. **Update CHANGELOG.md**:
   - The script will create a new version section in CHANGELOG.md
   - Fill in the actual changes under the new version section:
     ```markdown
     ## [0.2.9] - 2024-01-15
     
     ### Added
     - New feature X
     
     ### Fixed
     - Bug Y
     ```

3. **Commit and push the version bump**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Bump version to 0.2.9"
   git push origin main
   ```

4. **Create and push a git tag**:
   ```bash
   git tag v0.2.9
   git push origin v0.2.9
   ```
   
   The release workflow will automatically:
   - Create a GitHub Release with release notes from CHANGELOG.md
   - Publish to crates.io
   - Build example binaries for Linux, macOS, and Windows
   - Attach binaries to the release

5. **Monitor the workflow**:
   - Go to the Actions tab in GitHub
   - Watch the "Create Release" workflow
   - Check the Releases page once complete

**Alternative:** After steps 1-3, you can use the `--auto` flag to automatically create and push the tag:
   ```bash
   # Since Cargo.toml is already updated, just create and push the tag
   git tag v0.2.9
   git push origin v0.2.9
   ```

### Method 2: Manual Trigger via GitHub UI (With Button!)

You can trigger a release directly from GitHub's UI without creating a tag locally:

1. **Update the version** in `Cargo.toml` and `CHANGELOG.md`:
   ```toml
   [package]
   version = "0.2.9"  # Update this
   ```
   
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Bump version to 0.2.9"
   git push origin main
   ```

2. **Trigger the workflow from GitHub**:
   - Go to the repository on GitHub
   - Click on the "Actions" tab
   - Select "Create Release" workflow from the left sidebar
   - Click "Run workflow" button (⚡ button on the right)
   - Fill in the form:
     - **version**: Enter the version number (e.g., `0.2.9`)
     - **create_tag**: Check this to automatically create and push the tag
   - Click "Run workflow"

3. **Monitor the workflow**:
   - The workflow will automatically create the tag, run tests, create the GitHub Release, and publish to crates.io
   - Watch the workflow progress in the Actions tab

### Method 3: Traditional Tag-Based Release

The traditional approach still works:

1. **Update version** in `Cargo.toml` and `CHANGELOG.md`
2. **Commit and push**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Bump version to 0.2.9"
   git push origin main
   ```
3. **Create and push tag**:
   ```bash
   git tag v0.2.9
   git push origin v0.2.9
   ```
4. **Workflow runs automatically** when tag is pushed

## What Happens During a Release

The automated workflow performs these steps:

1. **Validation**:
   - Verifies `Cargo.toml` version matches the tag/input version
   - Fails fast if there's a mismatch

2. **Testing**:
   - Runs the full test suite (`cargo test --lib`)
   - Only proceeds if all tests pass

3. **Building**:
   - Builds the release version
   - Builds example binaries for Linux, macOS, and Windows

4. **GitHub Release**:
   - Extracts release notes from CHANGELOG.md
   - Creates a GitHub Release with the notes
   - Attaches example binaries as release assets

5. **crates.io Publishing**:
   - Publishes the crate to crates.io using the `CRATES_IO_TOKEN`

6. **Verification**:
   - Check https://crates.io/crates/rust-animation
   - Check https://github.com/joone/rust-animation/releases
   - Verify the new version appears in both places

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions
- **PATCH** version for backwards-compatible bug fixes

Example: `0.2.8` → `0.2.9` (patch), `0.3.0` (minor), `1.0.0` (major)

## Troubleshooting

### Version Mismatch Error

If you get a version mismatch error, make sure:
- The version in `Cargo.toml` matches the git tag (without the 'v' prefix)
- Example: `Cargo.toml` has `version = "0.2.8"` and tag is `v0.2.8`

To fix:
1. Update `Cargo.toml` to match the desired version
2. Commit and push the change
3. Delete the incorrect tag: `git tag -d v0.2.8 && git push origin :refs/tags/v0.2.8`
4. Recreate the tag with the correct version

### Publishing Fails

If publishing fails:
- Check that `CRATES_IO_TOKEN` is set correctly in repository secrets
- Verify the token has publish permissions
- Check the workflow logs for specific error messages
- Ensure all tests pass: `cargo test --lib`
- Make sure you haven't already published this version

### GitHub Release Creation Fails

If GitHub Release creation fails but crates.io publish succeeds:
- The package is already published to crates.io (cannot be unpublished)
- You can manually create a GitHub Release:
  1. Go to https://github.com/joone/rust-animation/releases/new
  2. Select the tag
  3. Copy release notes from CHANGELOG.md
  4. Publish the release

### Manual Publishing (Fallback)

If automated publishing fails completely, you can publish manually:

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

Then manually create a GitHub Release as described above.

## Best Practices

1. **Always update CHANGELOG.md** before releasing
2. **Test locally** before pushing tags: `cargo test --lib && cargo build --release`
3. **Use the version bump script** to avoid manual errors
4. **Never force-push tags** - if you need to fix a tag, delete and recreate it
5. **Check the Actions tab** after pushing to ensure the workflow succeeds
6. **Verify both releases** - check both crates.io and GitHub Releases after publishing
