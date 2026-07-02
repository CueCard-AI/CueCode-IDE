# cuecode-harness (private repo template)

**Scaffolded repo:** `~/CueInference/cuecode-harness/` (see that directory's `README.md`).

Copy this layout into a **private** repository sibling to `CueCode-IDE`:

```
~/CueInference/
├── CueCode-IDE/        # public GPL
└── cuecode-harness/    # this template
```

## Quick start

```bash
cd cuecode-harness
cp .env.example .env
cargo run -p harness-api -- --dev --port 8787
```

In another terminal:

```bash
cd ../CueCode-IDE
script/cuecode-local
```

## M0 parity

Until `harness-api` exists, use the **GPL stub** in the public repo:

```bash
cd ../CueCode-IDE
script/cuecode-local --stub
```

## Shared types

Depend on public CHP types:

```toml
cuecode_chp = { git = "https://github.com/YOUR_ORG/CueCode-IDE", branch = "main" }
```

Local dev patch:

```toml
[patch."https://github.com/YOUR_ORG/CueCode-IDE"]
cuecode_chp = { path = "../CueCode-IDE/crates/cuecode_chp" }
```

## Deploy

Tag-based deploy (mirror CueCode collab):

```bash
./script/deploy-harness staging
./script/deploy-harness production
```

Tags: `harness-staging`, `harness-production`
