# Build phase 1.3 — Spec UI stub + skills meta {#phase-1-3}

> **⚠ Superseded** — Do **not** implement. Use **[1.3-rev Planning Hub v0](./1-3-rev-planning-hub-v0.md)** → [1.4](./1-4-planning-hub-manifest.md) → [1.6 Organize](./1-6-organize-with-ai.md).  
> Design: [16-planning-hub §supersedes-1-3](../../design/16-planning-hub.md#supersedes-1-3)

| Field | Value |
|-------|-------|
| **Status** | `[~]` **Superseded** — spike code exists; product direction replaced |
| **Replacement** | [1-3-rev-planning-hub-v0](./1-3-rev-planning-hub-v0.md) (P-H0) |

## Why superseded {#phase-1-3-superseded}

Legacy tasks (header dropdown, dumb spec browser) conflict with [Planning Hub](../../design/16-planning-hub.md). Reuse patterns from `spec_linker.rs` / `spec_browser.rs` only when building 1.3-rev — do not QA or ship legacy UX.

| Legacy 1.3 task | Replacement |
|-----------------|-------------|
| 1.3.1 Header spec linker | Remove; pin chip + hub (1.3-rev) |
| 1.3.2 Open Spec Browser | **CueCode: Planning Hub** (1.3-rev) |
| 1.3.3 `implement-spec` skill | Keep; retarget in 1.3-rev / 1.4 |

---

## Original doc (archival) {#phase-1-3-archival}

<details>
<summary>Original 1.3 execution contract (do not invoke)</summary>

> **Invoke:** ~~Build phase 1.3~~ — use **Build phase 1.3-rev**

| Field | Value |
|-------|-------|
| **Duration** | 2–3 days |
| **Depends on** | 1.2 |
| **Blocks** | ~~2.1~~ → now **1.6** blocks 2.1 |

Deliverable was: Spec linker in agent header, command palette spec browser, `implement-spec` skill stub.

</details>

---

## Changelog {#phase-1-3-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
| 2026-06-19 | Marked superseded by Planning Hub track 1.3-rev–1.6 |
