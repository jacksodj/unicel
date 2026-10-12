# Domain Docs

How the engineering skills should consume this repo's domain documentation.

This repo is **single-context**. The repo root holds only `README.md`, `AGENTS.md`, and a one-line `CLAUDE.md` pointer, so the glossary lives at **`docs/CONTEXT.md`** instead of the root. ADRs live at **`docs/adr/`**. Both are created lazily, when the first term or decision is resolved.

## Before exploring, read these

- **`docs/CONTEXT.md`**: the glossary. Use its terms in issue titles, proposals, and test names.
- **`docs/adr/`**: ADRs touching the area you are working in.

When a skill says "create `CONTEXT.md` at the repo root", create or edit `docs/CONTEXT.md` instead. Flag any output that contradicts an ADR rather than silently overriding it.
