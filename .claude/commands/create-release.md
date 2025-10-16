---
description: Launch release-manager agent to create a new release
---

Launch the **release-manager** autonomous agent to handle the release process.

The agent will:
- Check current version
- Ask for new version number or help determine it (patch/minor/major)
- Gather changelog from git history
- Update package.json and tauri.conf.json
- Create git commit and tag
- Push to trigger GitHub Actions release workflow
- Monitor build progress (macOS Intel, macOS ARM, Windows)
- Verify release artifacts and code signing
- Provide release URL
