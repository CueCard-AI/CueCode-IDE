# Build phase 1.1 — `cuecode_specs` crate {#phase-1-1}

> **Invoke:** `Build phase 1.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x] Done` |
| **Last verified** | 2026-06-19 |
| **Duration** | 5–7 days |
| **Track** | 1 — Spec foundation |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) |
| **QA script** | QA-P1 steps 1–2, 5–7 |

## Deliverable {#phase-1-1-deliverable}

`.cursor/specs/` is indexed at runtime: scan, parse frontmatter, watch for changes, expose `SpecIndex` for agent and UI.

## Depends / blocks {#phase-1-1-deps}

| | Phase |
|---|-------|
| **Depends on** | 0.3 |
| **Blocks** | 1.2 |

## Out of scope {#phase-1-1-out-of-scope}

- System prompt injection, `@spec` composer (1.2)
- Spec browser UI stub (1.3)

---

## Tasks {#phase-1-1-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.1.1 | Create `crates/cuecode_specs/` | `crates/cuecode_specs/Cargo.toml`, `src/lib.rs` | `[x]` |
| 1.1.2 | `SpecIndex` scan `.cursor/specs/**/*.md` | `crates/cuecode_specs/src/index.rs` | `[x]` |
| 1.1.3 | YAML frontmatter parser (lenient) | `crates/cuecode_specs/src/parse.rs` | `[x]` |
| 1.1.4 | Filesystem watcher + debounce ≤2s | `crates/cuecode_specs/src/watch.rs` | `[x]` |
| 1.1.5 | `SpecEntry { path, title, summary, anchor_ids }` | `crates/cuecode_specs/src/index.rs` | `[x]` |
| 1.1.6 | Unit tests with fixture tree | `crates/cuecode_specs/tests/` | `[x]` |

---

## Implementation notes {#phase-1-1-impl}

Create `crates/cuecode_specs/` with these public types (from [06 §new-crates](../../core/06-system-design.md#new-crates)):

```rust
use std::path::{Path, PathBuf};

/// Compact index for system prompt injection.
pub struct SpecIndex {
    pub worktree_id: project::WorktreeId,
    pub entries: Vec<SpecEntry>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct SpecEntry {
    pub path: PathBuf,
    pub title: String,
    pub status: Option<SpecStatus>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub anchor_ids: Vec<String>,
}

pub enum SpecStatus {
    Draft,
    Active,
    Done,
    Deprecated,
}

pub fn load_spec_index(worktree_root: &Path, cx: &App) -> SpecIndex;
pub fn resolve_spec_query(index: &SpecIndex, query: &str) -> Vec<SpecEntry>;
```

- Scan only `.cursor/specs/**/*.md` under the worktree root.
- YAML frontmatter parser: lenient (missing fields OK).
- Filesystem watcher + debounce ≤2s → rebuild index in-memory (`Arc<SpecIndex>` per worktree).
- Index payload budget: titles + paths + status (~4KB target in system prompt).

---

## Verify {#phase-1-1-verify}

```bash
cd CueCode-IDE
cargo test -p cuecode_specs
cargo build -p cuecode_specs
```

---

## Exit criteria {#phase-1-1-exit}

- [x] Index loads ≥3 specs from workspace
- [x] Watcher refresh on new file without restart
- [x] `cargo test -p cuecode_specs` green

---

## QA {#phase-1-1-qa}

Manual steps before marking **Status** `[x]`:

1. Open workspace with `.cursor/specs/` — index loads ≥3 entries
2. Create `.cursor/specs/qa-scratch.md` — index updates within 2s
3. Delete scratch file — entry removed on next index read
4. Run `cargo test -p cuecode_specs` — all green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P1 steps 1–2, 5–7

---

## PR checklist {#phase-1-1-pr}

- [ ] PR title/body cites **Build phase 1.1** and this file
- [x] All task **Done** columns `[x]`
- [x] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-1-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| New crates | ../../core/06-system-design.md#new-crates |
| Spec integration | ../../core/04-sandbox-core.md#spec-integration |
| Conventions | ../../00-README.md#conventions |

---

## Changelog {#phase-1-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
| 2026-06-19 | Implemented `cuecode_specs` crate; tests + clippy green |
