# Cue Cloud — landing page spec {#cuecloud-landing}

> **Parent:** [marketing/README](./README.md) · Spec index: [00-README](../00-README.md)
>
> **Status:** product + UX + copy contract for the public Cue Cloud marketing
> site. This file owns **what we say publicly**, **page structure**, **visual
> rules**, and **ship criteria**. Architecture truth stays in the private root
> doc [`CUECLOUD.md`](../../../CUECLOUD.md) — do not paste confidential
> architecture into the page or into this marketing contract beyond the
> [confidentiality boundary](#confidentiality).
>
> Lives under `.cursor/specs/` so agents and Planning Hub can discover it. It
> syncs into `apps/CueCode-IDE/.cursor/` with other specs; that is intentional — this
> is a **public messaging** contract, not the private inference architecture.
>
> **Related:** [`CUECLOUD.md`](../../../CUECLOUD.md) ·
> [`cloud/dashboard/`](../../../dashboard/) ·
> [`inference-demo/`](../../../research/side-projects/inference-demo/) ·
> [01-vision](../core/01-vision.md)

---

## One sentence {#one-sentence}

**Cue Cloud** is owned coding inference — open models on hardware we run — that
you use through **CueCode** (our IDE + harness, tuned for this stack) **or**
through a standard API into the IDE you already use.

---

## Goals and non-goals {#goals}

### Goals (v1)

1. Make **Cue Cloud** the brand of the first viewport — not CueCode, not a
   generic “AI coding” pitch.
2. Explain the product in one breath: **per-seat agentic coding inference**.
3. Present **two equal paths in**: CueCode (optimized) · API (bring your IDE).
4. Drive a single primary CTA (default: **Request access** / waitlist).
5. Build belief with a **proof surface** (live or clearly labeled tok/s probe).
6. Hand off cleanly to the **Usage Dashboard** when keys/billing are ready.

### Non-goals (v1)

- Full docs site, API reference, or status page (link stubs OK).
- Pricing page with locked numbers before capacity is real.
- Deep architecture (Harness / Control / Scheduler / exo) on the marketing home.
- Investor deck content, pod SKUs, or white-paper economics on the public page.
- Competing as “yet another chat UI” or “we host DeepSeek.”

### Explicit confidentiality boundary {#confidentiality}

| May appear publicly | Must stay private (this repo / decks only) |
|---------------------|--------------------------------------------|
| “Owned hardware,” “open coding models,” “$1,500/seat/mo · unlimited tokens” | Exact pod configs (e.g. M3 Ultra 512GB counts), capital model |
| “Built for concurrent agent workloads” | Exact scheduler slot counts, floor tok/s targets as SLAs |
| “CueCode density advantages” (plain language) | CHP wire details, exo fork plan, tenant cache fork internals |
| Live or demo latency / throughput **when labeled** | White-paper financial thesis, batching multiples as company KPIs |
| `/architecture` public system story (see [§ architecture page](#architecture-page)) | Investor white paper PDF, debt/margin curves, supply-bet SKUs |

If a claim needs a confidential number to be true, **do not put the claim on
the page** until you can support it with a public measurement or softer wording.

---

## Brand {#brand}

| Token | Value |
|-------|--------|
| **Public product name** | Cue Cloud |
| **Wordmark** | Prefer two words in UI: **Cue Cloud**. Code / domain may use `CueCloud` / `cuecloud`. |
| **Mark (logo)** | Continuous-path icon, sky blue → neon violet gradient — required on nav + hero. See [Brand mark](#brand-mark). |
| **Not the hero brand** | CueCode, CueHarness, CueControl, CueScheduler, exo |
| **Sibling brand** | **CueCode** — our IDE (Zed fork) + harness, optimized for Cue Cloud models |
| **Tagline (working)** | Coding inference, per seat. · Price signal: **$1,500/mo per seat · unlimited tokens** |
| **Domain (TBD)** | Prefer `cuecloud.com` or `cloud.cuecode.dev` — lock before launch |

### Brand hierarchy on the page

1. **Cue Cloud** — platform / inference product (page owner).
2. **CueCode** — first-party IDE path (optimized client).
3. **API** — third-party / bring-your-own-IDE path.
4. Component names — footer, docs, or `/architecture` only — never hero.

### Voice {#voice}

- Direct, technical, confident — engineer talking to engineer.
- Prefer concrete verbs: *run*, *serve*, *batch*, *plug in*, *own*. Never *meter* as a billing story.
- No corporate fluff (“empower,” “seamless,” “revolutionary”).
- No emoji in marketing chrome.
- Spell out “API” and “IDE”; avoid “CHP,” “KV,” “TP MoE,” “admission” on the
  home page unless a later docs page needs them.

---

## Positioning {#positioning}

### The locked story

Cue Cloud runs **open coding models on infrastructure we own and operate**. The
product is not “access to a model.” It is **inference shaped for agentic coding
load**, with **flat per-seat pricing ($1,500/mo · unlimited tokens for that
user)**, exposed as:

1. **CueCode** — our own IDE, built on Zed, with our harness that optimizes
   around these models and agent loops.
2. **API** — OpenAI-compatible (and Ollama-shaped where useful) so you can
   integrate Cue Cloud into **your** IDE or tooling.

CueCode is the **best client**, not the whole company story. Third-party clients
get the same inference plane with **standard** API semantics; CueCode gets
additional harness affordances that improve agent density and latency (say this
in plain language — do not dump protocol names).

### Competitive frame (internal; soft on page)

| We are | We are not |
|--------|------------|
| Owned coding inference + flat unlimited seats | A thin wrapper on a hyperscaler chat API |
| Agent-loop aware (concurrent turns, long sessions) | A playground chatbot |
| CueCode-optimized **and** IDE-agnostic via API | Locked to one editor |
| Open weights on our metal | “The model company” |

### Thesis bullets (for implementers — compress for UI)

1. **Flat seat, not roulette** — $1,500/mo per seat · unlimited tokens for that user. Keys map to people.
2. **Owned stack** — we run the pods; cost and routing are ours to control.
3. **Agentic load** — designed for concurrent coding agents, not single chat demos.
4. **Two doors** — CueCode (tuned) or API (portable).

---

## Audiences and jobs {#audiences}

### Primary (v1 CTA)

**Engineer evaluating coding inference** — wants a key or early access, cares
whether it works in *their* workflow (CueCode or Cursor/Claude Code/custom).

**Job:** understand what Cue Cloud is → believe it’s real → request access.

### Secondary

**Team lead / founder** — seats, usage visibility, keys for the team. Job:
trust flat seat pricing + path to dashboard.

**Investor / partner** (optional deep link, not hero) — thesis + live proof.
Job: open `/demo` or a labeled probe; do not overload the home hero.

### Personas → page emphasis

| Persona | Care about | Surface on home |
|---------|------------|-----------------|
| Solo eng | Speed, plug-in, model quality | Hero + two paths + proof |
| Team lead | Seats, keys, usage | Short “for teams” + CTA to waitlist/dashboard |
| CueCode user | Density / harness fit | CueCode path copy |
| BYO IDE user | OpenAI-compatible endpoint | API path copy |

---

## User paths {#user-paths}

### Happy path A — waitlist (default v1)

1. Land on Cue Cloud home.
2. Read hero (brand + promise + two paths in one sentence).
3. Skim two paths and/or proof strip.
4. Click **Request access** → email (or waitlist form) → confirmation.
5. Optional: open CueCode download / docs stub.

### Happy path B — API-curious

1. Land → scroll to **Two paths**.
2. Choose **Use the API** → see “OpenAI-compatible” + link to docs stub / waitlist.
3. CTA still **Request access** until keys are self-serve.

### Happy path C — CueCode-curious

1. Land → **Two paths** → CueCode.
2. Secondary CTA: **Get CueCode** (download or GitHub) + note that cloud mode
   uses Cue Cloud when enabled.
3. Primary still Request access for cloud seats.

### Happy path D — keys live (v1.1+)

1. Primary CTA becomes **Get a key** → Usage Dashboard OAuth.
2. Dashboard mints `cue_…` → user configures CueCode or third-party client.

### Unhappy paths

| Failure | UX |
|---------|-----|
| Waitlist form error | Inline error; keep copy; retry |
| Demo/probe down | Hide live probe or show “probe offline” — never fake unlabeled numbers |
| Dashboard not ready | Do not link “Get a key”; keep waitlist |
| Mobile narrow | Hero still works; proof may stack; no horizontal card sprawl |

---

## Information architecture {#ia}

### v1 site map

```
/                     ← marketing home
/product/models       ← open coding models (detail)
/product/api          ← OpenAI-compatible API + cue_… auth
/product/pricing      ← flat seats · $1,500/mo
/product/agents       ← concurrent agent loops
/architecture         ← how Cue Cloud runs (public system story)
/access or #access    ← waitlist (modal)
/demo                 ← optional dedicated probe
/dashboard            ← link out when ready
/docs                 ← API quickstart stub
/legal/privacy
/legal/terms
```

### What does **not** live on `/` or `/product/*`

- Full system diagrams (Harness / Control / Scheduler / exo) — those belong on
  **`/architecture`** only, in public-safe language
- Build status of stubs
- Pricing tables before capacity is priced
- GPL / Zed fork legal essay (one footer line max: CueCode is built on Zed)
- Pod SKUs, slot math, floor tok/s, white-paper economics

---

## Page composition {#composition}

### Design hard rules (marketing)

These match the monorepo frontend design rules for branded landing pages:

1. **One composition** — first viewport reads as one piece, not a dashboard.
2. **Brand first** — **Cue Cloud** wordmark **+ mark logo** are hero-level; no
   headline overpowers the brand.
3. **Brand test** — if you remove the nav, the first viewport still feels like
   Cue Cloud (mark + name must be load-bearing).
4. **Hero budget** — brand (mark + name), one headline, one short supporting
   sentence, one CTA group, one dominant visual. No stats strips, schedule
   blocks, or promo chips in the first viewport.
5. **Full-bleed hero visual** — edge-to-edge plane driven by the **brand mark**
   (animated path / gradient) + stream atmosphere — not an inset card collage.
6. **No hero overlays** — no floating badges, stickers, or info chips on the media.
7. **Cards default off** — cards only when they containerize a real choice
   (e.g. two-path selector) or a deliberate contrast table.
8. **One job per section** — one purpose, one headline, usually one short support line.
9. **Sexy ≠ slop** — cinematic motion and logo-gradient energy are required; avoid
   purple-*on-white*, cream+serif terracotta, broadsheet layouts, emoji rows, and
   pill farms. Controlled glow on the mark/CTA is OK when it matches the logo.

### Brand mark (logo) {#brand-mark}

| Item | Rule |
|------|------|
| **Asset** | Official continuous-path mark (sky blue → neon violet gradient on black). Source PNG when available; app may ship an SVG approximation at `sites/landing/apps/web/public/brand/cue-cloud-mark.svg` (and optional `.png`). |
| **Name** | Always pair the mark with the words **Cue Cloud** in nav and hero (mark alone is not enough). |
| **Usage** | Nav (small), hero dominant visual (large), closing CTA, favicon / OG later. |
| **Motion** | Path-draw or gradient travel along the stroke; soft float/rotate ≤ a few degrees; respect `prefers-reduced-motion` (static mark). |
| **Do not** | Recolor to cyan-only; stretch; place on light pastel fields; bury in a card. |

### Section order (home) — L1.5+ {#sections}

L1 shipped a thin outline. **L1.5** densifies information, differentiation, logo,
and motion. Order:

| # | Section ID | Job | Primary content |
|---|------------|-----|-----------------|
| 0 | `nav` | Orient | Mark + **Cue Cloud** · Product · Pricing · Architecture · Docs · CueCode · Log in · Request access |
| 1 | `hero` | Brand + promise + CTA | Copy left · **system stage** right ([§](#hero-system)) |
| 2 | `product` | What Cue Cloud *is* | **Product explorer** (IDE-like) → `/product/*` |
| 3 | `contrast` | Differentiation | “Not another wrapper” — them vs us |
| 4 | `paths` | Two ways in | CueCode vs API (choice interaction) |
| 5 | `how` | How you use it | 3 steps: access → point client → flat seat |
| 6 | `proof` | Belief | Labeled demo probe theater (bigger) |
| 7 | `teams` | Seats | Console / keys / usage (short) |
| 8 | `cta` | Convert | Mark + Request access |
| 9 | `footer` | Legal + links | Privacy, terms, CueCode |

Optional later: `models`, `faq`.

### Differentiation thesis (public) {#differentiation}

Punch in this order on the page (especially `#contrast`):

1. **Owned decode** — we run the hardware open coding models decode on (not a
   thin wrapper on a hyperscaler chat API).
2. **Agent-loop load** — built for concurrent coding agents and long sessions,
   not single-chat demos.
3. **Flat unlimited pricing** — $1,500/mo per seat · unlimited tokens for that user (not usage-based).
4. **Two doors** — CueCode (harness optimized for this stack) **or** OpenAI-compatible
   API into the IDE you already use.

Do **not** name CueScheduler / exo / CHP / slot math on the marketing home.
---

## Section specs {#section-specs}

### 0 — Nav {#nav}

- Left: **brand mark** + **Cue Cloud** wordmark (links `/`).
- Right (main): `Product` (`/#product`) · `Pricing` (`/product/pricing`) · `Docs`
  (`/docs`) · `CueCode` (GitHub IDE).
- Right (CTA): text **Log in** → support dialog (`support@cuecloud.io` for
  dashboard access) until `dashboard.cuecloud.io` ships. Primary **Request access**.
- Shared `SiteNav` on home, `/product/*`, and `/docs`. No mega-menu.
- Sticky; keep height minimal. Subtle blur OK; no glassmorphism sludge.

### 1 — Hero {#hero}

**Layout**

```
┌──────────────────────────────────────────────────────────────┐
│  [mark] Cue Cloud              │  SYSTEM STAGE (dominant)     │
│  Coding inference, per seat.   │                              │
│  Support copy…                 │  Mac Studios → models        │
│  [ Request access ]  [ → ]     │       ↓                      │
│                                │  Cue Cloud (mark hub)        │
│                                │    ↙          ↘              │
│                                │ CueCode      API / IDE       │
│                                │    → coding tokens           │
└──────────────────────────────────────────────────────────────┘
```

**Copy (canonical draft)**

| Element | Copy |
|---------|------|
| Brand | Mark + Cue Cloud |
| Headline | Coding inference, per seat. |
| Support | We run open coding models on infrastructure we own — for concurrent agents, not chat demos. Use **CueCode** — our IDE and harness tuned for this stack — or call the same API from the IDE you already use. |
| Primary CTA | Request access |
| Secondary CTA | See how it works → `#product` |

**Visual — system stage {#hero-system}**

The hero’s dominant visual is a **looping system story**, not a giant static logo:

1. **Mac Studios** — abstract chassis silhouettes (owned hardware). Not Apple marketing art.
2. **Open coding models** — energy / labels rising from the studios.
3. **Cue Cloud hub** — brand mark as the center node (flat-seat inference).
4. **Split** — beam to **CueCode** (IDE + harness) and **API** (any IDE).
5. **Coding tokens** — monospace glyphs streaming into the CueCode pane (the payoff).

| Rule | Detail |
|------|--------|
| Loop | Continuous motion (pipes, LEDs, token marquee, hub breathe) — diagram always visible, never fade-from-zero |
| Density | 4–5 nodes max — cinematic, not a Cisco diagram |
| Confidentiality | No CueScheduler / exo / CHP / slot math / pod SKUs |
| Labels | Public words only: Studios · Models · Cue Cloud · CueCode · API · tokens |
| Reduced motion | Static diagram, all nodes visible, no particle stream |
| Mark | Hub only in the stage; small mark still pairs with wordmark in copy column |

**Must not**

- Lead with “Built on Zed” or “Fork of Zed.”
- Lead with model brand names as the product.
- Put three feature columns in the hero.
- Ship hero without the Cue Cloud mark (as hub or lockup).
- Fake unlabeled tok/s as an SLA in the hero stage.
### 2 — What you get {#product}

**Job:** Answer “what is Cue Cloud?” with **specific** deliverables, then send
people to deeper pages. Not a generic four-card feature grid.

**Headline:** What Cue Cloud is.

**Support:** Coding inference we own. Flat seats, unlimited tokens. Built for agents at work.

#### Home: product explorer (IDE surface)

Present as a **two-pane explorer** (sidebar + preview), not marketing cards:

```
┌──────────────────┬────────────────────────────────────────┐
│ models           │  preview: specific copy + sample       │
│ api          ●   │  Open full page →                      │
│ seats            │                                        │
│ agents           │                                        │
└──────────────────┴────────────────────────────────────────┘
```

| Rule | Detail |
|------|--------|
| Interaction | Rows are selectable (click / keyboard). Active row shows preview. |
| Look | File-tree / editor chrome: mono paths, line highlight, subtle border. Cards off. |
| Specificity | Preview shows concrete signals (e.g. `POST /v1/chat`, `cue_…`, seat mapping). |
| Depth | Every row links to `/product/{slug}` for the full breakdown. |
| Confidentiality | Still no scheduler/exo/slot/pod BOM on home or detail pages. |

| Slug | Sidebar label | Home preview (short) | Detail page |
|------|---------------|----------------------|-------------|
| `models` | `models` | Open coding models on hardware we run | `/product/models` |
| `api` | `api` | OpenAI-compatible + IDE-optimized coding path | `/product/api` |

**`/product/api` (beefed):** sticky outline · flat-seat trust strip · interactive
request lab (curl/Python/TS + model chips) · auth · model id table + HF ·
client drop-ins (Cursor / Claude Code / CueCode / scripts) · streaming sketch ·
CueCode vs API · **works anywhere, tuned for coding IDEs** · errors · pricing.
Base URL placeholder `https://api.cue.cloud/v1` until OQ-2 locks domain.

| `pricing` | `pricing` | $1,500/mo flat · unlimited tokens per seat | `/product/pricing` |

**`/product/pricing` (beefed):** sticky outline · **price monument** ($1,500 /
seat / mo · unlimited) · what unlimited means · deep vs Claude/GPT rate-card
table · illustrative spend scenarios (labeled) · why agents break usage pricing ·
seat anatomy · team math · console · FAQ. H1 **Pricing**, file `pricing.md`;
body copy still says “seats.” `/product/pricing` → permanent redirect.
| `agents` | `agents` | Why coding agents · latency · owned decode | `/product/agents` |

**`/product/agents` (beefed):** sticky outline · **why coding agents** (chat vs
agent contrast) · **animated agent loop** (plan → reads → tools → patch →
verify + tool fan-out) · **latency vs throughput** dual motion · **latency
levers** (owned path, workload fit, CueCode density, per-seat isolation —
no unlabeled SLAs) · **system + hardware path** (owned Mac Studio–class fleet
→ open models → Cue Cloud hub → CueCode/API stream; public altitude only) ·
CueCode density honesty · flat seat fuel CTA → `/product/pricing`. Respect
`prefers-reduced-motion`. No CueScheduler / exo / CHP / slot math / floor tok/s.

#### Detail pages (`/product/*`)

Shared chrome: Cue Cloud nav, **same site footer as `/`**, back to `/#product`,
Request access CTA.
Each page: headline, 2–4 concrete sections, one code or console sample where
useful, closing CTA. No fluff “empower” copy; no em dashes.

**Models must be specific:** public lineup is **DeepSeek V4**
(`deepseek-v4-pro`), **Kimi K2.7 Code** (`kimi-k2.7-code`), and **GLM 5.2**
(`glm-5.2`). Do not ship vague `"model": "coding"` sketches on `/product/*`.

**`/product/models` model lineup (L1.5+):** sticky **outline rail** (IDE file
tree: jump + scroll-spy; nested flagships select model + jump to specs) →
trust callout (“open weights we deploy from Hugging Face”) → Hub repo strip
with live links → segmented picker → live spec card (incl. HF source link) →
request sketch → **highlighted benchmark panel** (vendor-reported bars vs
frontier peers, sourced) → job-fit frontier matrix → prose. Specs live in
`sites/landing/apps/web/src/lib/models.ts`.

Hub receipts (must stay live):

- https://huggingface.co/deepseek-ai/DeepSeek-V4-Pro
- https://huggingface.co/moonshotai/Kimi-K2.7-Code
- https://huggingface.co/zai-org/GLM-5.2

### 3 — Contrast {#contrast}
**Job:** Name the contemporary fear — agent OpEx without a ceiling — then show
the escape: flat seats on owned Mac Studio decode.

**Kicker (mono):** Fixed seat · owned decode · unlimited agent tokens

**Headline:** The invoice became the bottleneck.

**Support:** Frontier APIs price coding agents like chat: every read, tool call,
and retry lands on an OpenAI or Anthropic meter. Teams want agents on every
desk; finance wants a ceiling. Cue Cloud is that ceiling — because inference
runs on Mac Studios we operate, not a thin client over someone else’s
`/v1/chat`.

| Metered frontier | Cue Cloud |
|------|-----------|
| `$/MTok` on every turn, retry, tool call | **$1,500 / seat / mo flat** (monument row) |
| Cost ∝ fan-out · context · session length | Unlimited tokens for that engineer |
| Shared org keys · mystery burn | One human ↔ one seat ↔ one `cue_…` |
| Hyperscaler chat as “coding infra” | Decode on Mac Studios we operate |
| Forecasting = spreadsheet anxiety | OpEx you can put in a headcount plan |

**Hardware band (under table):** “Owned hardware is the pricing model” —
Mac Studio–class nodes · open Hub weights · seats on capacity we control, not
a markup on someone else’s rate card. Public altitude only (no RAM/SKU/scheduler).

**CTAs:** See pricing → `/product/pricing` · Request access

**Interactive invoice duel (`ContrastDuel`):** same agent-loop turns on both
panes — left invoice climbs (illustrative $/turn), right seat stays
`$1,500`. Autoplay on scroll into view; Play / Pause / Replay; break-even
marker at Cue seat; Mac Studio LEDs pulse on Cue side. Labeled illustrative.
`prefers-reduced-motion` → final frame. Link footnote → `/product/pricing`.

Keep hardware band under the duel. No emoji. Name OpenAI / Anthropic.

### 4 — Two paths {#paths}

**Job:** Dual doors into the same Cue Cloud inference plane.

**Headline:** Two doors. One inference plane.

**Support:** Owned decode either way. CueCode rides the density path; the API
drops into the IDE you already use.

| Path | Preview | CTA |
|------|---------|-----|
| **CueCode** | Mini IDE chrome + agentic-linear chip + token marquee | **Get CueCode on GitHub** → `https://github.com/CueCard-AI/CueCode-IDE` |
| **API** | `POST /v1/chat` · `cue_…` · model id · stream pulse | **Request access** + **See API →** `/product/api` |

**UX:** Equal-weight door cards with live previews; active door gets logo-gradient
border + motion (inactive dims, pauses anim). Mini Cue Cloud hub fork above.
Honesty strip: *Same Cue Cloud. CueCode unlocks denser agent signals; the API
works everywhere.* No shared detail slab. `prefers-reduced-motion` kills door motion.

### 5 — How it works {#how}

**Job:** 3-beat onboarding pipeline — usable, not abstract.

**Headline:** From access to agents.

**Support:** Three beats to a flat seat. System internals live on
[`/architecture`](#architecture-page).

| Step | Title | Preview | CTA |
|------|-------|---------|-----|
| 01 | Request access | waitlist pulse | Request access (dialog) |
| 02 | Point your client | CueCode \| API split | CueCode → GitHub · See API |
| 03 | Ship on a flat seat | $1,500 lock | See pricing → `/product/pricing` |

**UX:** Connected pipeline nodes above cards; active step cycles (~2.8s) + hover
override; logo-gradient border on active; traveling beam between nodes.
`prefers-reduced-motion` freezes cycle. CueCode → `https://github.com/CueCard-AI/CueCode-IDE`.

### 5b — Architecture page {#architecture-page}

**Route:** `/architecture`  
**Nav:** Product · Pricing · **Architecture** · Docs · CueCode · …

**Job:** Make Cue Cloud feel *concrete* — how the system runs — without
publishing the confidential white paper or `CUECLOUD.md`.

**Audience:** developers / technical buyers. Jargon OK (`MoE`, `KV`,
`pipeline-parallel`, `continuous batching`). No marketing metaphors.

**Voice:** engineer Slack, not landing pitch. Prefer constraints and stack
nouns over “plane / invert / good-enough.”

**Job:** Diagram-led system story (sticky TOC + animated stages). Copy is
caption. Do not publish the confidential white paper or `CUECLOUD.md`.

**Not this page:** API reference (`/docs`), seat math (`/product/pricing`),
model cards (`/product/models`).

#### Section order

| # | ID | Headline | Content |
|---|-----|----------|---------|
| 0 | hero | Cue Cloud stack | Mac Studios named up front |
| 1 | problem | Agent loops break metered APIs | Meter vs flat seat |
| 2 | hosting | Your prompts go somewhere | Vendor / router / **Mac Studios we operate** |
| 3 | studios | Why Mac Studios | Unified memory vs CPU/VRAM copy |
| 4 | path | What happens when you call the API | Traveler through Studio pod |
| 5 | pod | One model, split across Mac Studios | Pipeline parallel, Studios labeled |
| 6 | layers | The chat API is not the hard part | API layer vs decode/batching |
| 7 | capacity | Shared vs dedicated | Fleet tenancy, plain language |
| 8 | build | Buying Mac Studios ≠ running Cue Cloud | DIY boxes vs serving stack |
| 9 | cta | Get a seat | Access · Pricing · API |

**Banned on this page:** “provenance”, “moat”, “our metal”, “hop the fabric”,
“planes” as section title (use API vs decode).

#### Forbidden

- White-paper PDF / “Confidential” framing
- Margin %, debt, capex, interconnect queues
- Exact batching multiples or floor tok/s as SLAs
- Pod BOM / hop latency as guarantees
- Wholesale paste from `CUECLOUD.md`

**Home hero:** “See how it works →” → `/architecture`.

### 6 — Proof {#proof}

**Job:** Show that inference is real (or honestly demoed). Make it theatrical.

**Headline:** See it decode.

**Support:** Shaped demo probe — pacing only until the fleet is public.

**Interactive probe (`Proof`):** console chrome with `DEMO · NOT PRODUCTION`
badge · model chip · session id · Replay/Pause · metrics labeled like the live
inference-demo — **Live display** / **Reply avg** (shaped paint rate, e.g.
roughly mid-teens /s; not billed API tok/s), TTFT, tokens out, sparkline · event
log · streaming code pane · Mac Studio LEDs. Autoplay on scroll. Always labeled
illustrative — not an SLA. Prefer linking/embedding
`research/side-projects/inference-demo/` without dropping the badge until
CueCloud is measured.

### 7 — Teams {#teams}

**Headline:** Built for seats, not mystery bills.

**Support:** Team keys, usage visibility, and billing land in the Cue Cloud
console. Coming with access.

### 8 — Closing CTA {#closing-cta}

Mark + **Ready to run on Cue Cloud?** + Request access.

### 9 — Footer {#footer}

- Mark + Cue Cloud
- Product: Explorer · `models.md` · `api.md` · `pricing.md` · `agents.md` · Access
  (root-relative links so `/product/*` still works)
- CueCode: IDE + harness · Demo
- Legal: Privacy · Terms
- Optional: CueCode is built on Zed.
- Render on **both** `/` and `/product/*`. No social icon soup unless accounts exist.
---

## Copy deck {#copy-deck}

### Meta

| Field | Draft |
|-------|-------|
| `<title>` | Cue Cloud — Coding inference, per seat |
| Meta description | Owned coding inference on open models. Use CueCode — our IDE and harness — or plug the API into your own IDE. |
| OG title | Cue Cloud |
| OG description | Coding inference, per seat. |

### Microcopy

| Key | Copy |
|-----|------|
| Waitlist placeholder | work@email.com |
| Waitlist submit | Request access |
| Waitlist success | You’re on the list. We’ll follow up when seats open. |
| Waitlist error | Something broke — try again. |
| CueCode density honesty | CueCode includes agent optimizations for this stack. The API works with any OpenAI-compatible client. |
| Probe mock label | Demo probe (illustrative) |
| Probe live label | Live probe |

### Words we avoid on `/`

`exo`, `CHP`, `CueScheduler`, `admission`, `KV cache`, `MoE`, `RDMA`,
`token roulette` (internal slang only), `hyperscaler tax` (ok in decks, sharp
for home — prefer softer “owned infrastructure”).

---

## Visual system {#visual}

### Direction

**Cinematic dark + logo gradient** — near-black stage, brand mark as the hero
signal, sky-blue → neon-violet energy from the official Cue Cloud mark. More
cinematic than the dashboard; less app-chrome than inference-demo.

### Tokens (L1.5 — locked to logo)

Default theme is **cue dark** (near-black + logo blue→violet). The site also
ships a **theme picker** (footer chip · `/` · settings modal) with editor-classic
palettes; all remaps map onto the same CSS variables (`--bg`, `--fg`, `--blue`,
`--violet`, `--panel`, …). Brand mark PNG stays fixed; shell recolors.

| Token | Guidance |
|-------|----------|
| Background | Near-black `#07080c`–`#0c0e14` with depth (radial logo-gradient washes, soft noise) |
| Foreground | High-contrast off-white |
| Muted | Cool gray |
| Accent | **Logo gradient** — sky blue `#4da3ff` → violet `#8b5cf6` / neon purple. Primary buttons may use a solid stop or gradient fill. |
| Glow | Tight, logo-colored bloom on mark + CTA hover only — not full-page neon soup |
| Fonts | **Space Grotesk** (display / wordmark) + **IBM Plex Sans** (body) + JetBrains Mono (code / probes). No Inter/Roboto-only; no soft geometric stacks (Syne, Figtree, Poppins). |
| Radius | Slightly softer to echo the mark’s rounded path (~8px). No pill farms. |

### Theme catalog

| Group | Themes |
|-------|--------|
| Core | `cue dark` (default) |
| Dark | catppuccin, terminal, tokyo night, dracula, nord, gruvbox, one dark, solarized, kanagawa, rose pine, vesper |
| Light | catppuccin latte, tokyo day, gruvbox light, one light, solarized light, kanagawa lotus, rose pine dawn |

Persistence: `localStorage` key `cuecloud-theme` · `html[data-theme]` + inline CSS vars via `ThemeProvider`.
| Borders | Hairline; logo-gradient border on selected path / contrast table |

### Motion {#motion}

**Required intentional set (L1.5+):**

1. **Hero system stage** — looped story: Studios → models → Cue Cloud hub →
   CueCode / API → coding tokens ([§ hero system](#hero-system)).
2. **Mark** — hub in the system stage (+ small lockup beside wordmark); subtle
   glow/pulse when the hub is active in the loop.
3. **Hero atmosphere** — slow gradient / stream drift (logo colors).
4. **Scroll reveal** — sections fade/rise once into view.
5. **CTA** — hover lift + tight gradient glow.
6. **Proof** — token cadence + latency probe pulse.
7. **Path selection** — accent border/gradient transitions.

Respect `prefers-reduced-motion`: static system diagram (all nodes visible), no
particle stream, no drift, instant reveals.

### Imagery

- Hero dominant visual = **system stage** (hardware → tokens); mark is the hub.
- Prefer product signal (stream, probe, IDE pane) over abstract blob-only backgrounds.
- No stock “developer smiling at laptop” hero.
- Mac Studio nodes are **abstract silhouettes**, not Apple product photography.---

## CTA and funnel {#funnel}

### v1 default

| Priority | CTA | Destination |
|----------|-----|-------------|
| Primary | Request access | Waitlist form (email + optional IDE preference: CueCode / API / both) |
| Secondary | See how it works | `/architecture` |
| Tertiary | Get CueCode | CueCode download / repo when public enough |

### v1.1 (keys live)

| Priority | CTA | Destination |
|----------|-----|-------------|
| Primary | Get a key | Usage Dashboard signup |
| Secondary | Open docs | API quickstart |
| Tertiary | Get CueCode | IDE |

### Waitlist fields (v1)

- Email (required)
- How you’ll use Cue Cloud: `CueCode` | `API / other IDE` | `Both` | `Not sure`
- Optional: company

Store somewhere durable (e.g. Resend audience, DB table, or spreadsheet for
alpha). Spec does not mandate vendor — implementation PR should name one.

---

## Relationship to existing surfaces {#surfaces}

| Surface | Role vs landing |
|---------|-----------------|
| **This page** | Acquisition + positioning |
| **Usage Dashboard** (`cloud/dashboard/`) | Post-access console — keys, usage, billing |
| **Inference Demo** (`research/side-projects/inference-demo/`) | Proof / probe — embed or link |
| **CueCode IDE** | Optimized client — secondary CTA |
| **[`CUECLOUD.md`](../../../CUECLOUD.md)** | Private architecture — never mirrored into page copy wholesale |

### Reuse guidance

- Steal **SSE event shapes** and tok probe UX from inference-demo; restyle for
  marketing (less app chrome).
- Do **not** reuse dashboard nav/shell on the marketing home.
- Implement as a **standalone Next.js app** in [`sites/landing/`](#repo-placement) —
  do not bolt a marketing hero onto the authenticated dashboard layout.

### Repo placement (implementation) {#repo-placement}

**Locked:** the Cue Cloud marketing site lives at **`sites/landing/`** (under the
`sites/` shelf — not bare monorepo-root `landing/`).

| | Spec (this file) | App (implementation) |
|--|------------------|----------------------|
| Path | `.cursor/specs/marketing/01-cuecloud-landing.md` | `sites/landing/` |
| Role | Product / UX / copy contract | Runnable Next.js site |
| Visibility | Synced via `.cursor/` | Private monorepo root (same class as `cloud/dashboard/`) |
| Not here | — | **Not** under `apps/CueCode-IDE/`, `cloud/dashboard/`, or `research/side-projects/` |

```
CueInference/
├── sites/landing/                         ← Cue Cloud marketing site (THIS APP)
│   ├── README.md                    ← run / deploy / env
│   ├── AGENTS.md                    ← agent notes (optional but preferred)
│   └── apps/web/                    ← Next.js App Router (or top-level app if simpler)
├── cloud/dashboard/                       ← post-access console (keys, usage) — separate
├── research/side-projects/inference-demo/    ← proof / probe — link or embed; do not merge
├── apps/CueCode-IDE/                     ← IDE only — do not put the landing page here
└── .cursor/specs/marketing/         ← this spec
    └── 01-cuecloud-landing.md
```

When scaffolding L1, also register the project in:

- [`MONOREPO.md`](../../../MONOREPO.md) (doc index + project map)
- [`AGENTS.md`](../../../AGENTS.md) (projects table)
- [`README.md`](../../../README.md) (“what's in this repo” table) if other web apps are listed there

Do **not** rename to `cuecloud-web/` or nest under `research/side-projects/` without updating this section first.
---

## Technical requirements {#tech}

| Concern | Requirement |
|---------|-------------|
| Stack | **Next.js App Router + TypeScript** (locked). Match cloud/dashboard/demo familiarity. Do not swap frameworks without updating this spec. |
| Performance | LCP-focused hero; lazy-load probe; no huge client JS for static copy |
| Responsive | Mobile: single column; hero type readable; CTAs thumb-reachable |
| A11y | Semantic landmarks, focus states, contrast, reduced motion |
| Analytics | Page view + CTA click + waitlist submit (vendor TBD) |
| SEO | Meta + OG + canonical; public indexing OK for `/` — **full plan:** [03-seo-geo](./03-seo-geo.md) |
| Secrets | Waitlist webhook keys in env — never in client bundle |
| Confidentiality | Build must not import or render text from `CUECLOUD.md` |

---

## Phased delivery {#phases}

### Phase L0 — Spec lock (this doc)

- [x] Positioning, IA, section order, copy drafts, confidentiality boundary
- [ ] Domain + primary CTA final (waitlist vs keys) — see [Open questions](#open-questions)

### Phase L1 — Static marketing page

- [x] Scaffold monorepo root **`sites/landing/`** ([repo placement](#repo-placement))
- [x] Add `sites/landing/README.md` (+ `AGENTS.md` preferred); register in `MONOREPO.md` / `AGENTS.md`
- [x] Nav + hero + paths + difference + teams + CTA + footer
- [x] Visual tokens + fonts + motion baseline
- [x] Waitlist form wired to a real sink
- [ ] Mobile pass + a11y pass (manual QA)

### Phase L1.5 — Cinematic + denser pitch (current)

- [x] Brand mark in nav / hero / closing / favicon slot (`public/brand/`)
- [x] Visual system pivoted to logo blue→violet gradient
- [x] Sections: `#product`, `#contrast`, `#how` added; `#difference` folded into contrast/product
- [x] Hero copy updated (agents + owned infra + two doors)
- [x] Motion set: mark, atmosphere, scroll reveal, CTA glow, proof theater
- [x] **Hero system stage** — Studios → models → Cue Cloud → CueCode/API → tokens
- [x] **Product explorer** + `/product/{models,api,seats,agents}` detail pages
- [ ] Page feels information-dense and differentiated vs “another AI API” (human QA)
### Phase L2 — Proof

- [x] Illustrative `#proof` probe with correct mock/demo labeling (shaped Live display / Reply avg)
- [x] Live tok-speed-demo embed removed from landing (demo stays at its own host; no iframe/CTA on `/`)

### Phase L3 — Funnel upgrade

- [ ] Primary CTA → Usage Dashboard when `cue_…` minting is ready
- [x] Optional `/docs` Coming soon stub (full quickstart later)
- [x] `/architecture` public system story (no white-paper economics)

### Phase L4 — Polish

- [ ] OG images, social card
- [ ] FAQ if support load warrants it
- [ ] Optional `/cuecode` page if IDE marketing needs room

---

## Acceptance criteria {#acceptance}

### Brand / content

- [ ] First viewport fails the brand test only if “Cue Cloud” is removed (i.e. brand is load-bearing).
- [ ] Headline does not overpower the wordmark.
- [ ] Two paths present CueCode and API with equal weight and honest density caveat.
- [ ] No confidential architecture numbers or internal component tour on `/`.
- [ ] Meta title/description match copy deck.

### UX

- [ ] Hero stays within hero budget (no stat strips / promo chips / card collage).
- [ ] Primary CTA visible without scrolling on desktop and mobile.
- [ ] Waitlist success and error states work.
- [ ] Probe is labeled if not production-live.
- [ ] `prefers-reduced-motion` disables continuous animation.

### Engineering

- [ ] App lives at monorepo root **`sites/landing/`** (not under `apps/CueCode-IDE/`, `cloud/dashboard/`, or `research/side-projects/`).
- [ ] Documented in `MONOREPO.md` / `AGENTS.md` (and `sites/landing/README.md`) when scaffolded.
- [ ] No secrets in git.

---

## Open questions {#open-questions}

| ID | Question | Default until decided |
|----|----------|----------------------|
| OQ-1 | Primary CTA: waitlist vs dashboard keys? | **Waitlist** until keys are self-serve |
| OQ-2 | Public domain? | TBD — block launch copy that hardcodes the wrong host |
| OQ-3 | Embed probe vs link `/demo`? | **Link** if embed threatens LCP; else embed strip |
| OQ-4 | Public model names on home? | Soft on hero; **named on `/product/models` + API samples**: DeepSeek V4, Kimi K2.7 Code, GLM 5.2 — with Hugging Face links |
| OQ-5 | CueCode download readiness? | Tertiary CTA can be “GitHub” or hidden until builds are ready |
| OQ-6 | Accent / brand colors? | **Locked to logo gradient** (sky blue → violet) — see [Visual system](#visual) |
| OQ-7 | Waitlist vendor? | **Supabase** `waitlist_leads` (+ local `.data/waitlist.jsonl` fallback; optional `WAITLIST_WEBHOOK_URL`) |
| OQ-8 | Official mark PNG vs SVG approx? | **Resolved** — official PNG at `sites/landing/apps/web/public/brand/cue-cloud-mark.png` |

---

## Implementation checklist for agents {#agent-checklist}

When building from this spec:

1. Read this file end-to-end + [`CUECLOUD.md`](../../../CUECLOUD.md) for truth —
   **do not** paste confidential sections into the page.
2. Scaffold the **site** at monorepo root **`sites/landing/`** — see
   [repo placement](#repo-placement). Not under `apps/CueCode-IDE/`. This **spec**
   stays in `.cursor/specs/marketing/`.
3. Follow section order and hero budget; do not invent extra first-viewport modules.
4. Keep Cue Cloud as the brand; CueCode is path A, not the site title.
5. Wire waitlist before obsessing over architecture diagrams.
6. Label any non-production probe.
7. Update this changelog when shipping behavior diverges from the drafts above.

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-07-20 | Monorepo layout H0–H6 complete — site stays at `sites/landing/` ([ops/14](../ops/14-monorepo-layout.md)). |
| 2026-07-13 | Linked SEO row to [03-seo-geo](./03-seo-geo.md) (sitemap/robots/OG/JSON-LD/llms.txt plan) |
| 2026-07-13 | `/architecture` tangible rewrite: Mac Studios named, hosting/why-Studios/pod/build-vs-buy; drop provenance/moat speak. |
| 2026-07-13 | `/architecture` rebuild: sticky TOC, animated stages, engineer-voice copy. |
| 2026-07-13 | Spec + ship `/architecture` public system story; nav Architecture; hero → `/architecture`. |
| 2026-07-13 | Log in opens support dialog (`support@cuecloud.io`) until `dashboard.cuecloud.io` ships. |
| 2026-07-13 | Waitlist verifies email MX (+ disposable block) and website HTTP reachability server-side. |
| 2026-07-13 | Waitlist success: “we’ll reach out shortly” + intro-call iframe (`sandbox-dev.cuegrowth.ai` booking) + new-tab fallback. |
| 2026-07-13 | Waitlist wired to Supabase project: `waitlist_leads` table + RLS insert policies; landing `.env.local` points at project. |
| 2026-07-13 | Waitlist: required company + website + engineers; FancySelect dropdowns; Supabase `waitlist_leads` (+ jsonl fallback). |
| 2026-07-13 | Theme curly-fry coachmark always visible (no localStorage dismiss). |
| 2026-07-13 | `/docs` is a Coming soon stub (nav stays; no fake quickstart). |
| 2026-07-13 | Product slug `seats` → `pricing` (H1 Pricing, `pricing.md`); `/product/seats` redirects. |
| 2026-07-13 | Nav: Product · Pricing · Docs · CueCode · Log in · Request access; `/docs` quickstart stub; shared `SiteNav`. |
| 2026-07-13 | Theme hint: continuous curly-fry tip + “change theme” microcopy (hint v4). |
| 2026-07-13 | Theme coachmark slim: curly-fry squiggle arrow, no themes pill, compact 36px chip (hint v3). |
| 2026-07-13 | Theme dock rebuild: 48px chip, ~100px draw+bob down-arrow, stays until open; hint key `v2`. |
| 2026-07-13 | Theme coachmark: large draw+bob arrow stays until chip click / `/` (no auto-timeout). |
| 2026-07-13 | Theme chip larger + one-shot “try themes” arrow coachmark (dismiss on open / 12s / localStorage). |
| 2026-07-13 | Theme chip fixed bottom-left (Herdr-style); hero/atmosphere washes use theme CSS vars. |
| 2026-07-13 | Theme system: 19 themes (cue dark default + editor classics), footer chip `/`, settings modal, `localStorage` + CSS var remap (`ThemeProvider`). |
| 2026-07-21 | `#proof`: removed live tok-speed-demo iframe/CTA; keep illustrative shaped probe only. |
| 2026-07-20 | `#proof` live embed failure state when `/health` fails; CTA remains. |
| 2026-07-20 | `#proof` UI: **Live display** / **Reply avg** + live demo CTA/iframe (`tok-speed-demo.cueinference.ai`). |
| 2026-07-20 | `#proof` metrics aligned with inference-demo: **Live display** / **Reply avg** (display paint rate, not billed SLA); link live probe. |
| 2026-07-13 | `#proof` shaped live probe: TTFT, 16–25 tok/s band, stream pane, studios (`Proof`). |
| 2026-07-13 | `#how` pipeline: cycling steps, previews, waitlist/GitHub/seats CTAs. |
| 2026-07-13 | `#paths` dual-door stage: live IDE/API previews, GitHub CueCode CTA, hub fork (`Paths`). |
| 2026-07-13 | `#contrast` invoice duel: interactive metered-vs-flat agent loop sim (`ContrastDuel`). |
| 2026-07-13 | `#contrast` rewritten: invoice bottleneck · metered frontier vs flat seats · Mac Studios pricing band · seats CTA. |
| 2026-07-13 | `/product/agents` beefed: why coding agents, animated loop, latency/throughput, latency levers, owned hardware path, density + seats CTA (`AgentsSurface`). |
| 2026-07-13 | Product explorer: IDE-like `#product` pane + `/product/{models,api,seats,agents}` detail pages (deeper, clickable, specific). |
| 2026-07-13 | Typography: Space Grotesk + IBM Plex Sans (replaced Syne/Figtree for a more official/tech feel). |
| 2026-07-13 | Hero **system stage**: Mac Studios → open models → Cue Cloud hub → CueCode/API → coding tokens (spec + `HeroSystem` animation). |
| 2026-07-20 | Repo placement → **`sites/landing/`** (monorepo layout H2). |
| 2026-07-13 | **L1.5 implemented:** brand mark SVG, denser sections (`product`/`contrast`/`how`), logo-gradient visual system + motion; official PNG still preferred when re-supplied (OQ-8). |
| 2026-07-13 | **L1.5 spec:** brand mark required; denser IA (`product` / `contrast` / `how`); differentiation thesis; cinematic motion set; visual tokens locked to logo blue→violet (replaces cyan-or-amber). |
| 2026-07-13 | L1 scaffolded: `sites/landing/apps/web` Next.js App Router site (sections + waitlist → `.data/waitlist.jsonl`). Stack locked to Next.js + TypeScript. |
| 2026-07-13 | Locked implementation path: monorepo root `sites/landing/` (not `cuecloud-web/`, not under IDE/dashboard/side-projects); L1 + acceptance + agent checklist updated. |
| 2026-07-13 | Moved from repo-root `CUECLOUD_LANDING.md` → `.cursor/specs/marketing/01-cuecloud-landing.md` for agent/spec discovery (syncs with `.cursor/`; architecture stays private in `CUECLOUD.md`). |
| 2026-07-13 | Initial Cue Cloud landing page spec — positioning (Cue Cloud platform; CueCode optimized IDE + harness **or** API for your IDE), IA, section/copy drafts, visual rules, funnel phases, confidentiality boundary. |
