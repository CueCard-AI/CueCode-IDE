# Cue Labs — hiring challenge: Small Model, Big Harness {#hiring-challenge}

> **Parent:** [marketing/README](./README.md) · [04-cuelabs](./04-cuelabs.md)
>
> **Status:** Open-ended multi-track **cracked bar** kit (spec). Smoke fixtures
> ship today (`hiring/small-model-big-harness/` → public shadow). Full track
> packs + hosted model are phased (M1–M4).
>
> **Surface:** `cuelabs.cloud/hiring` → `/hiring/challenge` ·
> [`CueCard-AI/small-model-big-harness`](https://github.com/CueCard-AI/small-model-big-harness)
>
> **Thesis:** Cue Labs makes agents cheap. This challenge filters for people who
> can make a **weak model look strong** via harness design — and who want a
> high bar.

---

## One sentence {#one-sentence}

An **open-ended** hiring lab: same locked **2-bit GLM 5.2**, a kit of **real
benchmark environments + fixtures** in one repo, maximize lift toward
Opus-class performance however you want — we hire on depth, taste, and proof,
not checklist completion.

---

## Intent (locked) {#intent}

### Who this is for

Cracked candidates who get excited by an unfair constraint and a big playground.
If the problem feels too hard, good.

### Who this is not for

Leetcode tourists, prompt-only applicants, people who need a single correct
homework with a participation grade.

### Bar signal (public)

> We’re not hiring for leetcode.
> Make a 2-bit model look unfair. Same weights. The environments we ship.
> Show lift. Explain what still fails.
> If that doesn’t sound like a good time, you won’t like working here.

### Open-ended (locked)

- Candidates may attack **any subset** of tracks.
- Depth on a hard track beats shallow tourism across all tracks.
- Completing every track is **not** required and is **not** the grading bar.
- Cross-track transfer (one harness, multiple environments) is a strong plus.

---

## Goals and non-goals {#goals}

### Goals

1. Attract people who think in **agent systems** under a brutal model constraint.
2. Ship a **single kit**: model contract + baseline + fixtures + scoring — no
   scavenger hunt for SWE-bench / Terminal / WebArena packs.
3. Keep scores **comparable** within a track (pinned packs, budgets, baseline).
4. Grade on **lift + harness taste + failure analysis**, with Opus as reference.
5. Keep `/hiring` sparse; detail lives on `/hiring/challenge` + the public repo.

### Non-goals

- Requiring Opus parity as pass/fail.
- Requiring every track completed.
- Fine-tuning / distillation as the solution (v1).
- Revealing CueCloud private architecture, SKUs, or proprietary harness code.
- Treating LMArena Frontend/WebDev Elo as a downloadable graded track.

### Later

- Hosted locked 2-bit inference for all candidates.
- Anonymous public leaderboard per track.
- CueCode / IDE embed as an optional feel-test (not the primary score).

---

## Framing (locked language) {#framing}

| Field | Value |
|-------|-------|
| Title | **Small Model, Big Harness** |
| Aspiration | Make a 2-bit GLM look like Opus |
| Honest grade | Opus 4.8 = **reference line**; hire on lift + taste + clarity |
| Model | `zai-org/GLM-5.2` via `unsloth/GLM-5.2-GGUF` **`UD-IQ2_M`** only |
| Repo (public) | `https://github.com/CueCard-AI/small-model-big-harness` |
| Monorepo source | `hiring/small-model-big-harness/` at **repo root** (shadow: `script/publish-smbh`). Not under `sites/labs/` — see [ops/14 §H3](../ops/14-monorepo-layout.md#h3). |
| Intake | `hello@cuelabs.cloud` |

**Forbidden at runtime:** Claude / GPT / Gemini / larger GLM; frontier teacher
in the loop; weight updates / LoRA; dropping hard tasks to inflate scores.

**Allowed:** planners, tools, verify loops, retrieval, scaffolds, self-consistency
under the **same** locked model, compilers/linters/browsers as oracles.

Do **not** say “must match Opus” on the page or in the brief.

---

## Kit architecture {#kit}

Everything a candidate needs is reached from the challenge repo.

```
hiring/small-model-big-harness/          # monorepo + public shadow
  README.md                              # cracked-bar brief
  MODEL.md                               # pin + serve / hosted endpoint
  baseline/                              # dumb ReAct (compare lift)
  scripts/
    score.py                             # score --track <id>
    bootstrap_track.py                   # fetch/verify pinned pack if not vendored
    verify_task.py
  tracks/
    smoke/                               # eval-v1 micro tasks (today’s 6 under tasks/)
    swe-lite/                            # SWE-bench Lite pack (pinned)
    swe-verified/                        # SWE-bench Verified subset (pinned)
    swe-multimodal/                      # Multimodal pack (pinned)
    swe-rebench/                         # live/fresh pack when ready
    terminal/                            # Terminal-Bench (or cousin) subset
    webarena/                            # WebArena subset
    visual-webarena/                     # VisualWebArena subset
  packs/                                 # optional large binaries / git-lfs
  results/
    OFFICIAL.md                          # per-track baseline + Opus refs
  PROVENANCE.md                          # licenses, upstream SHAs, how packs were cut
```

**Layout:** smoke aliases `tasks/` via `tracks/smoke/`; SWE packs under
`tracks/swe-lite/` and `tracks/swe-verified/` (agent JSONL + oracle digests).

### Fixture policy (locked)

1. **No scavenger hunt.** Candidates do not “go find SWE-bench.” Packs are
   vendored **or** pulled by `bootstrap_track.py` to a **pinned digest**.
2. **Freeze.** Each track has `track.lock.json` (instance IDs, upstream commit,
   content hash). Mutating tasks bumps `eval-vN`.
3. **Provenance.** Upstream license + attribution in `PROVENANCE.md`.
4. **Comparable scoring.** Same model, budgets, and lock file for a track.

---

## Tracks {#tracks}

| Track ID | Upstream | Role in kit | Hiring expectation |
|----------|----------|-------------|--------------------|
| `smoke` | First-party (eval-v1) | Setup / CI | Optional; proves tooling |
| `swe-lite` | [SWE-bench Lite](https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite) | On-ramp repo agents | Optional |
| `swe-verified` | [SWE-bench Verified](https://www.swebench.com/verified) (curated subset) | **Primary hard coding** | Strong default stab |
| `swe-multimodal` | SWE-bench Multimodal | Vision + issues | Optional / stretch |
| `swe-rebench` | SWE-rebench / live variants | Freshness | Optional when pack ready |
| `terminal` | Terminal-Bench (or equivalent) | Shell + tool ops | Optional / stretch |
| `webarena` | WebArena | Browser agents | Optional web-operator track |
| `visual-webarena` | VisualWebArena | Browser + vision | Optional / stretch |

**Open-ended rule:** any non-empty subset is a valid submission. A serious stab
at `swe-verified` (or `swe-lite` → Verified) plus whatever else sharpens the
harness thesis is the expected energy.

**Arena note:** LMArena Frontend/WebDev Elo is **not** a track. Frontend signal
comes from WebArena-style environments or a future first-party UI suite with
deterministic checks — not human pairwise votes.

---

## Budgets (default per task; track may override) {#budgets}

| Budget | Default |
|--------|---------|
| Tool calls | 40 (raise for SWE/WebArena in track lock if needed) |
| Wall clock | 600s (track lock may set higher for full repos) |
| Max output tokens / call | 4096 |

Track locks document overrides. Candidates must report the lock ID they scored.

---

## Metrics {#metrics}

| Metric | Definition |
|--------|------------|
| `pass@1` | Fraction of instances solved in one harness run (per track) |
| `lift` | `pass@1_yours − pass@1_baseline` on the **same** track lock |
| `steps` | Mean tool calls on successes (secondary) |
| `ref_gap` | Gap to Opus reference on that track (reported only) |

Multi-track submissions report a **table**, not a single blended score.

---

## Grading rubric {#rubric}

| Dimension | Weight | Cracked bar looks like |
|-----------|--------|-------------------------|
| Lift vs baseline | 30% | Real, reproducible gain on a hard track |
| Harness design | 30% | Tools, verify, context strategy for a weak model |
| Ambition / depth | 15% | Hard environment, not only smoke tasks |
| Failure analysis | 15% | Honest map of where 2-bit still dies |
| Ship quality | 10% | One-command score, clean writeup, no magic |

**Strong plus:** same harness transfers across tracks.  
**Auto-reject:** frontier teacher; trained weights; non-reproducible scores;
eval cheating; vibe-only writeup with no numbers.

---

## Deliverable {#deliverable}

1. Harness (fork / patch / separate repo) + how to score against this kit.
2. Results table: track → baseline vs yours (`pass@1`, lift, steps if available).
3. Writeup (clarity > length): design, what moved the needle, remaining failure modes.
4. Reproduce notes: hardware or hosted endpoint, GGUF revision, wall time,
   track lock IDs.

Submit: `hello@cuelabs.cloud` and/or PR against the public repo.

---

## Compute {#compute}

| Phase | Candidate access |
|-------|------------------|
| Preferred | **Hosted** edge `https://smbh.cuelabs.cloud/v1` — quant locked server-side (`hosted/`) |
| Optional | BYO llama.cpp of pinned `UD-IQ2_M` (~256GB-class) |

Hosted edge software ships in-kit. Production cutover = Studio upstream + DNS +
tokens. Until `/smbh/lock` shows `mock_upstream: false`, treat BYO as fallback.

---

## Public surfaces {#surfaces}

### `/hiring`

Sparse door. Cracked-bar one-liner + link to brief + mailto.

### `/hiring/challenge`

Full brief: intent, model links, track list, start-here clone, submit.
Primary CTA: **Challenge repo →**. Wide column (`shell--wide`) OK.

### Public GitHub

Fixtures/packs after shadow publish. Monorepo:
`hiring/small-model-big-harness/` → `script/publish-smbh` /
`.github/workflows/publish_smbh.yml` (secret `SMBH_PUBLISH_TOKEN`).

---

## Phased delivery {#phases}

### M0 — Positioning + smoke kit

- [x] Public repo + shadow publish wiring
- [x] On-site brief at `/hiring/challenge`
- [x] Smoke track (eval-v1, six tasks)
- [x] Land this open-ended multi-track cracked-bar spec
- [x] Rewrite public README + `/hiring` copy to match cracked-bar language
- [x] Document track table on the brief (packs may still say “landing”)

### M1 — SWE pack in-kit

- [x] Curate + vendor (or pin-bootstrap) `swe-lite` and/or `swe-verified` subset
- [x] `score.py --track swe-verified` (or lite)
- [x] Freeze locks + `PROVENANCE.md`
- [x] Publish baseline row (mock + real GLM when possible)
  - Mock dry-runs: smoke 0/6, swe-lite 0/10, swe-verified 0/15
  - Real GLM + Docker resolved@1 still TBD (needs GPU / Opus runs)
- [x] Official Docker + swebench grading path (`scripts/grade_swe.py`, `--grade docker`)

### M2 — Hosted model

- [x] Locked 2-bit endpoint for candidates
  - Edge: `hiring/small-model-big-harness/hosted/` (auth, force `glm-5.2-iq2`, RPM/concurrency, `/smbh/lock`)
  - Public URL contract: `https://smbh.cuelabs.cloud/v1`
  - Ops cutover: private llama.cpp `UD-IQ2_M` upstream + DNS + minted tokens (not in-repo GPU)
- [x] Update `MODEL.md` + brief (BYO becomes optional)

### M3 — Terminal + Web tracks

- [x] `terminal` pack (8 first-party shell fixtures, `terminal-v1`, local pytest)
- [x] `webarena` + `visual-webarena` packs (10 + 8 pinned intents)
- [x] Per-track OFFICIAL baselines (mock dry-runs; real GLM / env grades TBD)

### M4 — Multimodal + rebench + polish

- [x] `swe-multimodal` (10, Multimodal **dev**) + `swe-rebench` (10, leaderboard subset)
- [x] Optional anonymous leaderboard (`results/LEADERBOARD.md` + `submit_leaderboard.py`)
- [x] Optional CueCode feel-test (`feel-test/README.md` + brief link; not graded)

**Phases M0–M4 complete.** Further work = ops cutover (hosted Studio), real GLM/Opus OFFICIAL rows, upstream env grades.

---

## Acceptance {#acceptance}

Open-ended kit is “real” when:

1. Brief + README state cracked bar + open-ended multi-track clearly.
2. At least **smoke + one SWE track** are runnable from the repo without
   hunting upstream yourself.
3. Baseline numbers exist for those tracks (even if Opus ref still TBD).
4. Shadow publish keeps public GitHub in sync with `hiring/small-model-big-harness/`.

Full “all tracks available in-kit” = M3+ with hosted model (M2).

---

## Open questions {#open-questions}

1. SWE start: Lite-only vs jump straight to Verified subset size (15 vs 30)?
2. WebArena subset size / which sites to pin?
3. Hosted endpoint SLA / abuse limits for candidates? (defaults: 30 RPM, 2 concurrent — tunable in `hosted/.env`)
4. Any bounty for top lifts, or hiring-only?

---

## Changelog

| Date | Change |
|------|--------|
| 2026-07-20 | Placement lock: kit stays at root `hiring/` (monorepo layout H3) |
| 2026-07-20 | M4: swe-multimodal + swe-rebench; anonymous LEADERBOARD; CueCode feel-test |
| 2026-07-20 | M3: terminal (8) + webarena (10) + visual-webarena (8); mock OFFICIAL rows |
| 2026-07-20 | M2: hosted edge (`hosted/`) + MODEL/brief prefer token endpoint; BYO optional |
| 2026-07-20 | Wire Docker + swebench grading (`grade_swe.py`, `--grade docker`, requirements-swe.txt) |
| 2026-07-20 | M1: swe-lite (10) + swe-verified (15) packs; `--track` scoring; PROVENANCE; mock baselines |
| 2026-07-20 | M0 complete: cracked-bar copy on brief + README; track table documented; phases M0–M4 |
| 2026-07-20 | Open-ended multi-track cracked-bar kit (fixtures + benches in-repo); phases M0–M4 |
| 2026-07-19 | Wire `script/publish-smbh` + `publish_smbh.yml`; path `hiring/small-model-big-harness/` |
| 2026-07-19 | Public repo + on-site brief + eval-v1 smoke |
| 2026-07-19 | Initial end-to-end hiring challenge spec (H0) |
