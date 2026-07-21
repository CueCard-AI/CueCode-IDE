# Marketing specs {#marketing-index}

Public product marketing contracts — landing pages, positioning, and copy that
ship outside the CueCode IDE binary.

| Doc | Topic |
|-----|-------|
| [01-cuecloud-landing](./01-cuecloud-landing.md) | **Cue Cloud** marketing home — brand, IA, sections, copy, visual rules, funnel phases · includes [`/architecture`](./01-cuecloud-landing.md#architecture-page) public system story |
| [02-site-assistant](./02-site-assistant.md) | On-site chat assistant — `SITE.md` knowledge, Gemini Flash-Lite, separate `apps/chat` on landing VM (no RAG v1) |
| [03-seo-geo](./03-seo-geo.md) | **SEO + GEO** technical plan — sitemap, robots/CF, canonicals, OG, JSON-LD, `llms.txt`, phased acceptance |
| [04-cuelabs](./04-cuelabs.md) | **Cue Labs** company site (`cuelabs.cloud`) — mission, CueCloud + CueGrowth doors, `/story`, `/hiring`, Hetzner/CF deploy |
| [05-hiring-challenge](./05-hiring-challenge.md) | **Small Model, Big Harness** — cracked-bar open-ended multi-track kit (2-bit GLM + in-repo benches) |

**CueGrowth product (not marketing copy):** investor OS rebuild lives in the
CueInference workspace only at `.cursor/specs/growth/01-investor-os.md`
(excluded from GPL `sync-cursor-config` — same privacy class as root `CUECLOUD.md`).

**Implementation:** Cue Cloud marketing → [`sites/landing/`](../../../sites/landing/) ([01 § repo placement](./01-cuecloud-landing.md#repo-placement)). Cue Labs → [`sites/labs/`](../../../sites/labs/) ([04](./04-cuelabs.md)).

**SEO/GEO implementers:** start at [03-seo-geo](./03-seo-geo.md); say `Build SEO/GEO phase S0` (through S6).

**Cue Labs implementers:** start at [04-cuelabs](./04-cuelabs.md); say `Build Cue Labs phase L1` (through L4).

**Hiring challenge implementers:** start at [05-hiring-challenge](./05-hiring-challenge.md); say `Build SMBH phase M1` (through M4).

**Architecture (private, not synced via this tree):** monorepo root
[`CUECLOUD.md`](../../../CUECLOUD.md).

**Related:** [design/](../design/README.md) (GPUI / in-app UX) ·
[harness/cloud/](../harness/cloud/README.md) (agent runtime) ·
[00-README](../00-README.md)
