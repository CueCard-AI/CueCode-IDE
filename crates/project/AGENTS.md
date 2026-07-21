# `project` — agent notes

Files, buffers, LSP orchestration, git, worktrees, agent/context servers, debugger, prettier. Load `agent-inference` (for agent/context servers) + `rust-quality`.

## Where to look

`project.rs` (the core `Project`), `buffer_store.rs` / `buffers`, `lsp_store.rs` (+ `lsp_store/`), `git_store.rs` (+ `git_store/`), `connection_manager.rs`, `agent_server_store.rs`, `context_server_store.rs` (+ dir), `debugger.rs` (+ `debugger/`), `prettier_store.rs`, `project_settings.rs`, `manifest_tree.rs`, `environment.rs`.

## Traps

- **Worktree trust is explicit** — see `docs/src/worktree-trust.md` and the trust store work (Phase 4). Don't auto-trust arbitrary worktrees.
- **LSP lifecycle is orchestrated here** (`lsp_store.rs`), not in `lsp/` (which is the wire layer). Server crashes/restarts, capability negotiation, and multi-server fan-out live here.
- **Agent servers + context servers (MCP) are registered here** (`agent_server_store.rs`, `context_server_store.rs`) — these feed the agent's external-agent / MCP paths; see `agent-inference` skill.
- **Don't block the UI on project operations.** Buffer/git/LSP work is async; keep heavy work off the foreground thread.

## Verify

```bash
cargo test -p project
./script/clippy -p project
```
