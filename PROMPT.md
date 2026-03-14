# ezpdf — Rust PDF CLI Tool Implementation

You are implementing `ezpdf`, a fast lossless PDF manipulation CLI tool in Rust.
Working directory: `/Users/enzhou/Projects/ez-pdf/`

---

## Plan Files (read these every iteration)

- **Task plan** (phases, tasks, DoD): `task_plan.md`
- **Architecture plan** (stories, acceptance criteria, code shapes): `../docs/plans/2026-03-12-001-feat-ezpdf-cli-tool-plan.md`
- **Brainstorm** (key decisions and rationale): `../docs/brainstorms/2026-03-12-pdf-cli-tool-brainstorm.md`

---

## Workspace

- **Working directory**: `/Users/enzhou/Projects/ez-pdf/`
- **Branch**: `main` (or create `feat/<story>` for each story, then merge to main)

---

## Your Job Each Iteration

1. Read `task_plan.md` to find the **next unchecked task** (lowest phase + task number with `- [ ]`)
2. If the task is `[RED]`, invoke `superpowers:test-driven-development` skill — write **failing tests only**, do NOT write any implementation code
3. If the task is `[GREEN]`, invoke `superpowers:test-driven-development` skill — write the **minimum implementation** to make the RED tests pass
4. If the task is `[REFACTOR]`, clean up code without changing behaviour — all tests must still pass
5. If the task is `[SETUP]`, perform the described setup work (no TDD required for infrastructure tasks)
6. If the task is `[REVIEW]`, validate against the Phase Definition of Done — update plan if issues found
7. Mark the task as done (`- [x]`) in `task_plan.md` when complete
8. Commit with a conventional commit message on the current branch
9. Output a one-line status: `✓ Task X.Y complete: <what was done>`

---

## Non-Negotiable Rules

- **TDD always**: invoke `superpowers:test-driven-development` skill before writing any `[RED]` or `[GREEN]` task
- **Never skip RED**: no implementation code without a failing test already committed
- **Verify RED fails**: run `cargo test` after writing tests — confirm they fail before moving to `[GREEN]`
- **Verify GREEN passes**: run `cargo test` after implementing — confirm all tests pass before marking `[x]`
- **Never write more than needed**: `[GREEN]` means minimum code to pass the tests — nothing extra
- **No panics**: all errors must go through `EzPdfError` — no `unwrap()` or `expect()` in library code
- **No content stream decoding**: lossless is the core guarantee — never re-encode PDF content streams
- **All PRs must be draft**: `gh pr create --draft`
- **Run clippy before marking REVIEW done**: `cargo clippy --workspace -- -D warnings` must pass
- **No `[x]` without evidence**: paste the test output in the commit message or progress log

---

## Progress Logging

After completing a `[REVIEW]` task, append a section to `progress.md` (create it if it doesn't exist):

```
## YYYY-MM-DD — Phase N complete: <Phase Name>

**Completed tasks:**
- N.1 through N.X

**Tests passing:** <count>

**Deviations / blockers found:**
- (or "none")
```

Do NOT update `progress.md` for individual RED/GREEN/REFACTOR tasks — only at phase `[REVIEW]` boundaries.

---

## Blocker Protocol

If you discover a blocker during a task (something that prevents you from completing the story):

1. DO NOT continue with the current task
2. Append to `task_plan.md` under a `### Blockers` section:
   ```
   - [ ] **BLOCKER-N** [BLOCKER] <description> — found during Task X.Y
   ```
3. Note it in your status output: `⚠ BLOCKER found: <short description>`
4. The loop will stop — the user will inject the blocker as the next task

---

## Completion Signal

When ALL tasks across ALL phases in `task_plan.md` are checked off (`- [x]`), output:

<promise>EZPDF BACKLOG COMPLETE</promise>
