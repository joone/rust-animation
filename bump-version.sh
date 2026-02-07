#!/bin/bash
# Version bump script for rust-animation
# Usage: ./bump-version.sh [major|minor|patch|VERSION] [--auto|--push]
# Examples:
#   ./bump-version.sh patch           # 0.2.8 -> 0.2.9 (manual workflow)
#   ./bump-version.sh patch --auto    # 0.2.8 -> 0.2.9 (automatic commit, push, and tag)
#   ./bump-version.sh minor --push    # 0.2.8 -> 0.3.0 (automatic commit, push, and tag)
#   ./bump-version.sh major           # 0.2.8 -> 1.0.0 (manual workflow)
#   ./bump-version.sh 0.3.5 --auto    # Set to specific version (automatic workflow)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Cargo.toml exists
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Cargo.toml not found in current directory${NC}"
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${YELLOW}Current version: $CURRENT_VERSION${NC}"

# Parse version components
IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR="${VERSION_PARTS[0]}"
MINOR="${VERSION_PARTS[1]}"
PATCH="${VERSION_PARTS[2]}"

# Parse command line arguments
AUTO_MODE=false
VERSION_ARG=""

for arg in "$@"; do
    case "$arg" in
        --auto|--push)
            AUTO_MODE=true
            ;;
        *)
            if [ -z "$VERSION_ARG" ]; then
                VERSION_ARG="$arg"
            fi
            ;;
    esac
done

# Determine new version based on argument
if [ -z "$VERSION_ARG" ]; then
    echo "Usage: $0 [major|minor|patch|VERSION] [--auto|--push]"
    echo ""
    echo "Examples:"
    echo "  $0 patch           # $CURRENT_VERSION -> $MAJOR.$MINOR.$((PATCH + 1)) (manual)"
    echo "  $0 minor           # $CURRENT_VERSION -> $MAJOR.$((MINOR + 1)).0 (manual)"
    echo "  $0 major           # $CURRENT_VERSION -> $((MAJOR + 1)).0.0 (manual)"
    echo "  $0 0.3.5           # $CURRENT_VERSION -> 0.3.5 (manual)"
    echo "  $0 patch --auto    # Automatically commit, push, and tag"
    echo "  $0 minor --push    # Same as --auto"
    exit 1
fi

case "$VERSION_ARG" in
    major)
        NEW_VERSION="$((MAJOR + 1)).0.0"
        ;;
    minor)
        NEW_VERSION="$MAJOR.$((MINOR + 1)).0"
        ;;
    patch)
        NEW_VERSION="$MAJOR.$MINOR.$((PATCH + 1))"
        ;;
    *)
        # Assume it's a specific version number
        if [[ ! "$VERSION_ARG" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            echo -e "${RED}Error: Invalid version format. Use X.Y.Z format${NC}"
            exit 1
        fi
        NEW_VERSION="$VERSION_ARG"
        ;;
esac

echo -e "${GREEN}New version: $NEW_VERSION${NC}"

# Confirm with user
read -p "Update version from $CURRENT_VERSION to $NEW_VERSION? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 0
fi

# Update Cargo.toml
sed -i.bak "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

echo -e "${GREEN}✓ Updated Cargo.toml${NC}"

# Update CHANGELOG.md if it exists
if [ -f "CHANGELOG.md" ]; then
    TODAY=$(date +%Y-%m-%d)
    
    # Check if there's an [Unreleased] section
    if grep -q "## \[Unreleased\]" CHANGELOG.md; then
        # Add new version section after Unreleased
        sed -i.bak "/## \[Unreleased\]/a\\
\\
## [$NEW_VERSION] - $TODAY" CHANGELOG.md
        
        # Update comparison links at the bottom
        if grep -q "\[Unreleased\]:" CHANGELOG.md; then
            # Update Unreleased link
            sed -i.bak "s|\[Unreleased\]:.*|\[Unreleased\]: https://github.com/joone/rust-animation/compare/v$NEW_VERSION...HEAD|" CHANGELOG.md
            
            # Add new version link if it doesn't exist
            if ! grep -q "\[$NEW_VERSION\]:" CHANGELOG.md; then
                sed -i.bak "/\[Unreleased\]:/a\\
[$NEW_VERSION]: https://github.com/joone/rust-animation/releases/tag/v$NEW_VERSION" CHANGELOG.md
            fi
        fi
        
        rm CHANGELOG.md.bak
        echo -e "${GREEN}✓ Updated CHANGELOG.md${NC}"
        echo -e "${YELLOW}Note: Please update the CHANGELOG.md [Unreleased] section with changes for v$NEW_VERSION${NC}"
    else
        echo -e "${YELLOW}Warning: No [Unreleased] section found in CHANGELOG.md${NC}"
        echo -e "${YELLOW}Please manually update CHANGELOG.md with release notes${NC}"
    fi
fi

# Show what changed
echo ""
echo -e "${GREEN}Changes made:${NC}"
echo "  Cargo.toml: version = \"$NEW_VERSION\""
if [ -f "CHANGELOG.md" ]; then
    echo "  CHANGELOG.md: Added section for [$NEW_VERSION]"
fi

echo ""

# Execute automatic workflow if --auto flag is provided
if [ "$AUTO_MODE" = true ]; then
    echo -e "${GREEN}=== Automatic Release Workflow ===${NC}"
    echo ""
    
    # Check if we're on main branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [ "$CURRENT_BRANCH" != "main" ]; then
        echo -e "${YELLOW}Warning: You are on branch '$CURRENT_BRANCH', not 'main'${NC}"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Aborted."
            exit 0
        fi
    fi
    
    echo -e "${YELLOW}This will:${NC}"
    echo "  1. Commit Cargo.toml and CHANGELOG.md"
    echo "  2. Push to origin/$CURRENT_BRANCH"
    echo "  3. Create and push tag v$NEW_VERSION"
    echo "  4. Trigger automated release workflow"
    echo ""
    read -p "Proceed with automatic release? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted. Changes have been made but not committed."
        echo "Run 'git status' to see the changes."
        exit 0
    fi
    
    echo ""
    echo -e "${GREEN}Step 1: Committing changes${NC}"
    git add Cargo.toml CHANGELOG.md
    git commit -m "Bump version to $NEW_VERSION"
    echo -e "${GREEN}✓ Changes committed${NC}"
    
    echo ""
    echo -e "${GREEN}Step 2: Pushing to origin/$CURRENT_BRANCH${NC}"
    git push origin "$CURRENT_BRANCH"
    echo -e "${GREEN}✓ Changes pushed${NC}"
    
    echo ""
    echo -e "${GREEN}Step 3: Creating and pushing tag v$NEW_VERSION${NC}"
    git tag "v$NEW_VERSION"
    git push origin "v$NEW_VERSION"
    echo -e "${GREEN}✓ Tag v$NEW_VERSION created and pushed${NC}"
    
    echo ""
    echo -e "${GREEN}=== Release Workflow Complete! ===${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo "  1. Monitor the GitHub Actions workflow:"
    echo "     https://github.com/joone/rust-animation/actions"
    echo "  2. The workflow will automatically:"
    echo "     - Run tests and build the project"
    echo "     - Create a GitHub Release"
    echo "     - Publish to crates.io"
    echo "     - Build example binaries for multiple platforms"
    echo ""
    echo -e "${GREEN}Release process initiated successfully!${NC}"
else
    echo -e "${YELLOW}Next steps (Manual Workflow):${NC}"
    echo "  1. Review and update CHANGELOG.md with changes for this release"
    echo "  2. Commit the changes:"
    echo "     git add Cargo.toml CHANGELOG.md"
    echo "     git commit -m \"Bump version to $NEW_VERSION\""
    echo "     git push origin main"
    echo "  3. Create a release:"
    echo "     Option A - Create tag locally:"
    echo "       git tag v$NEW_VERSION"
    echo "       git push origin v$NEW_VERSION"
    echo "     Option B - Use GitHub Actions (go to Actions → Create Release → Run workflow)"
    echo "     Option C - Use automatic workflow:"
    echo "       ./bump-version.sh $VERSION_ARG --auto"
    echo ""
fi
