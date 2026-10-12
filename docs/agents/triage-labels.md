# Triage Labels

The skills speak in terms of five canonical triage roles. With the GitHub tracker, each role is a label of the same name, created by the setup-new-repo skill.

| Label in mattpocock/skills | Label in our tracker | Meaning                                  |
| -------------------------- | -------------------- | ---------------------------------------- |
| `needs-triage`             | `needs-triage`       | Maintainer needs to evaluate this issue  |
| `needs-info`               | `needs-info`         | Waiting on reporter for more information |
| `ready-for-agent`          | `ready-for-agent`    | Fully specified, ready for an AFK agent  |
| `ready-for-human`          | `ready-for-human`    | Requires human implementation            |
| `wontfix`                  | `wontfix`            | Will not be actioned                     |

`needs-human` is separate from triage: it marks something the owner must do, and the owner-questions mod shows it above the prompt (see `issue-tracker.md`).
