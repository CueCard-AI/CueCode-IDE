# `acp_thread` — agent notes

ACP (Agent Client Protocol) threads — external agent servers (Cursor/Claude-style ACP), mentions, diffs, terminal attachment. Load `agent-inference` + `rust-quality`.

## Where to look

`acp_thread.rs` (thread state + ACP lifecycle), `connection.rs` (`AgentConnection` / `AgentServer` impls), `diff.rs` (diff application), `mention.rs` (`@`-mention parsing incl. `MentionUri`), `terminal.rs` (terminal-thread attachment).

## Traps

- **ACP is for external agent servers**, distinct from the native `agent` crate thread. Don't conflate the two pipelines — native uses `agent::Thread`; ACP uses `acp_thread` + `connection`.
- **`mention.rs` owns `@`-mention parsing** including `MentionUri` variants (files, symbols, plan artifacts). New mention kinds go here; keep the agent panel's mention UI in sync (`agent_ui`).
- **Diff application (`diff.rs`) must be robust to stale buffers** — ranges drift; use the editor's transform helpers, not raw offsets.
- **Plan/spec mentions (`MentionUri::PlanArtifact`) are part of the Phase 1.5 spec-roots work** — coordinate with `cuecode_specs` / `agent` when extending.

## Verify

```bash
cargo test -p acp_thread
./script/clippy -p acp_thread
```
