# Cue Cloud — SEO & GEO technical plan {#cuecloud-seo-geo}

> **Parent:** [marketing/README](./README.md) · Spec index: [00-README](../00-README.md)
>
> **Status:** S0–S7 shipped (technical pack + `/compare` content). Monitor GSC indexing; optional Bing / analytics later.
>
> **Related:** [01-cuecloud-landing](./01-cuecloud-landing.md) (IA, confidentiality,
> SEO one-liner) · [02-site-assistant](./02-site-assistant.md) (`SITE.md` brief) ·
> app: [`sites/landing/apps/web`](../../../sites/landing/apps/web/) · live:
> [`https://cuecloud.io`](https://cuecloud.io)

---

## One sentence {#one-sentence}

Make **cuecloud.io** technically excellent for **classic search (SEO)** and
**answer engines / AI crawlers (GEO)** — crawlable URL map, honest robots policy,
canonical metadata, rich previews, structured data, and a public machine-readable
brief — without leaking confidential stack details from `CUECLOUD.md`.

---

## Definitions {#definitions}

| Term | Meaning here |
|------|----------------|
| **SEO** | Technical + on-page fitness for search engines (Google, Bing, etc.): crawl, index, rank, snippet quality |
| **GEO** | Generative engine optimization: fitness for AI answer products (ChatGPT Search, Perplexity, Gemini, Copilot, AI Overviews) that **cite** or **ground** on public pages |
| **Content signals** | Cloudflare / IETF-style robots annotations for `search` / `ai-input` / `ai-train` (see live `robots.txt` today) |
| **Public altitude** | Copy allowed on the marketing site per [01 § confidentiality](./01-cuecloud-landing.md#confidentiality) |

**GEO is not “stuff keywords into Ask Cue.”** Ask Cue helps on-site visitors. GEO
is about the **open web** being easy to fetch, trust, and cite.

---

## Current state (audit snapshot) {#current-state}

Captured **2026-07-13** against production. Re-verify before marking phases done.

### Working

| Item | Evidence |
|------|----------|
| HTML SSR of primary copy | Googlebot-UA fetch of `/` includes H1 “Coding inference, per seat.” and seat pricing in first HTML |
| `lang="en"` | Root layout |
| Per-route `<title>` + `description` | `/`, `/architecture`, `/cuecode`, `/docs`, `/product/*` via `metadata` / `generateMetadata` |
| `www` → apex | `https://www.cuecloud.io/` → **301** → `https://cuecloud.io/` |
| Icons | favicon + apple-touch + brand mark |
| Next font loading | Space Grotesk / IBM Plex / JetBrains via `next/font` |

### Broken or missing

| Item | Evidence / impact |
|------|-------------------|
| **`/sitemap.xml`** | **404** — no URL inventory for crawlers |
| **`/robots.txt`** | Serves Cloudflare **content-signal preamble only** — **no** `User-agent`, `Allow`/`Disallow`, or `Sitemap:` line. Not a usable crawl policy |
| **`metadataBase`** | Unset in root `metadata` — relative OG/canonical resolution is unreliable |
| **Canonical URLs** | No `alternates.canonical` on routes |
| **`og:image` / Twitter large card** | OG has title/description/type only; Twitter `summary` (not `summary_large_image`); no dedicated 1200×630 asset in `public/` |
| **`og:url` / `og:site_name`** | Missing |
| **JSON-LD** | No `application/ld+json` Organization / WebSite / Product / FAQ |
| **`/llms.txt`** (and full variant) | **404** — no sanctioned GEO brief |
| **Thin `/docs`** | “Coming soon” indexed like a real docs hub — risk of soft-404 / weak SERP |
| **Brand mark `alt=""`** | Decorative empty alts on repeated marks — fine for chrome, weak if logo is the only brand signal in some contexts |
| **Spec SEO row incomplete** | [01 § technical](./01-cuecloud-landing.md) still “Meta + OG + canonical” as a checkbox-level requirement without this plan |

### Cloudflare / edge note

Production `robots.txt` is **Cloudflare-substituted**. Origin Next currently
**404**s `/robots.txt`. See [S0 findings](#s0-findings). Until S1 ships
`app/robots.ts` **and** S1.7 confirms live merge/pass-through, treat edge robots
as **non-authoritative** for crawl policy.

---

## Goals and non-goals {#goals}

### Goals

1. Every public marketing URL is **discoverable** (sitemap) and **allowed** (robots)
   under an explicit policy.
2. Every indexable page has **unique title, description, canonical, and social
   preview** (OG + Twitter) with a real share image.
3. Machines can extract **who we are, what we sell, and price** via JSON-LD +
   `llms.txt` without scraping chat.
4. Answer engines can **cite stable paths** (`/product/pricing`, `/architecture`,
   `/cuecode`) that match on-page H1/H2 facts.
5. Stay inside the **confidentiality boundary** — no pod SKUs, tok/s floors,
   margins, scheduler internals, component code names.

### Non-goals

- Buying backlinks, guest posts, or paid directory spam.
- Keyword-stuffed blog farm in v1 (optional later content track).
- Guaranteeing rankings or AI citations (we control technical fitness only).
- Indexing Ask Cue transcripts or waitlist PII.
- Exposing `CUECLOUD.md`, Terraform, or deploy secrets.
- Replacing human sales / intro call.

---

## Crawl & AI policy (decision) {#crawl-policy}

**Status: LOCKED 2026-07-13 (phase S0).** Defaults below are the shipping policy
until this section is explicitly revised.

| Signal | Value | Rationale |
|--------|-------|-----------|
| Classic search crawl | **Allow** all public marketing paths | We want Google/Bing |
| `ai-input` (grounding / RAG-style fetch for answers) | **Allow (yes)** | GEO goal: be citable |
| `ai-train` (training / fine-tune corpora) | **Disallow (no)** | Prefer citation over free training on brand copy; revisit if legal/product wants otherwise |
| Private / junk paths | **Disallow** | `/api/*`, Next internals, any future `/admin` |

**(OQ1 resolved):** `ai-train` = **no**. If leadership flips this, update CF AI
Crawl Control + `app/robots.ts` (S1) in the same change.

**Disallow indexing (or noindex) candidates:**

| Path | Action |
|------|--------|
| `/docs` while stub | `robots: { index: false, follow: true }` **or** beef content first |
| API routes | Not in sitemap; disallow `/api/` in robots |
| Preview / staging hosts | Separate host rules if any (none today on apex) |

---

## URL inventory (canonical set) {#url-inventory}

**Origin:** `https://cuecloud.io` (apex only; www redirects).

| Path | Index? | Priority (sitemap) | Notes |
|------|--------|--------------------|-------|
| `/` | yes | 1.0 | Primary |
| `/architecture` | yes | 0.9 | System story |
| `/cuecode` | yes | 0.9 | IDE path |
| `/product/pricing` | yes | 0.9 | Commercial |
| `/compare` | yes | 0.9 | Seat vs metered (S7) |
| `/product/models` | yes | 0.8 | |
| `/product/api` | yes | 0.8 | |
| `/product/agents` | yes | 0.8 | |
| `/llms.txt` | yes (as file) | 0.5 | Short GEO brief (generated) |
| `/llms-full.txt` | yes (as file) | 0.4 | Full public brief (generated) |
| `/docs` | **no** until real docs **or** yes with substantial content | 0.3 | Prefer noindex stub |
| `/product/seats` | n/a | — | **301** → `/product/pricing` (already in `next.config.ts`) |

Do **not** put hash URLs (`/#paths`) in the sitemap. Internal anchors are fine
on-page for humans and Ask Cue.

---

## Phased delivery {#phases}

### Phase S0 — Edge + policy lock {#phase-s0}

**Owner:** whoever has Cloudflare + domain access.  
**Status: done 2026-07-13** (S0.3 residual verify after S1 `app/robots.ts` — see findings).

| # | Task | Exit criteria | Result |
|---|------|----------------|--------|
| S0.1 | Open CF → cuecloud.io → **AI Crawl / robots / content signals** | Notes: what overrides `robots.txt` today | **Done** — see [S0 findings](#s0-findings) |
| S0.2 | Decide crawl policy ([§ crawl-policy](#crawl-policy)) | Written decision in changelog | **Done** — policy LOCKED; OQ1 = `ai-train: no` |
| S0.3 | Ensure CF does not strip Next `Sitemap:` / `User-agent` rules after deploy | Live robots shows **standard rules** (+ optional content-signal lines) | **Done (S1.7)** — live robots is Next `User-Agent` / `Sitemap` / train-bot Disallows (CF no longer replaces with preamble-only) |
| S0.4 | Confirm apex is the only indexed host | www 301 only | **Done** — `https://www.cuecloud.io/` → **301** `https://cuecloud.io/` (CF); apex returns **200** (no host flip) |

**Verify (re-run anytime):**

```bash
curl -sS https://cuecloud.io/robots.txt
curl -sSI https://www.cuecloud.io/ | rg -i 'HTTP/|location'
# After S1 ships robots.ts — expect User-agent + Sitemap lines:
curl -sS https://cuecloud.io/robots.txt | rg -i 'user-agent|sitemap|disallow|allow|content-signal'
# Origin bypass (HTTP :80 via IP) — proves what Next/Caddy serve without CF HTML edge tricks:
curl -sS -D - -H 'Host: cuecloud.io' http://5.161.237.0/robots.txt -o /dev/null | head -15
```

#### S0 findings (2026-07-13) {#s0-findings}

| Check | Result |
|-------|--------|
| `GET https://cuecloud.io/robots.txt` | **200**, `server: cloudflare`, `content-type: text/plain`, ~1248 bytes |
| Body | Cloudflare **content-signal legal preamble only** — explains `search` / `ai-input` / `ai-train`. **No** `User-agent:`, **no** `Allow`/`Disallow`, **no** `Sitemap:`, **no** actual `Content-Signal: …` yes/no directive lines |
| Meaning of missing yes/no | Per CF’s own preamble rule (c): operator **neither grants nor restricts** via content signal for those uses — i.e. preamble alone is **not** an explicit allow/deny policy |
| Origin `GET http://<hetzner-ip>/robots.txt` (`Host: cuecloud.io`) | **404** Next not-found HTML (`Server: Caddy`, `X-Powered-By: Next.js`) — **no** `app/robots.ts` / `public/robots.txt` in the app yet |
| Edge vs origin | Cloudflare is **not** proxying an origin robots file; it is **substituting** a synthetic robots response when the path is `/robots.txt` (and/or when origin 404s). Behavior after we ship a real origin file is **unknown until S1.7** |
| `www` | **301** → `https://cuecloud.io/` (`server: cloudflare`) |
| Apex | **200**, no redirect to www |
| Terraform | Zone DNS only (`sites/landing/deploy/terraform/modules/cloudflare-dns`) — **cannot** manage AI Crawl / robots via current API token scope (see deploy README) |

#### Cloudflare dashboard checklist (human — do before or with S1) {#s0-cf-checklist}

API token today is DNS-capable only. In the Cloudflare dashboard for **cuecloud.io**:

1. **AI Crawl Control** (or equivalent “Content Signals” / bots UI):
   - **search** → allow / yes  
   - **ai-input** → allow / yes (GEO)  
   - **ai-train** → **block / no** (locked policy)
2. Prefer a mode that **merges or appends** origin `robots.txt` rather than **replacing** it entirely — so Next `User-agent` + `Sitemap:` survive.
3. After S1 deploy, re-fetch live robots and paste results into the changelog (S1.7).
4. Optional: confirm **Always Use HTTPS** + SSL **Full** still set (deploy README).

If CF cannot emit `ai-train: no` while still passing through Next robots, document the
limitation and keep explicit `Disallow` rules for known training bots in
`app/robots.ts` as a belt-and-suspenders measure (S1).

---

### Phase S1 — Crawl fundamentals (Next) {#phase-s1}

**Code home:** `sites/landing/apps/web/`  
**Status: done 2026-07-13** (S1.7 live verify after deploy).

| # | Task | Files (expected) | Exit criteria | Result |
|---|------|------------------|---------------|--------|
| S1.1 | Add `metadataBase` | `src/app/layout.tsx` + `src/lib/seo.ts` | `metadataBase: new URL("https://cuecloud.io")` | Done |
| S1.2 | Root defaults: title template, description, OG, Twitter, robots | `layout.tsx` | Absolute OG URLs; `twitter: summary_large_image` | Done (interim OG = brand mark until S2) |
| S1.3 | Canonicals per route | `page.tsx`, architecture, cuecode, docs, product | `alternates.canonical` | Done via `pageMetadata()` |
| S1.4 | `app/sitemap.ts` | `src/app/sitemap.ts` | Lists URL inventory (excludes `/docs`) | Done |
| S1.5 | `app/robots.ts` | `src/app/robots.ts` | Allow `/`; Disallow `/api/`; Sitemap; train-bot Disallow list | Done |
| S1.6 | `/docs` noindex while stub | `src/app/docs/page.tsx` | `robots: { index: false, follow: true }` | Done |
| S1.7 | Deploy + CF re-check | deploy script | Live sitemap 200; robots includes Sitemap / User-agent | **Done** — CF passes through Next robots (no longer preamble-only) |

**Verify:**

```bash
curl -sS -o /dev/null -w '%{http_code}\n' https://cuecloud.io/sitemap.xml
curl -sS https://cuecloud.io/robots.txt | rg -i 'user-agent|sitemap|disallow|allow'
curl -sS https://cuecloud.io/ | rg -i 'og:image|canonical|twitter:card'
```

---

### Phase S2 — Social previews + brand assets {#phase-s2}

**Status: done 2026-07-13**

| # | Task | Exit criteria | Result |
|---|------|---------------|--------|
| S2.1 | Create **OG image** 1200×630 | `public/og/cuecloud-og.png` | Done — generator `scripts/generate-og-image.py` / `npm run generate:og` |
| S2.2 | Wire `openGraph.images` + `twitter.images` | Meta points at `/og/cuecloud-og.png` with 1200×630 | Done via `OG_IMAGE_PATH` in `seo.ts` + layout |
| S2.3 | Optional: per-route OG titles | Shares not all identical | Done — `pageMetadata()` already sets unique title/description per route; shared brand OG image |
| S2.4 | Validate share preview | Image URL 200; meta tags present | Done after deploy (curl); FB/X debuggers optional scrape refresh |

---

### Phase S3 — Structured data (JSON-LD) {#phase-s3}

**Status: done 2026-07-13**

Emit JSON-LD via `JsonLd` + builders in `src/lib/seo.ts` — **not** confidential.

| # | Type | Where | Result |
|---|------|-------|--------|
| S3.1 | `Organization` | Home (+ inner pages) | Done |
| S3.2 | `WebSite` | Home | Done |
| S3.3 | `Product` + `SoftwareApplication` + Offer ($1500 USD) | Home; Product also on pricing | Done — price from `SEAT_PRICE_USD` |
| S3.4 | `FAQPage` | `/product/pricing` | Done — from `seatFaq()` |
| S3.5 | `BreadcrumbList` | Inner pages | Done |

**Rules still apply:** no fake ratings/availability; FAQ text must match UI.

---

### Phase S4 — GEO surface: `llms.txt` + full brief {#phase-s4}

**Status: done 2026-07-13**

| # | Task | Exit criteria | Result |
|---|------|---------------|--------|
| S4.1 | Add `public/llms.txt` (short) | 200 at `/llms.txt`; lists product one-liner, price, two doors, key URLs | **Done** — generated ~1.5 KB |
| S4.2 | Add `public/llms-full.txt` **or** generate from same source as Ask Cue | Longer factual brief; **no** confidential internals | **Done** — ~9 KB from same TS libs |
| S4.3 | Prefer **generate** from TS libs (like `npm run generate:site-md`) | Single source of truth; document command in `sites/landing/AGENTS.md` | **Done** — `npm run generate:llms` |
| S4.4 | Link `llms.txt` from robots comments or sitemap note (optional) | Discoverable | **Done** — both paths in sitemap |
| S4.5 | Cross-link from Ask Cue `SITE.md` page map | Assistant can point crawlers/humans to `/llms.txt` | **Done** |

**`llms.txt` content rules (v1):**

- Origin, product definition, price, paths (CueCode vs API), model names at public
  altitude, support email, canonical page list.
- Explicit: “Do not invent SLAs / pod BOM / internal code names.”
- Keep under ~2–4 KB for the short file; full file can mirror compressed `SITE.md`
  public sections.

---

### Phase S5 — On-page / IA hygiene {#phase-s5}

**Status: done 2026-07-13**

| # | Task | Exit criteria | Result |
|---|------|---------------|--------|
| S5.1 | One H1 per page; logical H2 hierarchy | Manual spot-check home, architecture, cuecode, pricing | **Done** — 1× H1 each; H2 sections follow |
| S5.2 | Internal links: footer/nav cover all inventory URLs | Already mostly true; fix gaps | **Done** — footer Product covers `/product/*`; Developers + `llms.txt` |
| S5.3 | Meaningful `alt` on **content** images; decorative marks stay empty | Policy documented | **Done** — `sites/landing/AGENTS.md` + BrandMark note |
| S5.4 | Ensure primary claims are in **HTML text**, not only canvas/CSS | Already true for hero; re-check animated stages | **Done** — stages `aria-hidden`; claims in ArchCopy / CueCode sections / hero |
| S5.5 | `/docs` stub decision executed (S1.6) | Live headers show noindex **or** real docs | **Done** — `noindex, follow` live |
| S5.6 | Strip marketing em-dash drift in meta descriptions where refreshing copy | Consistency with brand voice | **Done** — API `pageDescription` + pricing H2 / arch lead polish |

---

### Phase S6 — Search Console + monitoring {#phase-s6}

**Status: done 2026-07-13**

| # | Task | Exit criteria | Result |
|---|------|---------------|--------|
| S6.1 | Google Search Console property for `https://cuecloud.io` | Verified | **Done** — Domain property `cuecloud.io` |
| S6.2 | Submit sitemap | GSC shows sitemap fetched | **Done** — `sitemap.xml` Success; 9 pages discovered |
| S6.3 | Bing Webmaster Tools (optional but cheap) | Property + sitemap | **Skipped** — optional; GSC is enough for v1 |
| S6.4 | Baseline Lighthouse mobile (Perf / SEO / a11y) on `/` and `/product/pricing` | Scores recorded in changelog; fix regressions that block SEO | **Done** — home 90/100/100; pricing 48/100/100; no SEO blockers |
| S6.5 | Optional analytics: pageview + CTA (vendor TBD per 01) | Not blocking SEO pack | **Deferred** — vendor TBD; not blocking |

**Lighthouse mobile baseline (2026-07-13, live):**

| URL | Perf | SEO | A11y | Notes |
|-----|------|-----|------|-------|
| `/` | 90 | 100 | 100 | LCP ~3.3s |
| `/product/pricing` | 48 | 100 | 100 | High TBT (~2.4s) from client JS (contrast duel etc.); **not** an SEO meta failure |

Re-run: `cd sites/landing/apps/web && npm run lighthouse:baseline`

#### S6 owner checklist (GSC / Bing) {#s6-owner-checklist}

**Completed 2026-07-13** (founder Google account / OQ4):

1. ~~Open Google Search Console → Add property~~ ✓ Domain `cuecloud.io`
2. ~~Verify ownership~~ ✓
3. ~~Sitemaps → `https://cuecloud.io/sitemap.xml`~~ ✓ Success, 9 discovered pages
4. Bing Webmaster Tools — optional, skipped for v1
5. Analytics vendor — deferred (S6.5)

---

### Phase S7 — Content track (optional, post-technical) {#phase-s7}

**Status: done 2026-07-13** (technical pack + first content page)

| # | Task | Exit criteria | Result |
|---|------|---------------|--------|
| S7.1 | Comparison page: seat vs metered OpenAI/Anthropic | Live indexable URL; math from `seats.ts` only | **Done** — `/compare` |
| S7.2 | Architecture explainer as evergreen | Keep `/architecture` updated | **Done** — already shipping; no change this pass |
| S7.3 | CueCode page depth | Design Studio stays coming soon | **Done** — already shipping |
| S7.4 | Changelog / “what’s new” | Only if maintainable | **Skipped** — no changelog surface yet |

**`/compare`:** citation bait for coding-agent cost queries. Reuses `SeatVsMeteredBlocks` + `ContrastDuel`; prices from `seats.ts`. Nav + footer + sitemap + SITE.md / llms page maps updated.

Do **not** invent SLAs, latency floors, or rates outside `seats.ts`.

---

## Implementation map (code) {#impl-map}

| Concern | Location |
|---------|----------|
| Root metadata | `sites/landing/apps/web/src/app/layout.tsx` |
| Route metadata | `app/architecture/page.tsx`, `app/cuecode/page.tsx`, `app/docs/page.tsx`, `app/product/[slug]/page.tsx` |
| Sitemap | **Add** `app/sitemap.ts` |
| Robots | **Add** `app/robots.ts` |
| SEO helpers / JSON-LD | **Add** `src/lib/seo.ts`, `src/components/JsonLd.tsx` |
| OG asset | **Add** `public/og/…` |
| llms.txt | **Add** `public/llms.txt` (+ generator script optional) |
| Seat price source of truth | `src/lib/seats.ts` (reuse; do not hardcode a second price) |
| Site constants | `src/lib/site.ts` |
| Deploy | `sites/landing/deploy/scripts/deploy.sh` |
| Ask Cue brief | `sites/landing/apps/chat/SITE.md` via `npm run generate:site-md` |

---

## Metadata contract (per page) {#metadata-contract}

Every **indexable** page must export (directly or via layout merge):

```ts
{
  metadataBase: new URL("https://cuecloud.io"),
  title: string, // unique; use template "%s · Cue Cloud" for children
  description: string, // 150–160 chars target; unique; public facts only
  alternates: { canonical: string /* absolute or path */ },
  openGraph: {
    type: "website",
    url: string,
    siteName: "Cue Cloud",
    title: string,
    description: string,
    images: [{ url: string, width: 1200, height: 630, alt: string }],
  },
  twitter: {
    card: "summary_large_image",
    title: string,
    description: string,
    images: [string],
  },
  robots?: { index: boolean, follow: boolean },
}
```

**Home title (current, keep unless A/B):**  
`Cue Cloud · Flat seats, unlimited tokens`

**Home description must mention:** owned / open coding models, **$1,500** per seat,
unlimited tokens, CueCode **or** API.

---

## GEO citation targets (facts to keep consistent) {#geo-facts}

These strings should match across: on-page copy, JSON-LD, `llms.txt`, `SITE.md`,
and OG description.

| Fact | Canonical public wording |
|------|--------------------------|
| Product | Cue Cloud — owned coding inference on open models / Mac Studios we operate |
| Price | **$1,500/mo per seat · unlimited coding tokens** for that user |
| Paths | CueCode (density IDE + harness) **or** OpenAI-compatible API |
| Hardware (public) | Mac Studios we operate |
| Not claimed | tok/s SLAs, pod BOM, margins, internal scheduler names |

If pricing changes, update **`seats.ts` first**, then regenerate SITE.md / llms /
JSON-LD builders.

---

## Confidentiality {#confidentiality}

Same boundary as [01](./01-cuecloud-landing.md#confidentiality) and
[02](./02-site-assistant.md):

- Never put `CUECLOUD.md` into `llms.txt`, JSON-LD, or meta tags.
- Never invent competitive latency numbers for rich results.
- Design Studio may appear as **coming soon** only.

---

## Acceptance checklist (ship bar) {#acceptance}

Technical pack is **done** when all are true on production:

- [x] `GET /sitemap.xml` → 200; includes all indexable URLs; excludes `/api/*`
- [x] `GET /robots.txt` → includes `User-agent`, allow/disallow, and `Sitemap: https://cuecloud.io/sitemap.xml` (CF-compatible)
- [x] Home HTML includes `rel="canonical"`, `og:image`, `twitter:card=summary_large_image`
- [x] `/product/pricing` (and home) include valid Product/Offer JSON-LD; Rich Results Test clean of hard errors
- [x] `GET /llms.txt` → 200; price + paths + URL list
- [x] `/docs` noindex **or** non-thin content
- [x] www → apex 301 still works
- [x] Deployed via landing deploy script; Ask Cue `SITE.md` regenerated if page map gained `/llms.txt`
- [x] This spec changelog updated; marketing README lists this doc
- [x] GSC property + sitemap Success (S6)
- [x] `/compare` live (S7 content)

---

## Test plan (manual) {#test-plan}

1. **Curl matrix** — robots, sitemap, llms, home meta, www redirect (commands in S0/S1).
2. **View-source** home + pricing — H1 present; JSON-LD script present; no secrets.
3. **Rich Results Test** — home + pricing URLs.
4. **Social debuggers** — OG image renders; title not truncated into nonsense.
5. **GSC** — sitemap processed; no flood of “crawled / not indexed” on stubs.
6. **Ask Cue** — still answers price; can mention `/llms.txt` if in SITE.md.
7. **Regression** — Request access + intro call iframe still work; chat same-origin nav still works.

---

## Risks {#risks}

| Risk | Mitigation |
|------|------------|
| CF overwrites Next robots | S0 dashboard lock; test after every deploy |
| Duplicate price strings drift | Single builder from `seats.ts` |
| Over-claiming in JSON-LD | Review against confidentiality; no fake ratings |
| Indexing empty `/docs` | noindex until real |
| GEO “train: no” too aggressive | Revisit if partners need training allowance; input/citation still yes |
| OG image looks generic AI-slop | Design against Cue Cloud mark + existing dark theme tokens |

---

## Open questions {#open-questions}

| ID | Question | Decision / default |
|----|----------|---------------------|
| OQ1 | `ai-train` allow or deny? | **Denied (no)** — locked S0 2026-07-13 |
| OQ2 | Expand `/docs` vs noindex stub? | **noindex** until real docs |
| OQ3 | Generate `llms-full.txt` from SITE.md or hand-maintain short only? | Generate short + full from TS libs |
| OQ4 | Search Console account / owner? | Founder Google account; transfer to company later |
| OQ5 | Blog / comparison pages in v1? | **No** — S7 later |

---

## Build order (for implementers) {#build-order}

Say **`Build SEO/GEO phase S0`** … **`S6`** in chat when executing.

1. S0 policy + CF  
2. S1 sitemap/robots/metadataBase/canonicals/docs robots  
3. S2 OG image  
4. S3 JSON-LD  
5. S4 llms.txt (+ generator)  
6. S5 on-page hygiene  
7. S6 GSC + Lighthouse baseline  
8. S7 only if product asks for content expansion  

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-07-13 | **S7 done:** `/compare` seat vs OpenAI/Anthropic (math from `seats.ts`); shared `SeatVsMeteredBlocks`; nav/footer/sitemap/SITE.md/llms; changelog surface skipped |
| 2026-07-13 | **S6 done:** GSC Domain `cuecloud.io` verified; sitemap.xml Success (9 pages); Bing skipped; Lighthouse baseline recorded; analytics deferred |
| 2026-07-13 | **S6 partial:** Lighthouse mobile baseline recorded (home Perf 90 / SEO 100 / a11y 100; pricing Perf 48 / SEO 100 / a11y 100); `npm run lighthouse:baseline`; GSC+Bing owner checklist (OQ4); analytics deferred |
| 2026-07-13 | **S5 done:** H1 spot-check OK; footer + `llms.txt`; image alt policy in `sites/landing/AGENTS.md`; docs noindex confirmed; meta/on-page em-dash scrub on refreshed strings |
| 2026-07-13 | **S4 done:** `npm run generate:llms` → `public/llms.txt` + `llms-full.txt` from marketing libs; sitemap entries; SITE.md page map; documented in `sites/landing/AGENTS.md` |
| 2026-07-13 | **S3 done:** JSON-LD Organization/WebSite/Product/SoftwareApplication/FAQ/BreadcrumbList; `JsonLd` + builders in `seo.ts`; wired on home, pricing, and inner pages |
| 2026-07-13 | **S2 done:** `public/og/cuecloud-og.png` (1200×630), wired as OG/Twitter image; generator script + `npm run generate:og` |
| 2026-07-13 | **S1 done + live:** sitemap 200; robots pass-through from Next (User-Agent, Disallow /api/, train bots, Sitemap); canonical + og:image + twitter:summary_large_image on home; `/docs` noindex; S0.3 closed |
| 2026-07-13 | **S1 implemented:** `seo.ts`, `metadataBase`, canonicals, `sitemap.ts`, `robots.ts` (api + train bots), `/docs` noindex; interim OG = brand mark; S1.7 pending deploy verify |
| 2026-07-13 | **S0 done:** crawl policy LOCKED (search/ai-input allow, ai-train deny); CF robots = preamble-only substitute; origin `/robots.txt` 404; www→apex 301 confirmed; S0.3 residual verify deferred to S1.7; CF dashboard checklist added |
| 2026-07-13 | Initial SEO/GEO technical plan from production audit (missing sitemap, CF-only robots, no OG image/JSON-LD/llms.txt) |
