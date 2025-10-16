---
description: Investigate and fix performance issues
---

Investigate and fix performance issues in Unicel.

I will help you:
1. Ask what performance issue you're experiencing (slow calculations, UI lag, etc.)
2. Profile the issue (frontend or backend?)
3. Check against performance targets from CLAUDE.md:
   - 10,000 cells load: <1 second
   - Formula recalculation (1,000 cells): <200ms
   - Display toggle: <100ms
   - Cell edit to display: <16ms (60 FPS)
4. Investigate common bottlenecks:
   - Unnecessary cloning in Rust
   - Inefficient SQLite queries
   - React re-render issues
   - Missing memoization
5. Propose and implement optimizations
6. Verify performance improvement

Describe the performance issue and I'll help diagnose and fix it.
