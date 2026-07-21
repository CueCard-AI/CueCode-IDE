# Cue Labs — company / lab site {#cuelabs}

> **Parent:** [marketing/README](./README.md) · Spec index: [00-README](../00-README.md)
>
> **Status:** Live on Hetzner + Cloudflare. GHA secrets (`CUELABS_*`) still optional for auto-deploy.
>
> **Live:** [`https://cuelabs.cloud`](https://cuelabs.cloud) · app: [`sites/labs/`](../../../sites/labs/)
>
> **Related:** [01-cuecloud-landing](./01-cuecloud-landing.md) (product site) ·
> CueGrowth (external) · investor OS rebuild (private, CueInference workspace):
> `.cursor/specs/growth/01-investor-os.md`

---

## One sentence {#one-sentence}

A quiet Anysphere-style lab site for **Cue Labs**: mission to make agents cheap, two product doors (**CueCloud** then **CueGrowth**), and a longer **Story** page — separate from the loud CueCloud marketing site.

---

## Goals and non-goals {#goals}

### Goals

1. Ship `cuelabs.cloud` + `www` → apex.
2. Home = mission + two quiet product lines + CTAs.
3. `/story` = longer lab narrative (same visual system).
4. Deploy on its own Hetzner box + Cloudflare zone (mirror `sites/landing/deploy` pattern).
5. Keep copy and visuals calm: serif, off-white, almost no chrome.

### Non-goals

- Ask Cue, waitlist, Supabase, analytics in v1.
- Cue Cloud dark theme / gradients / product marketing density.
- Colocating on the Cue Cloud landing VM (v1 = **new** small VM).
- Blog, full careers portal, or legal essays. `/hiring` may door to the
  [open challenge](./05-hiring-challenge.md); mailto remains the intake.

---

## Information architecture {#ia}

```
/                 ← mission + CueCloud + CueGrowth + Our story + Join
/story            ← founder story (GTM → token bill → CueCloud)
/hiring           ← hiring soon + door to challenge brief
/hiring/challenge ← Small Model, Big Harness public brief
```

Outbound:

| Label | URL |
|-------|-----|
| CueCloud | `https://cuecloud.io` |
| CueGrowth | `https://cuegrowth.ai` |
| Join our team | `/hiring` |
| Challenge brief | `/hiring/challenge` |
| Fixtures repo | `https://github.com/CueCard-AI/small-model-big-harness` |

Related spec: [05-hiring-challenge](./05-hiring-challenge.md).

---

## Locked copy {#copy}

**No em dashes** in public copy.

### Home

> *Cue Labs* is here to make agents cheap.
> The future of software is agents on every desk: writing code, running GTM, operating the business. The bottleneck isn’t capability. It’s cost: every tool call, every retry, every long session turning into another meter.
> We build products that remove that bottleneck, so agents can be infrastructure, not a tax.

Product lines (order fixed):

> **CueCloud** — flat-seat coding inference. Unlimited agent tokens for that user.
> **CueGrowth** — AI-native GTM that doesn’t price itself out of the loop.

CTAs: `Our story →` · `Join our team →`

### Story (`/story`)

> *Cue Labs* is here to make agents cheap.
>
> We started with go-to-market. We blew millions on sales leaders and expensive tools that did not move the number. So we built **CueGrowth** (→ cuegrowth.ai): a GTM stack meant to make selling easier without the same bloated spend.
>
> Then the bill moved. Agents and AI workflows pushed us past **$40K/mo** in token spend with Anthropic and OpenAI. Frontier labs kept raising the meter. The application layer got more capable and more expensive at the same time. That is the squeeze: you need agents everywhere, and usage-based inference punishes you for using them.
>
> So we built **CueCloud** (→ cuecloud.io): owned coding inference, flat seats, unlimited tokens for that user. **CueCode** / **API** link to cuecloud.io. Same thesis as CueGrowth: collapse the cost of the loop so agents can be infrastructure, not a tax.
>
> We are a small lab. We ship products. We keep the public story honest.
>
> Agents should be abundant.

Footer links: CueCloud · CueGrowth · Home · Join our team → `/hiring` · centered brand mark

### Hiring (`/hiring`)

> *Cue Labs* is hiring soon.
>
> We just closed a round of funding. More news is coming.
>
> We’re not hiring for leetcode. Make a 2-bit model look unfair. Same weights.
> The environments we ship. Show lift. Explain what still fails.
>
> Challenge brief → `/hiring/challenge` · Results / interest: hello@cuelabs.cloud

Links: Home · Our story · Challenge brief · centered brand mark

### Challenge brief (`/hiring/challenge`)

> *Small Model, Big Harness*
>
> Cracked-bar, open-ended: make a 2-bit GLM look unfair. Same weights (hosted
> endpoint preferred; BYO optional). Full multi-track kit (SWE + terminal + web
> + multimodal + rebench). Optional anonymous leaderboard + CueCode feel-test.
>
> Sections: Start here · The ask · Locked model · Tracks · Budgets · Score · Submit · Rubric
>
> Full contract: [05-hiring-challenge](./05-hiring-challenge.md) (phases M0–M4) ·
> fixtures: [CueCard-AI/small-model-big-harness](https://github.com/CueCard-AI/small-model-big-harness)

Links: Challenge repo · Hiring · Home · Our story

---

## Visual rules {#visual}

| Rule | Detail |
|------|--------|
| Background | Off-white paper wash + very light noise (not Cue Cloud black / no hero gradient) |
| Type | **Source Serif 4** via `next/font`; italic brand lead (*Cue Labs*); lead ~1.6–1.85rem |
| Hierarchy | Lead strong; supporting lines `para--muted` / `para--quiet`; primary CTA separate |
| Layout | Single left-aligned text column, large margins, top-third placement on home |
| Chrome | No marketing nav bar, no cards, no glow, no pill CTAs |
| Footer mark | Small centered mark |
| Motion | Lead fade/slide ~280ms; honor `prefers-reduced-motion` |

Brand test: remove any chrome and the first viewport still reads as Cue Labs.

---

## Repo placement {#repo-placement}

```
sites/labs/
  README.md
  AGENTS.md
  apps/web/          ← Next.js (standalone), port 3200 local
  deploy/
    terraform/       ← Hetzner VM + Cloudflare DNS
    docker/          ← Caddy + web
    scripts/deploy.sh
```

---

## Deploy plan {#deploy}

| Piece | Choice |
|-------|--------|
| Domain | `cuelabs.cloud` (zone in Cloudflare) |
| Origin | New Hetzner `cpx11` (Ashburn), name `cuelabs-site` |
| Remote dir | `/opt/cuelabs` |
| Edge | Cloudflare proxied A for `@` + `www`; SSL **Full** |
| Origin TLS | Self-signed in `deploy/docker/certs/` (gitignored) |
| CI | `.github/workflows/deploy-cuelabs.yml` on `sites/labs/**` → main |

### Human prerequisites (before first `terraform apply`)

1. Register / point NS for `cuelabs.cloud` to Cloudflare.
2. Create CF zone; copy Zone ID.
3. `sites/labs/deploy/.env.secrets` from `.env.secrets.example` (`HCLOUD_TOKEN`, `CLOUDFLARE_API_TOKEN`, zone id, SSH pubkey).
4. Generate origin certs (same approach as landing).
5. Dashboard: SSL Full, Always HTTPS.

### Phased delivery

| Phase | Work | Status |
|-------|------|--------|
| L0 | Spec + scaffold in repo | Done |
| L1 | Local site looks right (home + story) | Done |
| L2 | Cloudflare zone + Terraform apply | Done (`cuelabs-site`) |
| L3 | First deploy + www→apex | Done |
| L4 | GitHub Actions secrets + workflow | Workflow present; secrets TBD |
| L5 | Soft launch; optional OG image | Later |

Say **`Build Cue Labs phase L1`** … **`L4`** when executing.

---

## Confidentiality {#confidentiality}

Same public altitude as Cue Cloud marketing: no pod SKUs, tok/s floors, margins, scheduler internals. Story page may describe flat seats and owned hardware in the same language as `cuecloud.io`.

---

## Acceptance {#acceptance}

- [x] Spec in marketing index; monorepo lists `sites/labs/`
- [x] `npm run dev` serves `/` and `/story` with locked copy
- [x] Terraform + deploy scripts present and applied
- [x] CI workflow file present (secrets may be empty until L4)
- [x] Live `https://cuelabs.cloud` after L3

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-07-20 | App path → **`sites/labs/`** (monorepo layout H2). |
| 2026-07-20 | Live cutover: Hetzner `cuelabs-site` + CF DNS; reuse landing SSH key |
| 2026-07-19 | Visual polish A: Source Serif 4, lead hierarchy, paper wash, CTA separation |
| 2026-07-19 | `/hiring/challenge` on-site brief; `/hiring` stays a door |
| 2026-07-19 | `/hiring` doors to Small Model, Big Harness ([05](./05-hiring-challenge.md)) |
| 2026-07-19 | Story rewrite (GTM → $40K/mo → CueCloud); `/hiring`; footer mark on all pages |
| 2026-07-19 | Initial Cue Labs plan + scaffold (`sites/labs/`, deploy stub, locked copy) |
