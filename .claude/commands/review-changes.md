---
description: Launch code-reviewer agent to review recent code changes
---

Launch the **code-reviewer** autonomous agent to review code for quality and correctness.

The agent will:
- Get git diff (uncommitted or recent commits)
- Review for unit correctness and dimensional analysis
- Check type safety and error handling
- Validate test coverage
- Identify performance issues
- Provide severity-rated feedback (🔴 Critical | 🟡 Warning | 🔵 Suggestion)
- Give overall assessment (Approved / Changes Requested)
