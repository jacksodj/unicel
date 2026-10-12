# Unicel: agent instructions

## What this is

Unicel is an open-source, unit-aware spreadsheet desktop app. Every value is a `(number, unit)` pair, so formulas do dimensional analysis and unit cancellation (`$100/hr × 720hr/month → $72,000/month`), and switching the display between As Entered, Metric and Imperial never changes stored data. Workbooks save as JSON (`.usheet`); Excel export is one-way, with metadata sheets that record the units. An MCP server exposes workbooks to AI tools.

It is a Tauri 2 app: a Rust library holds the whole calculation engine, and a React + TypeScript frontend calls it through Tauri commands. The repo is public and dual-licensed MIT OR Apache-2.0.

`requirements/Unit Aware Spreadsheet Design.md` is the full design spec. Parts of it are not built yet (an in-memory SQLite runtime, SQL table queries, Zustand state, a canvas grid, live currency rates); check the code before assuming a feature exists.

## Layout

```
src-tauri/                  Rust crate `unicel` (lib `unicel_lib` + the app binary); the root Cargo.toml is a one-member workspace
  src/core/units/           Unit library, unit parsing, dimensions (library.rs, parser.rs)
  src/core/formula/         Formula grammar (formula.pest), parser, AST, unit-aware evaluator
  src/core/cell/, cell_input.rs   Cell values and parsing user input into value + unit
  src/core/conversion/      Display conversion (storage unit vs display unit)
  src/core/table/, workbook/      Sheets and workbook model
  src/core/settings.rs      Workbook settings (unit preferences)
  src/formats/              json.rs (.usheet load/save), excel.rs (one-way export)
  src/commands/             Tauri commands the frontend calls
  src/mcp/                  MCP server, tools and types
  src/standalone/mcp-server.rs    Standalone MCP server binary (not declared in Cargo.toml; see below)
  tests/                    Rust integration tests, one file per topic
  examples/                 Rust examples that generate and exercise the sample workbooks
  tauri.conf.json           App config (version, window, bundling)
src/                        React + TypeScript frontend (components/, api/tauri.ts wraps the Tauri commands, types/)
examples/                   Sample .usheet workbooks
requirements/               Design spec, original requirements and use cases
docs/                       User guide, MCP docs, project plan, task history, changelog, release notes
docs/agents/                Issue-tracker, triage-label and domain notes for agents
.claude/agents/, .claude/commands/   Project agents (commit-gatekeeper, release-manager, unit-debugger, ...) and slash commands
.claude/skills/             Vendored planning skills (installed by the repo standard; skills-lock.json pins them)
.github/workflows/          pr-checks.yml (merge gate), ci.yml (cross-platform build and lint), release.yml (tagged releases), roadmap-dates.yml
```

## Build, test and run

Prerequisites: Rust stable, Node 18+ (CI uses 20), and on Linux the Tauri system packages (`libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`). `package-lock.json` is gitignored, so `npm install` resolves fresh each time.

```bash
npm install
npm run build                 # tsc + vite build into dist/ (the Rust build embeds dist/, so build it first)
cargo test                    # all Rust tests (unit tests inside src-tauri/src plus src-tauri/tests/)
cargo test --test long_unit_names   # one integration test file
cargo fmt --check
cargo clippy -- -D warnings

npm run tauri:dev             # run the desktop app with hot reload
npm run tauri:build           # production bundle for this platform
npm run dev                   # frontend only, in a browser, without the Rust backend
cargo run --example unit_conversion_tutorial   # regenerate a sample workbook (run from src-tauri/)
```

Cargo commands work from the root (workspace) or with `--manifest-path=./src-tauri/Cargo.toml`, as CI does.

The frontend has no tests yet (`npm run test` runs vitest over nothing), and there is no ESLint config yet, so `npm run lint` is not a working check.

The standalone MCP server binary is deliberately not declared in `src-tauri/Cargo.toml`, so Tauri does not bundle it. To build it, add the commented `[[bin]]` block there locally (see `docs/MCP_SERVER.md`).

### CI

- `pr-checks.yml` is the merge gate: `tests` (shellcheck on the hook scripts, then the frontend build, `cargo test` and `cargo fmt --check` on Ubuntu), `secrets` (gitleaks), and `vulnerabilities`.
- `ci.yml` runs the build, tests, `cargo fmt --check` and `cargo clippy -- -D warnings` on Ubuntu, macOS and Windows for pushes and PRs to `main`. Clippy follows the latest stable toolchain, so new lints can turn it red without a code change.
- `release.yml` runs on a `v*` tag: it builds macOS (Apple Silicon and Intel) and Windows bundles with `tauri-action`, using the Apple signing secrets, and publishes a GitHub release. Change it only when the owner asks.

## Conventions

- **Units are data.** A cell's storage unit is what the user entered and never changes as a side effect; the display unit is a non-destructive view. Formulas compute on storage units, so a display toggle never recalculates.
- **Dimensional analysis first.** Addition needs compatible dimensions; multiplication and division build compound units and cancel matching ones (`100mi / 2hr → 50 mi/hr`, `100m / 50m → 2`).
- **Soft warnings, not errors.** Incompatible operations (`5m + 10s`) still calculate, mark the cell with a warning and explain it in a tooltip.
- **Adding a unit:** add it to `src-tauri/src/core/units/library.rs` with its dimension and conversion factors, then test it under `src-tauri/tests/`.
- **Adding a formula function:** extend the grammar in `formula.pest` and the parser if needed, implement it in `src-tauri/src/core/formula/evaluator.rs` with unit-aware logic (check dimensions, preserve or derive units), and add an integration test such as `src-tauri/tests/ceiling_function.rs`.
- **Rust style:** rustfmt defaults; clippy clean with `-D warnings`; return `Result` rather than panic; `///` doc comments on public APIs; `anyhow` in application code, `thiserror` for library errors.
- **TypeScript style:** Prettier (`.prettierrc`, `npm run format`); functional components with hooks; prefer `const`; no `any`; descriptive names.
- **Every behavior change gets a test,** either a `#[cfg(test)]` module beside the code or a file in `src-tauri/tests/`.
- **Before committing,** run `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test` and `npm run build`. The `commit-gatekeeper` agent in `.claude/agents/` runs the same checks, then commits and pushes the current branch; use it from a worktree branch, never from `main`.
- **Releases:** the `release-manager` agent and `/create-release` bump the version in `package.json` and `src-tauri/tauri.conf.json`. The bump goes through a PR like any other change; tag `vX.Y.Z` on `main` only after that PR's checks pass and it merges.
- **Tracking:** new work goes in GitHub Issues (see `docs/agents/issue-tracker.md`). `docs/TASKS.md` is the history of the original phased plan.
- **Docs:** the root holds only `README.md`, `AGENTS.md` and `CLAUDE.md`; every other Markdown file goes under `docs/`. `docs/README.md` is the documentation index; keep it current when you add a doc.
- **Public repo:** commit no absolute local paths, account IDs, emails, tokens or private repo names. Write paths relative to the repo root.

## Guardrails

The same controls as the owner's other repos. Work with them, not around them.

- After cloning, run `scripts/setup-hooks.sh`. It points git at `.githooks/` (gitleaks scans every commit and push) and installs the pinned gitleaks. The Claude Code hook in `.claude/settings.json` runs it automatically before any commit or push.
- For a gitleaks false positive, add a targeted allowlist entry to `.gitleaks.toml`; commits always go through the hooks (no `--no-verify`).
- Work in a worktree. The main checkout stays on `main`, clean, and read-only for sessions. Before your first edit, call `EnterWorktree` with a name for the work: it cuts a branch from `origin/main` under `.claude/worktrees/` (gitignored). Push that branch and open the PR from it; to rename the branch, use `git branch -m`, never `git switch -c`, which leaves an orphaned branch behind. After the merge, GitHub deletes the remote branch, the Code tab's auto-archive removes a session's worktree, and the next session in the repo sweeps the merged local branch; never use `--force`, `-D`, `reset --hard` or `clean -f` to tidy up.
- `main` is protected: the `tests`, `secrets`, and `vulnerabilities` jobs in `.github/workflows/pr-checks.yml` must pass. Push a branch, open a PR, and merge with `gh pr merge --auto --squash`, including tracker and docs edits. No `--admin`, no force-push to `main`.
- The workflow uses no repository secrets and never `pull_request_target`. Keep the three job names stable; add the project's build and test steps to `tests` with the first build work. The vulnerability steps switch on by themselves when Python requirements, Python code, or npm lockfiles appear; add an audit for any other package manager the project adopts.
- OpenTofu/Terraform: CI runs `tofu fmt`, `tofu validate`, and tflint on any `.tf` files. Destroys and state surgery (`destroy`, `-destroy`, `state rm/push`) never run locally; stateful resources carry `prevent_destroy = true`; real infrastructure with state adopts the standard `tofu-deploy-reusable.yml` workflow (plan, destroy-guard, and the `allow-destroy` label that only the owner adds).
- Anything the owner must do is a `needs-human` issue with a `**Next:**` line, filed when it comes up rather than left in a reply, and closed or rewritten when the Next goes stale (see `docs/agents/issue-tracker.md`). If the effort spans repos, every repo is linked to the effort's private GitHub Project so the band shows the siblings' items. Give each one a Priority (`scripts/issue-field.sh <n> Priority High`). Decisions go to the owner one at a time through AskUserQuestion.
- Roadmap data lives on each issue as organization issue fields: set Horizon (`scripts/issue-field.sh <n> Horizon Now`) when charting a map or triaging a report, file tickets with `--parent` and `--blocked-by`, and leave Start date and Target date to the `roadmap-dates` workflow (see `docs/agents/issue-tracker.md`).
