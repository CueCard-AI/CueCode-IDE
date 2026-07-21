# Pass D polish — docs CDN & zed.dev URL backlog {#pass-d-docs-cdn-backlog}

Follow-on from [03-zed-reference-cleanup-phases → Pass D progress](./03-zed-reference-cleanup-phases#progress-pass-d).
Pass D **exit** (product copy `\bZed\b`) is done; this tracks **optional polish**:
remove dependency on Zed CDNs and stale Zed cloud URLs in `apps/CueCode-IDE/docs/`.

**Re-audit:**

```bash
cd apps/CueCode-IDE
rg 'images\.zed\.dev|cdn\.zed\.dev' docs/
rg 'zed\.dev' docs/   # expect only intentional upstream citations when done
```

**Suggested asset layout for rehosted files:**

```
docs/src/assets/images/project-panel/   # 7 PNGs — placeholders on disk; re-capture in CueCode
docs/src/assets/images/troubleshooting/ # 3 WEBPs — re-capture with CueCode as Instruments target
docs/theme/assets/theme/noise.png       # rehosted (done)
docs/theme/fonts/*.woff2                # rehosted (done)
```

Markdown already links via `../assets/images/...` from `docs/src/*.md`. **Drop-in:** replace files in place; no URL edits needed unless you add new shots.

**Return here from:** [build-plans README §doc-screenshots-backlog](../delivery/build-plans/README.md#doc-screenshots-backlog) · [Build phase 0.3 follow-up](../delivery/build-plans/phases/0-3-packaging-qa.md#phase-0-3-follow-up)

---

## Summary

| Category | Count | Your effort |
|----------|------:|-------------|
| **App screenshots** (`images.zed.dev`) | **10** | Capture in **CueCode** (7) + **Instruments/macOS** (3) |
| **Theme CDN** (`cdn.zed.dev`) | **5** | Download & rehost (no app screenshots) |
| **Docs cross-links** (`docs.zed.dev`) | **4** | Replace URLs → `cuecode.dev/docs/...` |
| **Zed cloud / billing copy** | **~20** | Product decision: stub, rewrite, or fork-notice |
| **Settings JSON examples** (`"provider": "zed.dev"`) | **6** | Replace with `ollama` / BYOK examples |
| **Writer conventions** | **~10** | Update guidelines to say `cuecode.dev` |
| **Misc email / firewall** | **~6** | Point to CueCode support or remove |

---

## A. App screenshots — 10 total (capture & rehost)

### A1. Project panel — 7 PNGs (`docs/src/project-panel.md`)

Capture in **CueCode** with title bar showing **CueCode**. **Replace files at:**
`docs/src/assets/images/project-panel/<name>.png` (same paths referenced in markdown).

| # | Current URL | File to create | What to show when capturing |
|---|-------------|----------------|----------------------------|
| 1 | `.../project-panel/panel.png` | `panel.png` | Default project panel open; file tree visible |
| 2 | `.../sticky-scroll-true.png` | `sticky-scroll-true.png` | Deep tree scrolled; `project_panel.sticky_scroll` **on** (pinned ancestors) |
| 3 | `.../sticky-scroll-true.png` | `sticky-scroll-false.png` | Same tree; sticky scroll **off** |
| 4 | `.../auto-fold-dirs-true.png` | `auto-fold-dirs-true.png` | `project_panel.auto_fold_dirs` **on** — collapsed path like `src/utils/helpers` |
| 5 | `.../auto-fold-dirs-false.png` | `auto-fold-dirs-false.png` | Same repo layout; auto-fold **off** (expanded levels) |
| 6 | `.../compare-marked-files.png` | `compare-marked-files.png` | Two files marked in panel; diff/compare view open |
| 7 | `.../git-status.png` | `git-status.png` | Git tinting and/or letter badges (M/A/D) visible |

**Replace in:** `docs/src/project-panel.md` lines 13, 35, 37, 47, 49, 60, 128.

### A2. Troubleshooting — 3 WEBPs (`docs/src/troubleshooting.md`)

These are **macOS Instruments**, not CueCode UI. Options:

- Re-capture the same Instruments steps while profiling **CueCode**, or  
- Keep upstream images temporarily with a fork note, or  
- Remove/simplify the Instruments section for the fork.

| # | Current URL | File to create | What to show |
|---|-------------|----------------|--------------|
| 8 | `.../instruments-template-picker.webp` | `instruments-template-picker.webp` | Instruments template picker, Time Profiler selected |
| 9 | `.../instruments-target-and-record.webp` | `instruments-target-and-record.webp` | Target = CueCode process, record button |
| 10 | `.../instruments-recording.webp` | `instruments-recording.webp` | Completed Time Profiler recording |

**Replace in:** `docs/src/troubleshooting.md` lines 49, 52, 55.

---

## B. Theme CDN — 5 assets (download & rehost, no screenshots)

| # | File | Current URL | Replace with |
|---|------|-------------|--------------|
| 1 | `docs/theme/index.hbs` (~L79) | `https://cdn.zed.dev/images/noise.png` | `docs/assets/theme/noise.png` or drop effect |
| 2 | `docs/theme/fonts/fonts.css` (~L7) | `.../fonts/iAWriterQuattroV.woff2` | local `docs/assets/theme/fonts/` |
| 3 | same (~L16) | `.../iAWriterQuattroV-Italic.woff2` | local |
| 4 | same (~L25) | `.../IBMPlexSerif-Var.woff2` | local (IBM Plex already in repo assets?) |
| 5 | same (~L34) | `.../Lilex-Regular.woff2` | local |

Verify docs site still renders after font paths change (`mdbook serve docs`).

---

## C. Docs cross-links — 4 URLs

| File | Lines | Replace |
|------|-------|---------|
| `docs/src/reference/all-settings.md` | 2815–2818 | `https://docs.zed.dev/languages/{go,rust,svelte,typescript}` → `https://cuecode.dev/docs/languages/...` |

---

## D. Settings JSON examples — replace `"provider": "zed.dev"`

| File | Lines | Replace with |
|------|-------|--------------|
| `docs/src/ai/inline-assistant.md` | 55, 60, 79, 84, 88 | `"provider": "ollama"` (or documented BYOK provider) + matching model ids |
| `docs/src/ai/agent-profiles.md` | 54 | same |

Align examples with `assets/settings/default.json` default agent model.

---

## E. Zed cloud / account pages — product decision required

CueCode v1 decouples from zed.dev accounts. For each page, pick one:

- **Stub** — top fork notice: “Not applicable to CueCode distribution; local/BYOK only.”  
- **Rewrite** — CueCode-specific cloud/dashboard/support URLs when they exist.  
- **Remove from SUMMARY** — hide pages until CueCode cloud exists.

| File | What to replace |
|------|-----------------|
| `docs/src/account/billing.md` | All `dashboard.zed.dev` links (L16, 43, 53, 65); `billing-support@zed.dev` emails (L39, 45, 67, 73, 83); hosted-model billing copy |
| `docs/src/account/plans-and-pricing.md` | `dashboard.zed.dev` (L48); Zed Pro / hosted credit narrative |
| `docs/src/business/organizations.md` | `dashboard.zed.dev/create-organization` (L26) |
| `docs/src/business/admin-controls.md` | `dashboard.zed.dev` (L14) |
| `docs/src/business/business-support.md` | `billing-support@zed.dev`, `hi@zed.dev` (L8, 10) |
| `docs/src/authentication.md` | Firewall note `zed.dev` / `collab.zed.dev` (L24); “signing in to zed.dev” wording (L35) — link already `cuecode.dev/sign_in` but text says zed.dev |
| `docs/src/telemetry.md` | `hi@zed.dev` + upstream GitHub issue link (L75) — point to apps/CueCode-IDE repo / your support |
| `docs/src/soc2.md` | `sales@zed.dev` (L10) — N/A fork notice or CueCode contact |

---

## F. Writer / maintainer docs — conventions

| File | What to replace |
|------|-----------------|
| `docs/.conventions/CONVENTIONS.md` | L236, 262, 366 — change “link to zed.dev” → `cuecode.dev` |
| `docs/.conventions/brand-voice/SKILL.md` | L10, 90, 118 — tone reference → cuecode.dev |
| `docs/.conventions/brand-voice/voice-examples.md` | L91, 149, 222 — example URLs → cuecode.dev |
| `docs/README.md` | L32 — image hosting example; L38 — docs-proxy / zed.dev routing → CueCode publish flow |

---

## Exit checklist

When this backlog is done, update [progress Pass D](./03-zed-reference-cleanup-phases#progress-pass-d):

- [ ] `rg 'images\.zed\.dev|cdn\.zed\.dev' docs/` → **0 hits**
- [ ] `rg 'zed\.dev' docs/` → only **documented allowlist** (if any upstream citations remain)
- [ ] `mdbook serve docs` — spot-check project-panel + theme fonts
- [ ] Check off: “Remaining zed.dev / images.zed.dev CDN URLs in docs”

**Allowlist candidates** (if you keep upstream references on purpose):

- Historical changelog lines citing Zed  
- Explicit “fork of Zed” attribution  
- Legal pages that cite Zed Industries services (with fork notice)

Document any allowlist in this file when you add one.

---

## Changelog

| Date | Change |
|------|--------|
| 2026-06-20 | Initial backlog inventory for Pass D CDN polish |
