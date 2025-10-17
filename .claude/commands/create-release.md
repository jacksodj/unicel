---
description: Launch release-manager agent to create a new release (invoked asynchronously)
---

Launch the **release-manager** autonomous agent asynchronously to handle the release process.

**This agent runs asynchronously** because the release process involves:
- Waiting for CI checks (5-10 minutes)
- Building across multiple platforms (15-20 minutes)
- Monitoring external workflows

The agent will report back when the release is complete or needs user input.

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
