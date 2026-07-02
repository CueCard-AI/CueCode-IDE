# Build phase 1.4a — QA fixture bootstrap (PulseBoard) {#phase-1-4a}

> **Invoke:** `Build phase 1.4a` — open **this file only**; seed + docs; agent waves run in fixture repo.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Complete — PulseBoard QA-P1-Plan passed 2026-06-23 |
| **Last verified** | 2026-06-23 |
| **Duration** | 0.5–1 day |
| **Track** | 1 — Planning Hub QA |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [ops/13-plan-e2e-fixture](../../ops/13-plan-e2e-fixture.md) |
| **QA script** | [QA-P1-Plan](../../ops/13-plan-e2e-fixture.md#qa-p1-plan) |

## Deliverable {#phase-1-4a-deliverable}

**PulseBoard** light seed lives in `qa-fixture/pulseboard/` and publishes to [CueCard-AI/cuecode-testing-repo](https://github.com/CueCard-AI/cuecode-testing-repo). CueInference specs document E2E Plan QA; **Implement E2E does not run on CueInference dogfood**.

## Depends / blocks {#phase-1-4a-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.4 (hub + `cuecode_plans`) |
| **Blocks** | 1.4 manual sign-off (Implement QA), 1.4b (optional parallel) |

## Out of scope {#phase-1-4a-out-of-scope}

- Full polyglot product (agent generation waves)
- Organize messy-repo scenario (1.6)

---

## Tasks {#phase-1-4a-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4a.1 | ops spec: fixture roles, QA-P1-Plan, waves | `.cursor/specs/ops/13-plan-e2e-fixture.md` | `[x]` |
| 1.4a.2 | Light PulseBoard seed (manifest + stub stacks) | `qa-fixture/pulseboard/` | `[x]` |
| 1.4a.3 | Cross-links: 16 exit, 1-4 QA, 07, 06 | `.cursor/specs/` | `[x]` |
| 1.4a.4 | Push seed to `cuecode-testing-repo` | `./script/sync-qa-fixture --to-remote --push` (CI on `qa-fixture/**`) | `[x]` |
| 1.4a.5 | Run QA-P1-Plan in CueCode on `cuecode-testing-repo/` | manual | `[x]` |

---

## Verify {#phase-1-4a-verify}

```bash
# From CueInference root
test -f qa-fixture/pulseboard/.cuecode/project.yaml
./script/clone-qa-fixture-repo
./cuecode-testing-repo/scripts/verify-all.sh

# After publish — open cuecode-testing-repo/ in CueCode, run QA-P1-Plan
```

---

## Exit criteria {#phase-1-4a-exit}

- [x] ops/13 and seed path documented
- [x] Fixture repo contains seed (or rsync from monorepo)
- [x] QA-P1-Plan steps 2–3 pass on fixture; step 5 optional without Send

---

## Changelog {#phase-1-4a-changelog}

| Date | Change |
|------|--------|
| 2026-06-23 | QA-P1-Plan passed on pulseboard fixture; 1.4a.5 signed off |
| 2026-06-22 | Initial sub-phase |
