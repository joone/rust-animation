# Automated Release Process - Quick Start Guide

This repository now has a **fully automated GitHub release process** with multiple options for triggering releases.

## 🚀 Three Ways to Release

### Option 1: Push Button Release (Recommended for Quick Releases)

**Perfect when:** You've already committed version changes and want to release with one click.

1. Update `Cargo.toml` version and `CHANGELOG.md`
2. Commit and push to main
3. Go to **Actions** → **Create Release** → **Run workflow** button
4. Enter version (e.g., `0.2.9`) and click "Run workflow"
5. Done! 🎉

The workflow will automatically:
- Create the git tag
- Run tests
- Create GitHub Release with changelog
- Publish to crates.io
- Build example binaries for Linux/macOS/Windows

### Option 2: Use the Version Bump Script (Recommended for Developers)

**Perfect when:** You want automation to help with version management.

```bash
# Bump version (patch: 0.2.8 → 0.2.9)
./bump-version.sh patch

# Update CHANGELOG.md with your changes
# (The script creates a new section for you)

# Commit and push
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.9"
git push origin main

# Create and push tag
git tag v0.2.9
git push origin v0.2.9
```

### Option 3: Traditional Tag-Based (Classic Approach)

**Perfect when:** You prefer the traditional git workflow.

```bash
# Update Cargo.toml and CHANGELOG.md manually
# Commit and push
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.9"
git push origin main

# Create and push tag
git tag v0.2.9
git push origin v0.2.9
```

## 📋 What You Get

Every release automatically includes:

✅ **GitHub Release** with formatted release notes from CHANGELOG.md  
✅ **crates.io Publication** for Rust users  
✅ **Example Binaries** for Linux, macOS, and Windows  
✅ **Automated Testing** before release  
✅ **Version Validation** to catch mistakes  

## 📁 Files Added

- **`.github/workflows/release.yml`** - Main release automation workflow
- **`CHANGELOG.md`** - Track changes between versions
- **`bump-version.sh`** - Helper script for version management
- **`RELEASING.md`** - Comprehensive release documentation

## 🔧 Setup Required

Only one setup step is needed:

1. Ensure `CRATES_IO_TOKEN` secret is configured in repository settings
   - Go to Settings → Secrets and variables → Actions
   - Should already exist if you've published before

## 🎯 Next Steps

**For your next release:**

1. Choose your preferred method above
2. Update CHANGELOG.md with changes
3. Follow the steps for your chosen method
4. Monitor Actions tab to watch the automation

**Questions?**

- See detailed docs: `RELEASING.md`
- Check workflow definition: `.github/workflows/release.yml`
- Run version script help: `./bump-version.sh`

## 🎨 Example Changelog Entry

```markdown
## [0.2.9] - 2024-01-15

### Added
- New particle effects system
- Improved animation easing functions

### Fixed
- Memory leak in texture loading
- Crash on window resize

### Changed
- Updated wgpu to version 23.0.0
```

---

**Happy Releasing! 🚀**
