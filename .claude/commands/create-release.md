---
description: Create a new release with proper versioning and changelog
---

Guide the user through creating a new Unicel release.

Steps:
1. Check current version:
   - Read: package.json and src-tauri/tauri.conf.json
   - Show current version
2. Ask user for new version number (e.g., 0.1.9, 0.2.0)
3. Determine version bump type:
   - Patch (0.1.8 → 0.1.9): Bug fixes
   - Minor (0.1.8 → 0.2.0): New features
   - Major (0.1.8 → 1.0.0): Breaking changes
4. Update version in both files:
   - package.json
   - src-tauri/tauri.conf.json
5. Ask user for changelog/release notes
6. Create git commit:
   ```
   git add -A
   git commit -m "Bump version to X.Y.Z for release"
   ```
7. Create and push git tag:
   ```
   git tag vX.Y.Z
   git push && git push --tags
   ```
8. Explain what happens next:
   - GitHub Actions will trigger release workflow
   - Builds for macOS (Intel + Apple Silicon) and Windows
   - Code signing for macOS builds
   - Draft release created automatically
9. Provide release URL:
   - https://github.com/jacksodj/unicel/releases/tag/vX.Y.Z

Remind user to:
- Test the release builds after they're created
- Update release notes if needed
- Announce the release if it's a significant update
