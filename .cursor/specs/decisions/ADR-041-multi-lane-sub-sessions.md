# ADR-041: One parent AcpThread with sub-sessions for multi-lane v1

**Status:** Accepted (2026-06-17; pending full implementation — decision for alpha)
**Date:** 2026-06-17
**Deciders:** Agent platform, GPUI, Product
**Related question:** [ops/12-open-questions.md#q13-lanes](../ops/12-open-questions.md#q13-lanes)
**Related specs:** [harness/local §C.4](../harness/local/01-agent-harness.md#c-4-multi-lane-gpui-native-swarms),
  [lane panel](../harness/local/01-agent-harness.md#lane-panel-ui), [06 §new-crates](../core/06-system-design.md#new-crates)

## Context

Multi-lane UI ([local §C.4](../harness/local/01-agent-harness.md#c-4-multi-lane-gpui-native-swarms))
requires a session model for parallel explore / implement / verify lanes. Two
architectures are viable:

1. **One parent `AcpThread`** with sub-sessions keyed by `session_id` (mirrors
   existing sidechain model for background agents).
2. **Multiple `AcpThread` entities** — one per lane tab — with explicit sync.

Dogfood needs lane panel in beta gate ([11 §beta-gate](../ops/11-metrics-and-success#beta-gate)).
Wrong choice causes plan desync, duplicate checkpoints, or notification routing bugs.

Existing code already spawns subagents with `session_id` and sidechain JSONL
([local §B.1](../harness/local/01-agent-harness.md#b-1-background-subagents)). Multi-lane should extend
this pattern rather than invent parallel thread entities.

## Decision drivers

| Driver | Weight | Notes |
|--------|--------|-------|
| Shared checkpoint stack | H | User expects one rewind timeline |
| Unified plan + spec index | H | SDAL assumes one plan per session |
| Implementation time to beta | H | Sub-session model reuses sidechain |
| Independent model per lane | M | Can set model on spawn, not thread entity |
| GPUI tab isolation | M | UI filter, not separate entity required |
| Notification routing clarity | M | Parent owns rail |
| Upstream merge cost | L | Mostly CueCode-new code |

## Considered options

### Option 1: One parent AcpThread + sub-sessions

**Description:** Lanes are UI tabs filtering sub-sessions under one parent. Shared
plan, checkpoint stack, spec index. Each lane spawn gets `session_id`; coordinator
is the parent thread (Orchestrate intent).

| Pros | Cons |
|------|------|
| Reuses sidechain + notification protocol | Parent state machine grows complex |
| Single plan + SDAL story | Lane conflict detection needed |
| One checkpoint stack | Cancel/lifecycle must not poison parent |
| Matches Q14 in-process spawn recommendation | Harder if lanes need fully independent plans later |

### Option 2: Multiple AcpThread entities

**Description:** Each lane tab is its own `AcpThread` with cross-thread sync layer.

| Pros | Cons |
|------|------|
| Strong isolation per lane | Plan/checkpoint sync is new hard problem |
| Maps 1:1 to GPUI tabs mentally | Duplicate spec index injection |
| Independent archive/close | Notification routing across threads |
| | Breaks "session-first" unit of work |

### Option 3: Hybrid — parent + one implement thread only

**Description:** Parent for coordinator/explore; second thread only for implement.

| Pros | Cons |
|------|------|
| Smaller than full multi-thread | Arbitrary split; verify still needs home |
| | Two plans or sync anyway |

## Decision outcome

**Chosen option:** Option 1 — **One parent `AcpThread` with sub-sessions keyed by
`session_id`.**

**Rationale:**

- Aligns with existing `spawn_agent` + sidechain infrastructure ([local §data-flow](../harness/local/01-agent-harness.md#data-flow)).
- Preserves single plan, single checkpoint stack, single SDSW session unit ([11 §SDSW](../ops/11-metrics-and-success#sdsW)).
- Q14 recommendation (in-process background `Task`) assumes shared parent state.
- Lane panel is a **view** over sub-sessions, not new entity types ([local §lane-switch](../harness/local/01-agent-harness.md#lane-switch)).
- Independent models per lane achieved via `SpawnAgentToolInput` + model picker, not separate threads.

## Consequences

### Positive

- Faster beta: extend notification rail + lane tabs without new session entity.
- Coordinator (Orchestrate) naturally owns parent composer — no nested coordinator thread.
- `LaneConflict` notification ([local §rust-types](../harness/local/01-agent-harness.md#rust-types)) blocks dual writers on same path set.
- Metrics: one `session_start` per user session; lanes tracked via `spawn_background` + lane id field.

### Negative / tradeoffs

- Parent `AcpThread` state machine must track N sub-session lifecycles.
- Closing a lane cancels `Task` but must not drop parent plan entries.
- If dogfood shows plan sync pain, revisit many-thread model in ADR-042 (explicit supersede).

### Neutral

- GPUI implements lanes as `Vec<LaneTab>` subscribed to parent sub-session metadata.
- Explore lane defaults async; implement defaults active — execution context per spawn, not per thread.

## Compliance / verification

- [ ] `harness/local/01-agent-harness.md` lane sections reference parent + sub-session model
- [x] `12-open-questions.md` Q13 marked Resolved with link to ADR-041
- [ ] `cargo test -p acp_thread` covers multi sub-session notification routing
- [ ] GPUI test: lane tab switch filters transcript without new AcpThread
- [ ] Metric: `lane_id` optional field on `spawn_background` event ([11 §event-catalog](../ops/11-metrics-and-success#event-catalog))
- [ ] Dogfood: multi-lane story in [local §lane-panel-full](../harness/local/01-agent-harness.md#lane-panel-full) replayable

## Notes

- Spike: 2-day prototype lane tabs over mock sub-session list in `agent_ui`.
- If superseded, new ADR must address checkpoint/plan sync requirements explicitly.
- Cross-link [Q14](../ops/12-open-questions#q14-background): in-process spawn remains v1 transport.
