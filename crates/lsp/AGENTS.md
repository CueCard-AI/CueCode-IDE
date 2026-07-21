# `lsp` — agent notes

The LSP **wire layer** — JSON-RPC communication with external language servers. Load `rust-quality`.

## Where to look

`lsp.rs` (protocol types + transport), `input_handler.rs`.

## Traps

- **This is the wire layer, not the orchestrator.** Multi-server fan-out, capability negotiation, server lifecycle (crash/restart), and project-level LSP state live in `project/src/lsp_store.rs` (+ `lsp_store/`). Don't add orchestration here.
- **Don't block on server responses.** LSP is request/notification/notification-async — keep the foreground thread free; surface results via tasks.
- **Handle server crashes gracefully** — a crashing server must not take the editor down; the store restarts/reports.

## Verify

```bash
cargo test -p lsp
./script/clippy -p lsp
```
