# Zed Reference Cleanup — Phased Passes {#zed-reference-cleanup}

Follow-on work after [Phase 0 rebrand](./03-fork-and-rebrand) tiers 1–4 and script
gates. Phase 0 made CueCode **shippable**; this doc tracks **remaining Zed
references** and how to burn them down without breaking upstream compatibility.

**Related:**

- Rebrand checklist: [03-fork-and-rebrand](./03-fork-and-rebrand#checklist)
- **Progress tracker:** [#progress](#progress) — check off passes here
- Roadmap: [07-implementation-roadmap](../delivery/07-implementation-roadmap#phase-0)
- Verification: `CueCode-IDE/script/rebrand-check.sh`, `CueCode-IDE/script/qa-p0.sh`, `CueCode-IDE/script/rebrand-progress.sh`, `CueCode-IDE/script/network-idle-audit.sh`
- CI: `.github/workflows/cuecode_rebrand_check.yml`
- Agent skill: [rebrand-progress](../../skills/rebrand-progress/SKILL.md)

---

## Two different “tier” maps {#two-tier-maps}

Do not confuse these:

| Map | Where | Meaning |
|-----|-------|---------|
| **Rebrand tiers 1–5** | [03-fork-and-rebrand](./03-fork-and-rebrand#checklist) | Product identity, cloud decouple, packaging, deferred renames |
| **Script tiers 1 / 2 / 2b / 4** | `rebrand-check.sh` | Automated grep gates on specific crates and packaging files |
| **Cleanup passes A–E** | This doc | Optional post–Phase 0 sweeps for stragglers (~800+ files still mention Zed) |

**Phase 0 exit** = rebrand tiers 1–4 done + script gates green + `qa-p0.sh` pass.
**Cleanup passes** = polish and debt; not blockers for Phase 1.1 unless user-visible
strings slip through.

---

## Status (summary) {#status}

High-level rollup only — use [#progress](#progress) to check off individual tasks.

| Area | State |
|------|-------|
| Phase 0 identity + cloud decouple | Done (gated + qa-p0) |
| Cleanup Pass A — settings/keymap comments | Done |
| Cleanup Pass B — user-visible stragglers | Done |
| Cleanup Pass C — artifact renames | Done (auto-update E2E deferred) |
| Cleanup Pass D — docs/legal/README | Done |
| Cleanup Pass E — upstream CI/scripts | Done |

Rough scale: ~846 files under `CueCode-IDE` still match `Zed` / `zed` / `zed.dev`; most
are comments, compat aliases, crate names, or upstream citations — not user-facing product copy.

---

## Progress {#progress}

**Last verified:** 2026-06-20 (`./script/rebrand-progress.sh --full` → rebrand-check + qa-p0 green)

**Legend**

| Marker | Meaning |
|--------|---------|
| `[x]` | Complete |
| `[ ]` | Not done |
| *(gate)* | Verified by `rebrand-check.sh` — re-run `./script/rebrand-progress.sh` after changes |
| *(manual)* | Human verification only — gates do not cover this |

**Workflow:** finish work on a pass → run `./script/rebrand-progress.sh --full` → update
checkboxes here → sync [Phase 0 roadmap](../delivery/07-implementation-roadmap#progress)
exit items if applicable. See skill: [rebrand-progress](../../skills/rebrand-progress/SKILL.md).

### Phase 0 — automated gates

- [x] `rebrand-check.sh` all sections green *(gate)*
- [x] `qa-p0.sh` 20/20 *(gate, run with `--full`)*
- [x] Tier 1–4 packaging + UI string gates *(gate)*
- [x] L3.1–L3.4 crate renames + `cuecode://` scheme *(gate)*

### Phase 0 — manual / partial

- [x] `cargo run` / `cuecode` binary launches *(qa-p0)*
- [x] Title bar shows **CueCode** *(qa-p0)*
- [x] Config isolation under `~/.config/cuecode/` *(qa-p0 isolated profile)*
- [ ] Dock/taskbar icon fully custom (not upstream Zed artwork) *(manual)*
- [x] Agent prompt to local model via `eval-cli` + Ollama — `script/qa-agent-ollama.sh` *(gate locally; CI: `agent-ollama-linux` job)*
- [x] Idle app does not phone `zed.dev` — `script/network-idle-audit.sh` *(gate locally; CI: `network-idle-linux`; optional: `QA_NETWORK_IDLE=1 ./script/qa-p0.sh`)*

### Pass A — Settings and keymap comments {#progress-pass-a}

- [x] `assets/settings/default.json` comment sweep *(gate)*
- [x] `assets/keymaps/*.json` comment sweep *(gate)*
- [x] Pass A gates in `rebrand-check.sh` *(gate)*

**Exit:** [#pass-a](#pass-a)

### Pass B — Remaining user-visible strings {#progress-pass-b}

- [x] Collab / explorer / extension CLI / sandbox / editor surfaces *(gate)*
- [x] Component previews and agent templates *(gate)*
- [x] Auto-update error contexts (no `Zed.exe`) *(gate)*
- [x] Expanded `TIER1_UI_CRATES` / Pass B surfaces in `rebrand-check.sh` *(gate)*

**Exit:** [#pass-b](#pass-b)

### Pass C — Filename and artifact renames {#progress-pass-c}

- [x] Linux `cuecode{}.app` + `libexec/cuecode-editor` in auto-update *(gate)*
- [x] Release assets `cuecode` / `cuecode-remote-server` *(gate)*
- [x] Bundle scripts (`bundle-linux`, mac, windows) *(gate)*
- [x] Windows updater `bin/cuecode` *(gate)*
- [ ] Auto-update install smoke test on macOS + Linux *(manual — `poll_for_updates()` off)* — see **[Pass C auto-update E2E](./03-pass-c-auto-update-smoke-test.md)**

**Exit:** [#pass-c](#pass-c)

### Pass D — Docs and legal {#progress-pass-d}

- [x] `docs/` product copy rebrand (~196 files) *(gate + audit)*
- [x] `configuring-cuecode.md` + SUMMARY + `book.toml` redirect *(gate)*
- [x] `legal/` fork notices (preserve Zed Industries entity) *(gate)*
- [x] Root `README.md`, `CONTRIBUTING.md` sweep *(manual)*
- [x] Pass D gates in `rebrand-check.sh` *(gate)*
- [ ] Remaining `zed.dev` / `images.zed.dev` CDN URLs in docs *(optional polish)* — see **[Pass D CDN backlog](./03-pass-d-docs-cdn-backlog.md)** (10 app screenshots + 5 theme assets + URL/copy sweep)

**Exit:** [#pass-d](#pass-d) — `rg '\bZed\b' docs/` → 1 intentional upstream hit only

### Pass E — Upstream scripts and CI {#progress-pass-e}

- [x] Document exception list in [#pass-e](#pass-e) — **[Pass E exception list](./03-pass-e-upstream-ci-exceptions.md)**
- [x] Audit `.github/workflows/` — CueCode enforcement: root `cuecode_rebrand_check.yml`, `publish_cuecode_ide.yml`, `cuecode-cloud-m0.yml`; 41 upstream workflows inert via owner guards *(see exception doc §1)*
- [x] Release helpers in `script/` — catalogued; touch only when CueCode pipeline needs them *(exception doc §3)*
- [x] `tooling/xtask` / `after_release` upstream URLs — defer; fork plan when CueCode releases ship *(exception doc §4)*

**Exit:** [#pass-e](#pass-e)

---

## Do not fix (compat) {#do-not-fix}

Keep these unless there is an explicit L3/L4 decision in
[rename depth](./03-fork-and-rebrand#rename-depth):

| Pattern | Reason |
|---------|--------|
| `zed://` URL scheme | Deep link / extension compat |
| `.zed/` project paths | Upstream project layout |
| `ZED_*` env vars | Tooling and extension ecosystem |
| WIT `zed:extension` | Extension API namespace |
| Serde alias `zed.dev` on settings keys | Settings migration |
| GPL About line citing Zed upstream | License requirement |
| Legacy icon theme alias `"Zed (Default)"` in registry | Existing user settings |
| Crate/dir names `crates/zed`, `zed_actions`, etc. | Tier 5 / L3 — separate effort |

---

## Cleanup passes {#passes}

Run passes in order. Each pass should end with `rebrand-check.sh` + `qa-p0.sh`
unchanged or stricter — never weaker gates without review.

### Pass A — Settings and keymap comments {#pass-a}

**Goal:** No product-name “Zed” in shipped defaults users might read in Settings UI.

| Scope | Examples |
|-------|----------|
| `assets/settings/default.json` | Comment strings (~56) |
| `assets/keymaps/*.json` | Comment / description fields |
| `settings_content` schema doc strings | Already gated as Tier 2b — extend if needed |

**Effort:** ~1 day  
**Exit:** Grep `default.json` + keymaps for `\bZed\b` returns zero **or** only
entries on the [#do-not-fix](#do-not-fix) list.

### Pass B — Remaining user-visible strings {#pass-b}

**Goal:** Catch dialogs, errors, explorer labels, and component previews not in
current Tier 1 crate list.

| Scope | Examples |
|-------|----------|
| Collab / call error strings | Status toasts, connection failures |
| Explorer / open-with | “Open with Zed” |
| Extension CLI User-Agent | Product name in HTTP headers |
| Component previews / story fixtures | Dev-only but visible in UI previews |
| Auto-update error contexts | Remaining `Zed.exe` strings in `auto_update.rs` |

**Effort:** ~2–3 days  
**Exit:** Expand `TIER1_UI_CRATES` / `TIER2_STRING_CRATES` in `rebrand-check.sh`
for any new surfaces; all gates green.

### Pass C — Filename and artifact renames (optional) {#pass-c}

**Goal:** Align on-disk names with CueCode where safe.

| Scope | Examples |
|-------|----------|
| Linux bundle | `zed{}.app` → CueCode naming in auto-update install path |
| `libexec/zed-editor` | Linux helper binary path |
| Release artifact names | Already partially done in Phase 0.3 |

**Effort:** 1–2 days + platform QA  
**Exit:** Install/update smoke test on macOS + Linux when auto-update is re-enabled — playbook:
**[03-pass-c-auto-update-smoke-test.md](./03-pass-c-auto-update-smoke-test.md)**  
**Note:** `poll_for_updates()` is currently off — full E2E belongs here, not Pass B.

### Pass D — Docs and legal {#pass-d}

**Goal:** Public-facing repo docs match product; legal stays accurate.

| Scope | Action |
|-------|--------|
| `CueCode-IDE/docs/` (~196 files) | Replace product copy; keep historical/upstream citations where needed |
| `legal/`, `LICENSE` pointers | CueCode distribution name; preserve GPL lineage |
| Root `README.md`, `CONTRIBUTING.md` | Already partially updated — sweep remaining Zed product refs |

**Effort:** Multi-day; can parallelize by directory  
**Exit:** `rg '\bZed\b' docs/` returns only intentional upstream references (changelog,
attribution).

### Pass E — Upstream scripts and CI (defer) {#pass-e}

**Goal:** Do **not** mass-rename upstream Zed workflow files unless we fork CI entirely.

| Scope | Action |
|-------|--------|
| `.github/workflows/` upstream jobs | Leave inert (owner guards); CueCode jobs at repo root + `cuecode-cloud-m0.yml` |
| `script/` release helpers | Touch only when CueCode release pipeline needs them |
| `tooling/xtask` / `after_release` | Defer URL fork until CueCode releases; see repoint plan |

**Full audit:** **[03-pass-e-upstream-ci-exceptions.md](./03-pass-e-upstream-ci-exceptions.md)**

**Exit:** Documented exception list; CueCode-specific workflows
(`cuecode_rebrand_check.yml`, `publish_cuecode_ide.yml`, `qa-p0.sh`) remain the enforcement surface.

---

## Verification {#verification}

After any pass:

```bash
cd CueCode-IDE
./script/rebrand-check.sh
./script/qa-p0.sh
cargo run -p cuecode -- --help   # binary name smoke check
```

Optional audit (informational, not a gate):

```bash
cd CueCode-IDE
rg -l '\bZed\b' --glob '*.rs' crates/ | wc -l
rg '\bZed\b' assets/settings/default.json
```

---

## When to stop {#when-to-stop}

Stop burning down references when:

1. Phase 0 [exit criteria](../delivery/07-implementation-roadmap#phase-0-exit) hold.
2. No user-visible “Zed” in gated surfaces (`rebrand-check.sh`).
3. Remaining hits are [#do-not-fix](#do-not-fix) or Pass D/E backlog.

Then proceed to **[Build phase 1.1](./build-plans/phases/1-1-cuecode-specs.md)** (`cuecode_specs` crate) per
[build-plans README](./build-plans/README.md#phase-index).

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Pass E upstream CI/scripts exception list (`03-pass-e-upstream-ci-exceptions.md`) |
| 2026-06-20 | Pass D CDN backlog doc (`03-pass-d-docs-cdn-backlog.md`) — 10 screenshots + rehost inventory |
| 2026-06-20 | Pass C auto-update E2E playbook (`03-pass-c-auto-update-smoke-test.md`) |
| 2026-06-20 | Network idle audit: `network-idle-audit.sh`, CI `network-idle-linux`, optional qa-p0 Step 9 |
| 2026-06-20 | Agent Ollama smoke: `qa-agent-ollama.sh`, qa-p0 Step 4b, Linux CI job, eval-cli wait_for_model fix |
| 2026-06-20 | Progress tracker (#progress), `rebrand-progress.sh`, rebrand-progress skill |
| 2026-06-17 | Pass D: docs rebrand, legal fork notices, CONTRIBUTING/README sweep, rebrand-check gates |
| 2026-06-17 | Initial phased cleanup doc after Phase 0.3 + Tier 1 string sweep |
