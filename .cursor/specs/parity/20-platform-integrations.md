# Platform integrations {#platform-integrations}

> **Program:** [15-competitive-parity](./15-competitive-parity.md) · **CC:** bridge, remote, plugins, cron, voice, GitHub  
> **Inventory:** [00 §services](../research/00-claude-code-inventory.md#services)

Platform-level Claude Code capabilities — adopt where CueCode is the IDE; defer or reject where CC is CLI-centric.

---

## IDE native (replaces bridge) {#ide-native}

**CC analog:** `bridge/`, `/ide`, `/bridge`.

**CueCode decision:** **Adapt** — no external bridge. Editor selection, diagnostics, and diffs are **native GPUI**.

| CC bridge feature | CueCode native |
|-------------------|----------------|
| Selection context | Editor selection → composer attach |
| Permission callbacks | GPUI confirm modal ([08 §permissions](../agent/08-agent-tools-and-skills#permissions)) |
| Diff review | Unified review + multi_buffer |
| LSP context | Project LSP + agent tools |

**Reject:** `/ide`, `/bridge` as user commands ([19 §rejected](./19-command-surface.md#rejected-commands)).

---

## Remote sessions {#remote-sessions}

**CC analog:** `remote/`, `/remote-setup`, RemoteSessionManager.

**Decision:** **Defer** Competitive 1.0.

| Mode | Target |
|------|--------|
| Cloud session resume | Opt-in CHP session restore [cloud/02](../harness/cloud/02-architecture.md) |
| Remote dev container | Align with Zed `dev_container` — post-v1 |

---

## Session share & export {#session-share}

**CC analog:** `/share`, `/export`.

| Feature | Decision | Phase |
|---------|----------|-------|
| Export bundle (transcript, sidechains, verdicts) | **Adapt** | 4 |
| Public share link | **Defer** | 6+ (privacy review) |

Bundle format: JSON manifest + file attachments under user control.

---

## Scheduled agents {#scheduled-agents}

**CC analog:** `ScheduleCronTool` (Create/List/Delete).

**Decision:** **Defer** Phase 5+ ([16 flow F](./16-end-to-end-flows.md#flow-f-scheduled)).

| Component | Location |
|-----------|----------|
| Cron store | `~/.config/cuecode/cron/` |
| Trigger | Local scheduler or cloud harness |
| Execution | Headless Async session |
| Delivery | Notification rail |

Teammate-scoped crons (CC routes to agent queue): map to **lane_id** in [18](./18-teams-and-tasks.md).

---

## Webhooks {#webhooks}

**CC analog:** `RemoteTriggerTool`.

**Decision:** **Defer** Phase 6+.

CI webhook → inject `session-notification` on parent thread (Async → Hybrid).

---

## Plugins {#plugins}

**CC analog:** `plugins/`, marketplace commands.

**Decision:** **Reject** v1 ([01-vision §non-goals](../core/01-vision#non-goals)).

**Instead:** MCP servers + `.cursor/skills/` + Cursor hooks ([20 §hooks](#hooks)).

---

## Hooks {#hooks}

**CC analog:** `/hooks`, Claude hooks config.

**CueCode:** **Adapt**

| Hook type | Mechanism |
|-----------|-----------|
| User automation | Cursor `.cursor/hooks/` |
| Product stop hooks | `agent::Thread` post-turn ([17 §stop-hooks](./17-memory-and-context.md#stop-hooks)) |
| Format on save | Editor + optional hook script |

---

## Voice {#voice}

**CC analog:** `/voice`, `voice/`.

**Decision:** **Defer** post-Beta.

If shipped: Active composer input only; no Async voice agents v1.

---

## GitHub & PR {#github}

**CC analog:** `/pr_comments`, `install-github-app`.

| Feature | Decision | Phase |
|---------|----------|-------|
| PR comments in agent | **Defer** | 6+ |
| GitHub App install flow | **Defer** | 6+ |
| Basic git in agent | **Adopt** | 0 (Zed git) |

---

## Notebooks {#notebooks}

**CC analog:** `NotebookEditTool`.

**Decision:** **Defer** Phase 6+ unless Jupyter workflow is dogfood-critical.

---

## Proactive mode {#proactive-mode}

**CC analog:** `SleepTool`, proactive briefs.

**Decision:** **Reject** SleepTool; **Adapt** proactive notifications via high-signal `SessionNotificationKind::ProactiveBrief` only ([local §B.4](../harness/local/01-agent-harness.md#b-4-proactive)).

---

## Statusline {#statusline}

**CC analog:** `statuslineSetupAgent`, `/statusline`.

**Decision:** **Defer** — use Cursor status line skill + CueCode session chips in agent header.

---

## Desktop / mobile handoff {#handoff-reject}

**CC analog:** `/desktop`, `/mobile`.

**Decision:** **Reject** v1 — CueCode is desktop-native IDE.

---

## Security & diagnostics {#diagnostics}

| CC | CueCode |
|----|---------|
| `/doctor` | CueCode Doctor palette → [10 runbooks](../ops/10-infrastructure#operational-runbooks) |
| `/status` | Agent header status + cloud connection |
| `/feedback` | Defer feedback form |
| `/security-review` | [16 flow G](./16-end-to-end-flows.md#flow-g-security-review) |

---

## Document status {#status}

| Field | Value |
|-------|-------|
| Status | Draft — gap spec |
| Last updated | 2026-06-17 |
