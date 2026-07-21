# Cue Cloud — site assistant spec {#cuecloud-site-assistant}

> **Parent:** [marketing/README](./README.md) · Spec index: [00-README](../00-README.md)
>
> **Status:** product + UX + engineering contract for the public **site
> assistant** on [`cuecloud.io`](https://cuecloud.io) (landing app).
>
> **Related:** [01-cuecloud-landing](./01-cuecloud-landing.md) (site IA +
> confidentiality) · deploy: [`sites/landing/deploy/`](../../../sites/landing/deploy/) ·
> private architecture: [`CUECLOUD.md`](../../../CUECLOUD.md) (do **not** feed
> to the assistant).

---

## One sentence {#one-sentence}

A small on-site chat widget answers visitor questions from a **single curated
markdown brief**, can **link deeper into the site**, and runs as a **separate
backend service** on the same Hetzner host as the Next.js landing app — powered
by a cheap Gemini Flash-Lite class model.

---

## Goals and non-goals {#goals}

### Goals (v1)

1. Help visitors understand Cue Cloud (seats, Mac Studios, paths, architecture
   summary) without hunting the whole site.
2. **Source of truth = one markdown file** loaded into the model context (no
   vector DB / RAG in v1).
3. When more depth exists on-site, **reply with links** to the right page
   (`/architecture`, `/product/pricing`, etc.).
4. Separate **`apps/chat`** service; deploy next to `web` + Caddy on the
   existing landing VM.
5. Stay inside the [landing confidentiality boundary](./01-cuecloud-landing.md#confidentiality)
   — assistant must not invent SKUs, SLAs, or white-paper economics.

### Non-goals (v1)

- True RAG / embeddings / chunk index.
- Scraping live HTML at runtime.
- Agent tools, browsing, or writing waitlist rows for the user.
- Replacing `/docs` or human sales (intro call / Request access).
- Training or fine-tuning; multi-model routing.
- Indexing `CUECLOUD.md`, Terraform, or investor decks.

### Later (explicitly out of v1)

- RAG if `SITE.md` exceeds practical context or answers go stale often.
- Logged analytics dashboard for unanswered questions.
- Authenticated “logged-in customer” mode.

---

## User journeys {#journeys}

### Happy path

1. Visitor opens any marketing page → sees collapsed chat chip (bottom-right,
   clear of theme chip bottom-left).
2. Opens panel → types a question (e.g. “Why Mac Studios?”).
3. Streams an answer grounded in `SITE.md`, with a link to `/architecture`
   when relevant.
4. Can ask follow-ups in the same thread (short history window).

### Unhappy paths

| Case | Behavior |
|------|----------|
| Question outside `SITE.md` | Say so plainly; offer Request access / `/docs` / support email — **no fabrication** |
| Model / upstream error | Inline error + retry; never show raw API keys or stack traces |
| Rate limited | Calm message; suggest browsing Product / Architecture |
| Empty / spam input | Client validation; ignore empty submits |
| `prefers-reduced-motion` | No gratuitous motion; panel still usable |

---

## Knowledge: `SITE.md` {#knowledge}

### Role

`SITE.md` is the **only** factual corpus for the assistant. Site pages remain
the long-form UX; the markdown is a **digest + link map** for the model.

### Path (locked)

```
sites/landing/apps/chat/SITE.md
```

**Generated** from public marketing libs — do not hand-edit. After changing
models/pricing/architecture copy under `sites/landing/apps/web/src/lib/`:

```bash
cd sites/landing/apps/web && npm run generate:site-md
```

Generator: `sites/landing/apps/web/scripts/generate-site-md.ts`.

Loaded at process start (and optionally on `SIGHUP` / admin `POST /reindex`
that re-reads the file — no embedding step).

### Required sections (v1 outline)

Authors keep this file short and factual. Suggested headings:

1. **What Cue Cloud is** — open coding models on Mac Studios we operate; flat
   seat; two paths (CueCode / API).
2. **Pricing** — `$1,500/mo per seat · unlimited tokens` + link
   `/product/pricing`.
3. **Mac Studios / why** — memory-bound MoE decode; unified memory; link
   `/architecture#studios` (or equivalent anchors).
4. **How a request works** — OpenAI-compatible → scheduler → Studio pod →
   stream; link `/architecture`.
5. **Build vs buy** — buying Studios ≠ serving stack; link `/architecture#build`.
6. **CueCode vs API** — links `/#paths`, CueCode GitHub, `/product/api`.
7. **Access** — Request access waitlist; support `support@cuecloud.io`;
   dashboard TBD.
8. **Docs** — stub status + link `/docs`.
9. **Hard rules for the model** — bullet list of “never invent…” (mirrored in
   system prompt).

### Link rules

- Prefer **absolute production URLs** in `SITE.md`
  (`https://cuecloud.io/architecture`) so answers work in any client.
- Also list path-only forms for local dev (`http://localhost:3100/...`) only in
  a “Local” footnote — or inject `PUBLIC_SITE_ORIGIN` when building the system
  prompt so one file works in both envs.

### Confidentiality

Same table as landing 01. If it must not appear on the public site, it must not
appear in `SITE.md`.

---

## Model and prompting {#model}

| Item | v1 choice |
|------|-----------|
| Provider | Google Gemini (AI Studio API key) |
| Model | Flash-Lite class (pin exact model ID in env, e.g. `GEMINI_MODEL=…`) — cheapest suitable chat model at ship time |
| Temperature | Low (≈0.2–0.4) |
| Max output tokens | Cap (e.g. 512–1024) |
| History | Last N turns only (e.g. 8) + full `SITE.md` each request |

### System prompt (contract)

Must include:

1. Role: Cue Cloud site assistant for developers / buyers.
2. Answer **only** from `SITE.md` content provided below.
3. Prefer **links from SITE.md** when the user wants more detail.
4. Never invent pricing, tok/s SLAs, pod BOM, or internal code names.
5. If unknown → admit + suggest Request access or the closest page link.
6. Tone: direct, technical, short — match architecture page voice, not hype.

`SITE.md` body is appended (or injected) as the knowledge block each call.

---

## Backend {#backend}

### Service (locked)

| | |
|--|--|
| Path | `sites/landing/apps/chat/` |
| Runtime | **Python + FastAPI** (OQ-2 locked) |
| Port | `8100` internal (Docker network only — not published on host) |
| Health | `GET /health` → `{ "ok": true }` |
| Chat | `POST /v1/chat` (SSE) — public path via Caddy: `/api/chat/v1/chat` |

Public URL prefix (locked): **`/api/chat`**

| Browser calls | Caddy strips/forwards | Container |
|---------------|----------------------|-----------|
| `POST /api/chat/v1/chat` | → `chat:8100/v1/chat` | FastAPI |
| `GET /api/chat/health` | → `chat:8100/health` | FastAPI |

Use Caddy `handle_path /api/chat/*` (or equivalent) so the chat app does **not**
need to know the `/api/chat` prefix. Document the exact Caddyfile block in
[deploy](#deploy).

### API

#### `POST /v1/chat`

Request:

```json
{
  "messages": [
    { "role": "user", "content": "Why Mac Studios?" }
  ],
  "session_id": "optional-client-uuid"
}
```

Response (v1, locked): **SSE** text deltas; terminal event `{ "done": true }`.

#### `GET /health`

Liveness for compose / smoke tests (mapped publicly as `/api/chat/health`).

#### Optional `POST /v1/reload`

Re-read `SITE.md` from disk. Auth: `Authorization: Bearer $CHAT_RELOAD_TOKEN`.
Nice-to-have (A4).

### Env (chat container)

| Var | Purpose |
|-----|---------|
| `GEMINI_API_KEY` | Provider key (server only) |
| `GEMINI_MODEL` | Pinned model id |
| `PUBLIC_SITE_ORIGIN` | `https://cuecloud.io` (link base for answers) |
| `CHAT_RATE_LIMIT_PER_HOUR` | Default `30` |
| `CHAT_RELOAD_TOKEN` | Optional reload auth |
| `SITE_MD_PATH` | Default `/app/SITE.md` inside the image |

Never expose these to the Next.js client bundle (`NEXT_PUBLIC_*` forbidden for
Gemini).

### Rate limiting / abuse

- Per client IP (`CF-Connecting-IP` or `X-Forwarded-For` behind Cloudflare) +
  optional `session_id`.
- Reject oversized messages (e.g. >2k chars) and oversized histories.
- No Turnstile in v1; add if abused.

---

## Frontend {#frontend}

### Widget

- Mounted from root marketing layout / shell so it appears on **all** public
  pages (home, product, architecture, docs).
- Component path: `sites/landing/apps/web/src/components/ChatWidget.tsx` (+ CSS in
  `globals.css` or co-located module).
- Bottom-right floating chip (**Ask Cue**) → expands to a larger panel.
- Desktop: drag the header to move; **Dock** pins a full-height right rail;
  **Float** undocks. Prefs (layout + position) persist in `localStorage`.
- Mobile (≤720px): opens full-viewport; no free drag / dock toggle.
- Theme-aware (CSS vars). Do **not** cover theme chip (bottom-left).
- Accessible: focus trap when open, Escape closes, `aria-expanded`.

### Client → server

Browser calls **same-origin** only:

```
POST /api/chat/v1/chat
POST /api/chat/v1/followups
GET  /api/chat/health
```

Caddy terminates TLS and reverse-proxies to `chat:8100`. **Do not** put the
Gemini key in Next Route Handlers unless we later choose a BFF — v1 locked to
Caddy → chat.

### Copy

- Chip label: **Ask Cue**
- Panel title / `aria-label`: **Ask Cue**
- Subtitle: seats / Mac Studios / stack (not “site brief”)
- Modes: float (default, larger ~32×70vh) · dock right · collapsed chip
- Drag: header handle in float mode only
- Persist UI: `cuecloud-ask-cue-ui` (`layout`, `pos`)
- Persist transcript (tab session): `cuecloud-ask-cue-transcript` + **Clear**
- **Email gate** before chat: domain MX + disposable/junk/typo checks; lead →
  Supabase `ask_cue_leads`; HMAC `gate_token` required on `/v1/chat` +
  `/v1/followups`
- Assistant bubbles: render markdown (`react-markdown`)
- Follow-ups: LLM via `/v1/followups` after each answer (seeds until then)
- Empty state: “Ask about seats, Mac Studios, or how the stack runs.”
- Suggested prompts: pricing · why Mac Studios · CueCode vs API

---

## Repo layout {#layout}

Canonical tree after implementation (paths relative to monorepo root):

```
sites/landing/
  apps/
    web/                                    # existing Next.js marketing site
      Dockerfile                            # copied into context by deploy
      src/
        app/layout.tsx                      # mount <ChatWidget />
        components/
          ChatWidget.tsx                    # NEW — floating assistant UI
        …
    chat/                                   # NEW — site assistant service
      SITE.md                               # knowledge brief + deep links
      Dockerfile
      pyproject.toml                        # or requirements.txt
      README.md
      src/
        __init__.py
        main.py                             # FastAPI app: /health, /v1/chat
        prompt.py                           # system prompt + SITE.md load
        gemini.py                           # Gemini client + SSE
        rate_limit.py                       # IP / session limits
        config.py                           # env parsing
  deploy/
    docker/
      docker-compose.yml                    # web + chat + caddy
      Caddyfile                             # /api/chat* → chat; else → web
      Dockerfile                            # Next/web image (existing)
      .env.production                       # gitignored — Supabase + Gemini
      certs/                                # gitignored — origin TLS
    scripts/
      deploy.sh                             # rsync web + chat + compose up
      remote-rebuild.sh
    terraform/                              # unchanged — VM + DNS
    .env.secrets                            # gitignored — Hetzner/CF/SSH
    .env.secrets.example
    README.md
  …

.github/workflows/
  deploy-landing.yml                        # sync sites/landing/** → Hetzner
```

### What lives where

| Artifact | Location | Notes |
|----------|----------|-------|
| Knowledge | `apps/chat/SITE.md` | Baked into chat image / bind-mounted; **not** in Next bundle |
| Gemini key | server `.env.production` / GH secrets | Chat container only |
| Widget | `apps/web` | Talks to `/api/chat/*` |
| Routing | `deploy/docker/Caddyfile` | Path-based split |
| Host path | `/opt/cuecloud-landing/` | Existing landing deploy root |

### Local layout expectation

```
/opt/cuecloud-landing/          # production on Hetzner
  apps/web/
  apps/chat/
  deploy/docker/
    docker-compose.yml
    Caddyfile
    .env.production
    certs/
```

---

## Deploy {#deploy}

Same Hetzner VM as the landing site. Ops overview:
[`sites/landing/deploy/README.md`](../../../sites/landing/deploy/README.md).

### Runtime topology

```
Internet
  → Cloudflare (proxied, SSL Full)
  → origin :443 (Caddy, self-signed origin cert)
       ├─ host www → 301 https://cuecloud.io{uri}
       ├─ /api/chat/*  →  chat:8100   (handle_path strips /api/chat)
       └─ /*           →  web:3100
```

Chat is **not** published on the host network — only reachable via Caddy on the
compose network.

### Caddyfile (target)

Lock this shape (adapt to existing `cuecloud.io` / `:80` blocks):

```caddy
cuecloud.io, www.cuecloud.io {
	tls /etc/caddy/certs/origin.pem /etc/caddy/certs/origin.key
	@www host www.cuecloud.io
	redir @www https://cuecloud.io{uri} permanent

	handle_path /api/chat/* {
		reverse_proxy chat:8100
	}
	handle {
		reverse_proxy web:3100
	}
}
```

`:80` block mirrors the same `handle_path` / `handle` split (HTTP for CF Full
edge cases / health).

### docker-compose (target)

Extend existing `web` + `caddy` with:

```yaml
services:
  web:
    # existing — Next standalone :3100
    …

  chat:
    build:
      context: ../../apps/chat
      dockerfile: Dockerfile
    env_file:
      - .env.production
    environment:
      SITE_MD_PATH: /app/SITE.md
      PUBLIC_SITE_ORIGIN: https://cuecloud.io
    expose:
      - "8100"
    restart: unless-stopped

  caddy:
    # existing ports 80/443 + certs volume
    depends_on:
      - web
      - chat
```

`SITE.md` is `COPY`’d in the chat `Dockerfile` (rebuild to pick up knowledge
edits). Optional bind-mount for hot reload on the server is A4 only.

### `.env.production` (server)

Gitignored. Contains waitlist **and** chat vars:

```bash
# existing
SUPABASE_URL=…
SUPABASE_SERVICE_ROLE_KEY=…

# chat
GEMINI_API_KEY=…
GEMINI_MODEL=…                 # pin Flash-Lite-class id at ship
PUBLIC_SITE_ORIGIN=https://cuecloud.io
CHAT_RATE_LIMIT_PER_HOUR=30
```

### Manual deploy (`deploy.sh`)

Extend [`sites/landing/deploy/scripts/deploy.sh`](../../../sites/landing/deploy/scripts/deploy.sh):

1. Keep existing web rsync + docker/Caddy/certs sync.
2. **Also** rsync `sites/landing/apps/chat/` → `/opt/cuecloud-landing/apps/chat/`
   excluding `__pycache__`, `.venv`, `.env`.
3. Ensure remote dirs: `apps/web`, `apps/chat`, `deploy/docker/certs`.
4. Merge Gemini vars into `.env.production` (from local secrets or CI).
5. `docker compose build --pull && docker compose up -d --remove-orphans --force-recreate`
6. Smoke web **and** chat (below).

Do **not** `rsync --delete` on `deploy/docker/` (preserves certs / env).

### GitHub Actions

Extend [`.github/workflows/deploy-landing.yml`](../../../.github/workflows/deploy-landing.yml):

| Change | Detail |
|--------|--------|
| mkdir | Also `/opt/cuecloud-landing/apps/chat` |
| rsync | Sync `sites/landing/apps/chat/` (exclude venv / pyc) |
| `.env.production` | Append `GEMINI_API_KEY`, `GEMINI_MODEL`, `PUBLIC_SITE_ORIGIN`, rate limit |
| compose | Build/up already picks up new `chat` service once compose file is synced |
| smoke | Add `curl -fsS https://cuecloud.io/api/chat/health` |

#### New Actions secrets

| Secret | Purpose |
|--------|---------|
| `LANDING_GEMINI_API_KEY` | → `GEMINI_API_KEY` on server |
| `LANDING_GEMINI_MODEL` | → `GEMINI_MODEL` (optional if defaulted in compose) |

Existing: `LANDING_HOST`, `LANDING_USER`, `LANDING_SSH_KEY`, Supabase secrets.

### Failure mode

| Failure | Expected |
|---------|----------|
| `chat` container down | Marketing pages still 200; widget shows “Assistant unavailable” |
| Gemini 4xx/5xx | Stream/error in panel; no stack traces to client |
| Missing `SITE.md` | Chat fails health or returns 503 — do not start answering empty |

### Smoke checklist (prod)

```bash
curl -fsS -o /dev/null -w "%{http_code}\n" https://cuecloud.io/
curl -fsS https://cuecloud.io/api/chat/health
# optional chat probe (expect SSE / 200):
curl -fsS -N -X POST https://cuecloud.io/api/chat/v1/chat \
  -H 'content-type: application/json' \
  -d '{"messages":[{"role":"user","content":"What is the seat price?"}]}'
```

### Local bring-up

```bash
cd sites/landing/deploy/docker
# .env.production with Gemini + PUBLIC_SITE_ORIGIN=http://localhost:3100
docker compose up --build
# site http://localhost:3100  ·  chat via Caddy http://localhost/api/chat/health
```

Alternatively run Next on `:3100` and chat on `:8100` with a local Caddyfile —
compose is the source of truth for “prod-like.”

---

## Build phases {#phases}

### A0 — Spec (this doc)

- [x] Goals, knowledge file, API, UI locked
- [x] Full [repo layout](#layout) + [deploy](#deploy) (compose, Caddy, GHA, smoke)

### A1 — Knowledge + backend

- [x] Author v1 `SITE.md` from public landing facts + links
- [x] Scaffold `sites/landing/apps/chat` (FastAPI, Dockerfile, health + SSE chat)
- [x] Load `SITE.md` into system prompt; Gemini Flash-Lite call
- [x] Rate limit; compose + Caddy `/api/chat/*`

### A2 — Widget

- [x] `ChatWidget` on all marketing pages
- [x] SSE client → `POST /api/chat/v1/chat`
- [x] Suggested prompts + markdown link rendering

### A3 — Prod

- [x] Extend `deploy.sh` + `deploy-landing.yml` (rsync chat, Gemini secrets)
- [x] Caddyfile `handle_path /api/chat/*`
- [ ] Smoke `/api/chat/health` on cuecloud.io (needs `LANDING_GEMINI_*` secrets + deploy)
- [ ] Manual QA: pricing, Studios, unknown question

### A4 — Polish (optional)

- [ ] Reload endpoint
- [ ] Unanswered-question log (no PII)
- [ ] Turnstile if abuse appears

---

## Acceptance criteria {#acceptance}

- [ ] Assistant answers “What is the seat price?” from `SITE.md` with
      `$1,500` and a link to pricing.
- [ ] Assistant answers “Why Mac Studios?” with unified-memory style facts and
      a link to `/architecture`.
- [ ] Out-of-corpus question does **not** invent pod SKUs / SLAs.
- [ ] Gemini key never appears in client JS or public repo.
- [ ] Chat runs as its own container on the landing host; site still works if
      chat is down (widget shows error, pages load).
- [ ] No RAG / vector store in v1.

---

## Open questions {#oq}

| ID | Question | Resolution |
|----|----------|------------|
| OQ-1 | Exact Gemini model id at ship | **Locked: `gemini-3.1-flash-lite`** |
| OQ-2 | FastAPI vs Node for `apps/chat` | **Locked: FastAPI** |
| OQ-3 | SSE vs single JSON reply | **Locked: SSE** |
| OQ-4 | Persist transcripts? | **No** (v1 memoryless beyond client session) |
| OQ-5 | Public URL prefix | **Locked: `/api/chat`** → Caddy `handle_path` → `chat:8100` |

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-07-13 | Implemented A1–A2 + deploy wiring: `apps/chat`, `ChatWidget`, Caddy `/api/chat`, GHA Gemini secrets. |
| 2026-07-13 | Expanded [layout](#layout) + [deploy](#deploy): full tree, Caddy `handle_path`, compose, deploy.sh/GHA, secrets, smoke, failure modes. Locked FastAPI + `/api/chat` + SSE. |
| 2026-07-13 | Initial site assistant spec — `SITE.md` context, no RAG, separate chat service on landing VM, Gemini Flash-Lite class, deep links into site. |
