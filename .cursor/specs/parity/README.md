# Competitive parity program {#parity-index}

Claude Code harness parity specs — program charter, E2E flows, and gap docs for memory, teams, commands, platform, and AI surfaces.

**Invoke build work:** [build-plans/00-master-build-plan](../delivery/build-plans/00-master-build-plan.md) (`Build phase X.Y`)

---

## Reading order

1. [15-competitive-parity](./15-competitive-parity.md) — program thesis, phases P0–P4, Competitive 1.0 gate
2. [research/00-claude-code-inventory](../research/00-claude-code-inventory.md) — tool/command inventory
3. [research/01-parity-decisions](../research/01-parity-decisions.md) — Adopt / Adapt / Defer / Reject policy
4. [16-end-to-end-flows](./16-end-to-end-flows.md) — acceptance flows A–H (Gherkin)
5. Gap specs (as needed):
   - [17-memory-and-context](./17-memory-and-context.md)
   - [18-teams-and-tasks](./18-teams-and-tasks.md)
   - [19-command-surface](./19-command-surface.md)
   - [20-platform-integrations](./20-platform-integrations.md)
   - [21-ai-surfaces](./21-ai-surfaces.md)

---

## When implementing

| Task | Load |
|------|------|
| Match CC tool behavior | [research/00-inventory](../research/00-claude-code-inventory.md) row + [08-agent-tools](../agent/08-agent-tools-and-skills) |
| Slash command parity | [19-command-surface](./19-command-surface.md) |
| Async / verify / lanes | [16 flows](./16-end-to-end-flows.md) + [harness/local](../harness/local/01-agent-harness.md) |
| Memory / compact | [17-memory-and-context](./17-memory-and-context.md) |
| Coordinator / tasks | [18-teams-and-tasks](./18-teams-and-tasks.md) |
| Reject bridge/plugins | [20-platform-integrations](./20-platform-integrations.md) |
| Agent vs Copilot split | [21-ai-surfaces](./21-ai-surfaces.md) |

---

## Related tracks

| Track | Path |
|-------|------|
| Execution (Build phase X.Y) | [build-plans/](../delivery/build-plans/README.md) |
| Harness semantics | [harness/](../harness/README.md) |
| Core product specs | [00-README](../00-README.md) (`core/`, `delivery/`, `agent/`, `design/`, `ops/`) |
