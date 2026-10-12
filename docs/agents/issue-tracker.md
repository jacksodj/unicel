# Issue tracker: GitHub

Issues, specs, and wayfinder maps for this repo live as GitHub issues. Use the `gh` CLI for all operations; it infers the repo from `git remote -v`.

GitHub Issues is the tracker because the global Claude Code mods read it: **owner-questions** shows open `needs-human` issues in a band above the prompt (decisions first), and **backlog-queue** files follow-ups as `wayfinder:task` sub-issues of the open `wayfinder:map` (also the `/backlog` command).

## Conventions

- **Create**: `gh issue create --title "..." --body "..."` (heredoc for multi-line bodies).
- **Read**: `gh issue view <n> --comments`.
- **List**: `gh issue list --state open --json number,title,body,labels --jq '[.[] | {number, title, labels: [.labels[].name]}]'` with `--label` / `--state` filters.
- **Comment / label / close**: `gh issue comment`, `gh issue edit --add-label` / `--remove-label`, `gh issue close --comment`.
- Refer to issues by title in anything the owner reads; the number rides inside the link.

## Owner items (`needs-human`)

Anything the owner must *do* (an apply, a console click, a phone setting, a credential) is an issue labelled `needs-human`, never a prose request. Put one line `**Next:** <the single thing to do next>` in the body; the band shows it. A future item gets `**Not before:** YYYY-MM-DD` (or `YYYY-MM-DD HH:MM`) and stays hidden until then. Where a wizard exists for the step, the Next line names it. Decisions go through AskUserQuestion, one at a time, not issues.

- **File it the moment it exists.** A to-do mentioned only in a reply scrolls away; file the issue in the same turn and refer to it by title.
- **File it where the work is.** The item lives in the repo whose work it unblocks, even when the session runs elsewhere.
- **Keep Next current.** When a step is done or overtaken, close the issue with what happened, or rewrite its Next line. A stale Next line is worse than none.
- **More than one owner.** Assign the to-do to the person who must do it. The band shows each person their own items plus unassigned ones, so an unassigned item is everyone's to triage.
- **Give it a Priority** when you file it: `scripts/issue-field.sh <n> Priority <Urgent|High|Medium|Low>`. The band and each session's start take the owner's to-dos highest priority first, then oldest. The roadmap workflow makes every to-do Horizon Now.
- **Repo groups.** The band reads the session's own repo plus every repo linked to an open GitHub Project that repo is linked to. Every repo is in exactly one project: an effort that spans repos has one private project with all of its repos linked (`gh project link <number> --owner <owner> --repo <name>`), so an item filed in any of them shows up whichever repo the session runs in; a repo with no group has a project of its own. Maps and `needs-human` to-dos also belong in the org-wide "Portfolio" project, which is linked to no repo.

## Roadmap data (Horizon and dates)

Every issue carries the data a GitHub Project's Roadmap view needs, as organization issue fields on the issue itself:

- **Horizon** (Now / Next / Later) is the gate: Now and Next are committed, Later is backlog and future; unset means not yet triaged. Set it with `scripts/issue-field.sh <n> Horizon <Now|Next|Later>`: on a map when it's charted (ask the owner), and on a filed report when it's triaged. Tickets inherit their map's Horizon.
- **Start date and Target date** are kept by the `roadmap-dates` workflow (`.github/workflows/roadmap-dates.yml`): only open, unblocked Now work gets them, claiming moves the start, and closing sets the target. Don't set dates by hand.
- **File tickets with their links**, so the workflow sees them at once: `gh issue create --parent <map> --blocked-by <n>`. Links added afterwards are picked up by the nightly sweep.

## Pull requests as a triage surface

**PRs as a request surface: no.**

## When a skill says "publish to the issue tracker"

Create a GitHub issue.

## When a skill says "fetch the relevant ticket"

Run `gh issue view <number> --comments`.

## Wayfinding operations

Used by `/wayfinder`. The **map** is a single issue with **child** issues as tickets.

- **Map**: one issue labelled `wayfinder:map`, holding the Destination / Notes / Decisions-so-far / Not yet specified / Out of scope body. Ask the owner for its Horizon when charting and set it (`scripts/issue-field.sh <map> Horizon Now`); its tickets inherit it.
- **Child ticket**: an issue linked to the map as a GitHub sub-issue: file it with `gh issue create --parent <map>` (or link it later with `gh api --method POST repos/<owner>/<repo>/issues/<map>/sub_issues -F sub_issue_id=<child-db-id>`). Labels: `wayfinder:<type>` (`research`/`prototype`/`grilling`/`task`). A claimed ticket is assigned to the driving dev.
- **Blocking**: GitHub's native issue dependencies: `gh issue create --blocked-by <n>` when filing, or `gh api --method POST repos/<owner>/<repo>/issues/<child>/dependencies/blocked_by -F issue_id=<blocker-db-id>`, where `<blocker-db-id>` is the blocker's numeric database id (`gh api repos/<owner>/<repo>/issues/<n> --jq .id`, not the `#number`). The blocker may live in a sibling repo. A ticket is unblocked when every blocker is closed (`issue_dependencies_summary.blocked_by == 0`).
- **Frontier**: the map's open sub-issues with no open blocker and no assignee; first in map order wins.
- **Claim**: `gh issue edit <n> --add-assignee @me`, the session's first write.
- **Resolve**: `gh issue comment <n> --body "<answer>"`, `gh issue close <n>`, then append a context pointer (gist + link) to the map's Decisions so far with `gh issue edit <map> --body-file`.
- **Research**: a research ticket's findings go in `docs/research/<slug>.md`, committed on a `research/<slug>` branch cut from `origin/main` and merged through a PR (`gh pr merge --auto --squash`). The resolution comment links to `blob/main/docs/research/<slug>.md`. Findings are never left on an unmerged branch.
