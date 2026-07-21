# Teams and tasks {#teams-and-tasks}

> **Program:** [15-competitive-parity](./15-competitive-parity.md) · **CC:** TeamCreate, Task*, SendMessage, coordinatorMode  
> **Inventory:** [00 §tools-tasks-teams](../research/00-claude-code-inventory.md#tools-tasks-teams)

Lane-based multi-agent protocol — **Adapt** Claude Code teams/tasks into GPUI-native **lanes** under Orchestrate intent, with optional task entity for CC parity.

---

## Teams vs lanes {#teams-vs-lanes}

**Decision (v1):** **Adapt** — no separate `Team` git-tracked entity. A **lane** is a sub-session with role, execution context, and optional task binding.

| CC concept | CueCode v1 |
|------------|------------|
| `TeamCreateTool` | Spawn lane under Orchestrate parent |
| Teammate agent | Builtin agent type on lane (`explore`, `implement`, …) |
| `SendMessageTool` | `send_lane_message` → notification + parent inject |
| In-process teammate tools | Lane-scoped task tools ([task-protocol](#task-protocol)) |

**v2 option:** Formal `Team` entity if dogfood requires CC SDK parity — ADR required.

---

## Coordinator {#coordinator}

**CC analog:** `coordinatorMode.ts`, `COORDINATOR_MODE_ALLOWED_TOOLS`.

**CueCode:** **Orchestrate intent** on main thread ([04 §intent-profiles](../core/04-sandbox-core#intent-profiles)).

| Rule | Enforcement |
|------|-------------|
| Main thread read-only for edits | Rust tool wall |
| Allowed | `spawn_agent`, `read_spec`, `list_specs`, plan tools, `send_lane_message` |
| Denied | `edit_file`, `terminal`, `checkpoint` (workers produce artifacts) |
| UI | Coordinator composer ([local §coordinator-ui](../harness/local/01-agent-harness.md#coordinator-ui)) |

**Hybrid artifact:** Synthesis message + merged plan entries citing lane artifacts.

---

## Task protocol {#task-protocol}

**CC analog:** `TaskCreateTool`, `TaskGetTool`, `TaskListTool`, `TaskUpdateTool`, `TaskStopTool`, `TaskOutputTool`.

Map to CueCode **Task** entity stored on parent `AcpThread` metadata + lane binding.

```rust
pub struct HarnessTask {
    pub id: TaskId,
    pub lane_id: Option<LaneId>,
    pub title: String,
    pub status: TaskStatus, // pending | running | done | failed | cancelled
    pub agent_type: Option<String>,
    pub session_id: Option<SessionId>,
    pub artifact_path: Option<PathBuf>,
}
```

### Tool mapping

| CC tool | CueCode tool | Mode |
|---------|--------------|------|
| TaskCreate | `task_create` | Hybrid |
| TaskGet | `task_get` | Active |
| TaskList | `task_list` | Active |
| TaskUpdate | `task_update` | Hybrid |
| TaskStop | `task_stop` | Async |
| TaskOutput | `task_output` | Async |

**TodoWriteTool:** **Adapt** to `AcpThread.plan` — plan entries may link to `task_id` when Orchestrate.

---

## Lane messaging {#lane-messaging}

**CC analog:** `SendMessageTool`.

```json
{
  "lane_id": "implement",
  "message": "Focus on auth crate only",
  "delivery": "inject_next_turn | notification_only"
}
```

Coordinator and parent may message lanes; lanes respond via notification envelope ([local §notification-payloads](../harness/local/01-agent-harness.md#notification-payloads)).

---

## Cancellation {#cancellation}

**CC analog:** `TaskStopTool`, `TASK_STOP`.

| Action | Behavior |
|--------|----------|
| User cancel pill | Drop `Task`, cancel subagent, keep sidechain |
| `task_stop` | Same; coordinator-only on Orchestrate |
| Parent session | Never poisoned ([local §policies](../harness/local/01-agent-harness.md#policies)) |

---

## Worktree isolation {#worktree-isolation}

**CC analog:** `EnterWorktreeTool`, `ExitWorktreeTool`.

**Decision:** **Defer** Phase 5 — required for CC batch skill parity (`isolation: worktree`).

| Phase | Behavior |
|-------|----------|
| Ship intent | Optional isolated worktree per session |
| Enter | `enter_worktree` tool or UI "Ship in worktree" |
| Exit | Merge via unified review + user confirm |
| Lanes | Share parent worktree unless Ship isolation enabled |

---

## Lane conflict {#lane-conflict}

One writer per path set ([local §policies](../harness/local/01-agent-harness.md#policies)).

Emit `lane_conflict` notification ([local §notification-example-lane-conflict](../harness/local/01-agent-harness.md#notification-example-lane-conflict)).

---

## UI {#ui}

| Surface | CC analog | Spec |
|---------|-----------|------|
| Lane panel tabs | tmux-less teams | [local §lane-panel](../harness/local/01-agent-harness.md#lane-panel-ui) |
| `/tasks` | Task list command | [19 §harness](./19-command-surface.md#harness-commands) |
| Task pills | LocalAgentTask | [local §task-pills](../harness/local/01-agent-harness.md#task-pills) |

---

## Cloud harness {#cloud}

Lane scheduler runs server-side ([cloud/05 §scheduler](../harness/cloud/05-cloud-services.md)); client executes tools. Semantics identical to local spec.

---

## Acceptance {#acceptance}

### AC-TEAM-1: Coordinator tool wall

**Given** Orchestrate intent  
**When** main agent calls `edit_file`  
**Then** intent blocked ([16 flow C](./16-end-to-end-flows.md#flow-c-coordinator))

### AC-TEAM-2: Task CRUD parity

**Given** coordinator creates task via `task_create`  
**When** implement lane completes  
**Then** `task_update` marks done and notification references task_id

---

## Document status {#status}

| Field | Value |
|-------|-------|
| Status | Draft — gap spec |
| Last updated | 2026-06-17 |
