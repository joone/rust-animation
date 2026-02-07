#!/bin/bash
# Script to fix the v0.2.8 tag issue
# This script should be run from the main branch of the repository

set -e  # Exit on error

echo "==================================================================="
echo "Fix v0.2.8 Tag - Move tag to correct commit"
echo "==================================================================="
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Error: Not in a git repository"
    exit 1
fi

# Get the current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo "Current branch: $CURRENT_BRANCH"

# Get the current commit SHA
CURRENT_COMMIT=$(git rev-parse HEAD)
echo "Current commit: $CURRENT_COMMIT"

# Get the version from Cargo.toml
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Cargo.toml version: $CARGO_VERSION"
echo ""

# Check if we're on main branch
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "Warning: You are not on the main branch!"
    echo "It's recommended to run this from the main branch."
    read -p "Do you want to continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

# Verify the commit message
COMMIT_MESSAGE=$(git log -1 --pretty=%B)
echo "Current commit message: $COMMIT_MESSAGE"
echo ""

TAG_NAME="v$CARGO_VERSION"

# Check if tag exists
if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
    TAG_COMMIT=$(git rev-parse "$TAG_NAME")
    echo "Tag $TAG_NAME exists and points to: $TAG_COMMIT"
    
    if [ "$TAG_COMMIT" == "$CURRENT_COMMIT" ]; then
        echo "Tag already points to the current commit. Nothing to do!"
        exit 0
    fi
    
    echo ""
    echo "The tag needs to be moved from $TAG_COMMIT to $CURRENT_COMMIT"
    echo ""
    read -p "Do you want to move the tag? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
    
    echo ""
    echo "Step 1: Deleting local tag..."
    git tag -d "$TAG_NAME"
    echo "✓ Local tag deleted"
    
    echo ""
    echo "Step 2: Deleting remote tag..."
    git push origin ":refs/tags/$TAG_NAME"
    echo "✓ Remote tag deleted"
    
else
    echo "Tag $TAG_NAME does not exist yet."
    echo ""
    read -p "Do you want to create it on the current commit? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

echo ""
echo "Step 3: Creating new tag on current commit..."
git tag "$TAG_NAME"
echo "✓ Tag created: $TAG_NAME -> $CURRENT_COMMIT"

echo ""
echo "Step 4: Pushing tag to remote..."
git push origin "$TAG_NAME"
echo "✓ Tag pushed to remote"

echo ""
echo "==================================================================="
echo "SUCCESS! Tag $TAG_NAME has been created/moved successfully."
echo "==================================================================="
echo ""
echo "Next steps:"
echo "1. Go to https://github.com/joone/rust-animation/actions"
echo "2. Check for the 'Publish to crates.io' workflow run"
echo "3. Monitor the workflow to ensure it completes successfully"
echo "4. Verify the package at https://crates.io/crates/rust-animation"
echo ""
