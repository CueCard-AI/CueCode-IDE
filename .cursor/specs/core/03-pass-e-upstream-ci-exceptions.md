# Pass E — Upstream scripts & CI exception list {#pass-e-upstream-ci-exceptions}

Follow-on from [03-zed-reference-cleanup-phases → Pass E progress](./03-zed-reference-cleanup-phases#progress-pass-e).

**Policy:** Do **not** mass-rename upstream Zed GitHub workflows, release scripts, or `xtask` workflow
generators unless CueCode forks the entire release/CI pipeline. Upstream files stay as-is for merge
hygiene; they are **inert on the CueInference fork** because jobs guard on `zed-industries` / `zed-extensions`.

**CueCode enforcement surface** (what actually runs on this repo):

| Layer | Path | Role |
|-------|------|------|
| CI | [`.github/workflows/cuecode_rebrand_check.yml`](../../../.github/workflows/cuecode_rebrand_check.yml) (repo root) | `rebrand-check.sh`, `cuecode --help`, Ollama agent smoke, network idle audit |
| CI | [`.github/workflows/publish_cuecode_ide.yml`](../../../.github/workflows/publish_cuecode_ide.yml) | Public shadow publish |
| CI | `apps/CueCode-IDE/.github/workflows/cuecode-cloud-m0.yml` | CHP / `cuecode_cloud` crate tests |
| Local | `script/rebrand-check.sh`, `script/qa-p0.sh`, `script/rebrand-progress.sh` | Phase 0 + Pass A–D gates |

Re-audit upstream references (informational):

```bash
cd apps/CueCode-IDE
rg 'zed-industries|zed\.dev|cloud\.zed' .github/workflows script/ tooling/xtask --glob '!**/rebrand-*'
```

---

## 1. GitHub Actions audit (`apps/CueCode-IDE/.github/workflows/`)

**42 upstream workflow files** ship with the Zed tree. On **CueCard-AI/CueInference** they do not run
meaningful jobs: almost all include `if: github.repository_owner == 'zed-industries'` (or
`github.repository == 'zed-industries/zed'`).

### 1.1 Keep — CueCode-only (active)

| Workflow | Location | Notes |
|----------|----------|-------|
| `cuecode_rebrand_check.yml` | **Repo root** `.github/workflows/` | Primary rebrand regression CI |
| `publish_cuecode_ide.yml` | **Repo root** | Shadow publish; not product release |
| `cuecode-cloud-m0.yml` | `apps/CueCode-IDE/.github/workflows/` | Cloud M0 crate gate |

**Action taken:** No change to upstream YAML bulk. Root workflows are the fork’s enforcement surface.

### 1.2 Leave inert — upstream release & build (defer full fork)

Guarded; safe to keep for upstream merges. **Fork or replace only when CueCode ships releases.**

| Workflow | Upstream purpose | Zed URLs / refs | CueCode fork trigger |
|----------|------------------|-----------------|----------------------|
| `release.yml` | Stable/preview release | `gh release … --repo=zed-industries/zed`, Slack | CueCode release pipeline |
| `release_nightly.yml` | Nightly channel | Same org guards | Optional nightly for CueCode |
| `after_release.yml` | Post-release hooks | `cloud.zed.dev/releases/refresh`, `zed.dev/api/revalidate`, Discord | See [§4 after_release](#4-toolingxtask--after_release) |
| `run_bundling.yml` | PR label bundling | zed-industries guards | CueCode PR preview builds |
| `run_tests.yml` | Main CI matrix | `ci@zed.dev` git identity | Separate slim CueCode test workflow if needed |
| `deploy_docs.yml` | docs.zed.dev deploy | `zed-industries` owner | `cuecode.dev/docs` deploy |
| `deploy_nightly_docs.yml` | Nightly docs | Reuses upstream `deploy_docs` call | Same |
| `deploy_collab.yml` | Collab staging/prod | collab deploy secrets | CueCode cloud only |
| `nix_build.yml` | Nix bundle | Label-gated | If Nix packaging for CueCode |
| `bump_zed_version.yml` | Version bump bot | zed-industries | `bump_cuecode_version.yml` (new) |
| `bump_patch_version.yml` | Patch releases | zed-industries | New workflow |
| `bump_collab_staging.yml` | Collab staging bump | zed-industries | CueCode cloud staging |
| `publish_extension_cli.yml` | Extension CLI → extensions repo | `zed-industries/extensions` | Only if CueCode extension registry |
| `extension_bump.yml` | Extension version sync | extensions org | Defer |
| `extension_auto_bump.yml` | Auto bump | extensions org | Defer |
| `extension_tests.yml` | Extension CI | extensions org | Defer |
| `extension_workflow_rollout.yml` | Rollout automation | zed-industries/zed@sha | Defer |
| `compliance_check.yml` | License/compliance | zed PR links | Optional CueCode compliance job |
| `danger.yml` | PR danger | `danger-proxy.zed.dev` | Defer or drop on fork |
| `autofix_pr.yml` | Autofix bot | zed-industries | Defer |
| `cherry_pick.yml` | Release cherry-pick | zed-industries | Defer |
| `docs_suggestions.yml` | Docs bot | zed-industries | Defer |

### 1.3 Leave inert — upstream community / triage (never fork)

These exist only for Zed Industries GitHub org operations. **Do not rebrand or run on CueInference.**

| Workflow | Purpose |
|----------|---------|
| `triage_project_sync.yml` | Org project #84 sync |
| `pr_issue_labeler.yml` | Issue/PR labeling |
| `track_duplicate_bot_effectiveness.yml` | Duplicate bot metrics |
| `update_duplicate_magnets.yml` | Duplicate issue magnets |
| `comment_on_potential_duplicate_issues.yml` | Duplicate comments |
| `catch_blank_issues.yml` | Blank issue handler |
| `add_commented_closed_issue_to_project.yml` | Project automation |
| `community_pr_board.yml` / `community_pr_board_refresh.yml` | Community PR board |
| `community_close_stale_issues.yml` | Stale issue closer |
| `community_update_*_top_ranking_issues.yml` | Top issues ranking |
| `stale-pr-reminder.yml` | Stale PR Slack |
| `slack_notify_*.yml` | Slack integrations |
| `hotfix-review-monitor.yml` | Hotfix monitoring |
| `good_first_issue_notifier.yml` | GFI notifications |
| `congrats.yml` | Contributor congrats |

**Exception rationale:** Renaming these breaks upstream merges for zero benefit — guards already skip them.

---

## 2. Repo root vs `apps/CueCode-IDE/` CI layout

```
CueInference/                          ← fork home
├── .github/workflows/
│   ├── cuecode_rebrand_check.yml      ← ENFORCE (Pass 0 + agent + network)
│   └── publish_cuecode_ide.yml        ← ENFORCE (shadow publish)
└── apps/CueCode-IDE/
    └── .github/workflows/
        ├── cuecode-cloud-m0.yml       ← ENFORCE (cloud M0)
        └── … 41 upstream workflows   ← INERT (owner guards)
```

**Do not** move upstream workflows out of `apps/CueCode-IDE/` without an explicit CI fork project — that
creates painful merge conflicts. **Do** add new CueCode workflows at repo root or beside
`cuecode-cloud-m0.yml`.

---

## 3. `script/` release helpers — touch only when needed

### 3.1 CueCode enforcement (use today)

| Script | Role |
|--------|------|
| `rebrand-check.sh` | Automated Pass A–D gates |
| `qa-p0.sh` | Phase 0 manual QA harness |
| `rebrand-progress.sh` | Progress reporter |
| `network-idle-audit.sh` | No `zed.dev` idle connections |
| `qa-agent-ollama.sh` | Agent + Ollama smoke |
| `rebrand-docs-pass-d.py`, `rebrand-docs-pass-d2.py` | Docs rebrand batch (historical) |

### 3.2 Deferred — repoint when CueCode release pipeline exists

| Script | Current upstream behavior | CueCode action |
|--------|---------------------------|----------------|
| `install.sh` | Downloads from `cloud.zed.dev`, asset `zed` | Repoint to `cuecode.dev` / asset `cuecode`; or ship `install-cuecode.sh` |
| `get-released-version` | `cloud.zed.dev/.../asset?asset=zed` | Query CueCode release API |
| `trigger-release` | Links to `zed-industries/zed` Actions | New script → CueCard-AI/CueCode-IDE |
| `trigger-docs-build` | Upstream docs workflow URL | CueCode docs deploy trigger |
| `draft-release-notes` | GitHub compare on `zed-industries/zed` | Fork repo slug + compare URLs |
| `get-release-notes-since` | GraphQL owner `zed-industries` | CueCard-AI org |
| `get-pull-requests-since` | zed-industries/zed PR URLs | CueCard-AI/CueCode-IDE |
| `bump-zed-version` | Triggers upstream bump workflow | New `bump-cuecode-version` |
| `bump-extension-cli` | extensions org | Defer unless CueCode extensions |
| `lib/deploy-helpers.sh` | `collab.zed.dev` URLs | CueCode collab host when live |
| `flatpak/convert-release-notes.py` | zed-industries release tags | CueCode tag URLs |
| `bundle-mac` | Uses `zed-industries/cargo-bundle` git dep | **Allowed exception** — upstream build tool fork until CueCode hosts own |
| `bundle-linux`, `bundle-windows.ps1` | Product naming already CueCode | Use locally; wire into CueCode `release.yml` when ready |

### 3.3 Never needed on CueInference fork (upstream ops only)

| Script | Reason |
|--------|--------|
| `github-*.py`, `github-pr-status` | Zed triage/community automation |
| `triage_project_sync.py`, `triage_watcher.jl` | Org project sync |
| `update_top_ranking_issues/` | Community ranking bot |
| `test-docs-suggest-batch`, `docs-suggest` | Upstream docs suggestion bot |
| `github-track-duplicate-bot-effectiveness.py` | Duplicate bot metrics |
| `community-pr-track-mapping.json` | Zed area labels incl. `area:zed.dev` |

### 3.4 Legal / terms (not Pass E rebrand)

| Path | Note |
|------|------|
| `script/terms/terms.rtf` | Upstream Zed Industries ToS template — CueCode legal is separate ([Pass D legal](../core/03-zed-reference-cleanup-phases#pass-d)) |

---

## 4. `tooling/xtask` / `after_release` {#4-toolingxtask--after_release}

Workflow YAML under `.github/workflows/` is **generated** from `tooling/xtask`. Upstream generators
hard-code Zed infra:

| Source | Upstream URL / ref | Defer or fork? |
|--------|-------------------|----------------|
| `after_release.rs` | `POST cloud.zed.dev/releases/refresh` | **Fork** → `cuecode.dev` refresh when releases live |
| `after_release.rs` | `GET zed.dev/api/revalidate?tag=releases` | **Fork** → CueCode CDN revalidate token |
| `after_release.rs` | Discord posts to `zed.dev/releases/{stable\|preview}` | **Fork** → `cuecode.dev/releases/...` |
| `after_release.rs` | Reuses `deploy_docs` workflow call | **Fork** → CueCode docs deploy |
| `release.rs` | `gh release … zed-industries/zed` | **Fork** on CueCode release |
| `deploy_docs.rs` | `github.repository_owner == 'zed-industries'` | **Fork** |
| `danger.rs` | `danger-proxy.zed.dev` | **Defer** |
| `publish_extension_cli.rs`, `extension_bump.rs` | `zed-industries/extensions` | **Defer** |
| `run_tests.rs` | `ci@zed.dev` committer | Cosmetic; change when forking CI |
| `setup_webrtc.rs` | `zed-industries/livekit-rust-sdks` | **Keep** — upstream WebRTC dep fork (not product branding) |
| `vars.rs` | Comment: asset names shared with zed.dev codebase | **Keep** until release API diverges |

**Recommended sequence when CueCode releases ship:**

1. Add **`cuecode_after_release.yml`** at repo root (hand-written or xtask fork) — do not edit generated upstream YAML in place first.
2. Fork `tooling/xtask/src/tasks/workflows/after_release.rs` → `cuecode_after_release.rs` with CueCode URLs.
3. Point `trigger-release` / release tagging at **CueCard-AI/CueCode-IDE**.
4. Leave upstream `after_release.yml` in tree (inert on fork) until merge strategy says otherwise.

**Do not run** `cargo xtask workflows` regen on the fork unless intentionally syncing upstream CI — it will rewrite Zed workflows.

---

## 5. Explicit exceptions (allowed Zed references in CI/scripts)

| Reference | Where | Why kept |
|-----------|-------|----------|
| `zed-industries` org in workflow `if:` guards | All upstream workflows | Makes jobs no-op on fork; upstream merge compat |
| `zed-industries/cargo-bundle` git URL | `script/bundle-mac` | Build tool fork; replace when CueCode hosts cargo-bundle |
| `zed-industries/livekit-rust-sdks` | `tooling/xtask/setup_webrtc.rs` | WebRTC binary dep, not user-facing |
| `ZED_*` env vars in scripts | `install.sh`, bundle scripts, qa | Ecosystem compat per [#do-not-fix](./03-zed-reference-cleanup-phases#do-not-fix) |
| `ci@zed.dev` in generated CI | `run_tests.rs` / `run_tests.yml` | Inert on fork; fix when forking test workflow |
| `danger-proxy.zed.dev` | `danger.yml` | Inert on fork |
| Upstream workflow **filenames** (`bump_zed_version.yml`, etc.) | `.github/workflows/` | Rename only during full CI fork |

---

## 6. Exit checklist (Pass E complete)

- [x] Exception list documented (this file)
- [x] `.github/workflows/` audited — CueCode enforcement = root `cuecode_rebrand_check.yml` +
  `publish_cuecode_ide.yml` + `cuecode-cloud-m0.yml`; upstream jobs inert via owner guards
- [x] `script/` release helpers catalogued — touch list in [§3.2](#32-deferred--repoint-when-cuecode-release-pipeline-exists)
- [x] `tooling/xtask` / `after_release` — defer full fork; repoint plan in [§4](#4-toolingxtask--after_release)

**Not required for Pass E exit:** Deleting upstream workflows, repointing `install.sh`, or forking `after_release.rs` — those belong to **CueCode release infrastructure** (post–Phase 0 product work).

---

## 7. Future: CueCode release CI (out of Pass E scope)

When ready, add (new files, prefer repo root):

| Deliverable | Suggested path |
|-------------|----------------|
| CueCode stable release workflow | `.github/workflows/cuecode_release.yml` |
| Post-release refresh | `.github/workflows/cuecode_after_release.yml` |
| Install script | `script/install-cuecode.sh` or repoint `install.sh` |
| Version bump | `script/bump-cuecode-version` |

Cross-link: [Pass C auto-update E2E](./03-pass-c-auto-update-smoke-test.md) for update server + smoke after release CI exists.

---

## Changelog

| Date | Change |
|------|--------|
| 2026-06-20 | Initial Pass E audit + exception list |
