#!/usr/bin/env bash
set -e

TAURI_CONFIG="desktop/src-tauri/tauri.conf.json"

BUMP_TYPE=${1:-patch}

if ! command -v jq &> /dev/null; then
  echo "jq is required but not installed."
  exit 1
fi

# Read current version
CURRENT_VERSION=$(jq -r '.version' "$TAURI_CONFIG")

IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

case "$BUMP_TYPE" in
  major)
    MAJOR=$((MAJOR + 1))
    MINOR=0
    PATCH=0
    ;;
  minor)
    MINOR=$((MINOR + 1))
    PATCH=0
    ;;
  patch)
    PATCH=$((PATCH + 1))
    ;;
  *)
    echo "Invalid bump type: $BUMP_TYPE"
    echo "Use: patch | minor | major"
    exit 1
    ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
TAG="v$NEW_VERSION"

echo "Current version: $CURRENT_VERSION"
echo "New version: $NEW_VERSION"

# Update tauri.conf.json
tmp=$(mktemp)
jq ".version = \"$NEW_VERSION\"" "$TAURI_CONFIG" > "$tmp"
mv "$tmp" "$TAURI_CONFIG"

echo "Updated Tauri version."

# Commit
git add "$TAURI_CONFIG"
git commit -m "release: v$NEW_VERSION"

# Tag
git tag "$TAG"

# Push
# git push origin release
git push origin "$TAG"

echo "Release $TAG created 🚀"