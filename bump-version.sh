#!/bin/bash
# Version bump script for rust-animation
# Usage: ./bump-version.sh [major|minor|patch|VERSION]
# Examples:
#   ./bump-version.sh patch    # 0.2.8 -> 0.2.9
#   ./bump-version.sh minor    # 0.2.8 -> 0.3.0
#   ./bump-version.sh major    # 0.2.8 -> 1.0.0
#   ./bump-version.sh 0.3.5    # Set to specific version

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

# Determine new version based on argument
if [ $# -eq 0 ]; then
    echo "Usage: $0 [major|minor|patch|VERSION]"
    echo ""
    echo "Examples:"
    echo "  $0 patch    # $CURRENT_VERSION -> $MAJOR.$MINOR.$((PATCH + 1))"
    echo "  $0 minor    # $CURRENT_VERSION -> $MAJOR.$((MINOR + 1)).0"
    echo "  $0 major    # $CURRENT_VERSION -> $((MAJOR + 1)).0.0"
    echo "  $0 0.3.5    # $CURRENT_VERSION -> 0.3.5"
    exit 1
fi

case "$1" in
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
        if [[ ! "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            echo -e "${RED}Error: Invalid version format. Use X.Y.Z format${NC}"
            exit 1
        fi
        NEW_VERSION="$1"
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
echo -e "${YELLOW}Next steps:${NC}"
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
echo ""
