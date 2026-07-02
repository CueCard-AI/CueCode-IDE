# Rust change checklist

Use before marking a Rust change complete or opening a PR.

## Code quality

- [ ] Read surrounding code; matched naming, types, and patterns
- [ ] No new `unwrap()` / `expect()` in production paths
- [ ] Errors propagated with `?` or handled explicitly (no `let _ =` on fallible ops)
- [ ] No panicking indexing; bounds handled safely
- [ ] Full variable names; no unnecessary new files or `mod.rs`

## GPUI (if applicable)

- [ ] State changes that affect UI call `cx.notify()`
- [ ] No nested `entity.update` while already inside an update closure
- [ ] Async spawns use `WeakEntity` where appropriate; tasks not dropped accidentally
- [ ] GPUI tests use executor timers, not `smol::Timer::after` with `run_until_parked()`

## Agent / CueCode (if applicable)

- [ ] Tool errors surface meaningfully to the user
- [ ] Diff scoped to the requested feature; no drive-by refactors
- [ ] Relevant `.cursor/specs/` section read and linked in PR if product work

## Verification

- [ ] `./script/clippy` passes for touched crates
- [ ] Targeted `cargo test -p <crate> ...` passes
- [ ] Diff is one logical change; unrelated files reverted
