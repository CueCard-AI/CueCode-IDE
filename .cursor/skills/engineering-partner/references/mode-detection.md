# Mode detection

Use these signals to pick **Ideate**, **Solve**, or **Product**. When signals
conflict, prefer **Solve** if there is an error/repro; otherwise ask one question.

## Ideate signals

- "ideas", "brainstorm", "what if", "crazy thought", "spitball"
- "options", "approaches", "tradeoffs", "worth doing"
- Vague goal without repro or file path
- "I'm thinking about building…"
- Early CueCode feature shape with no implementation yet
- "help me think through"

## Solve signals

- Stack traces, error messages, "build failed", "doesn't compile"
- "fix", "debug", "why does", "where is", "how do I run"
- Specific file, line, crate, or command output pasted
- "make it work", "still broken", "same error"
- Regression after a change

## Product signals

- "should we", "user experience", "UX", "flow", "onboarding"
- "confusing", "discoverable", "first-time user"
- Feature priority, scope, MVP
- References to `.cursor/specs/` or CueCode vision

## Hybrid examples

| User message | Start as | Then |
|--------------|----------|------|
| "I want intent switcher but not sure where it lives" | Product + Ideate | narrow to `agent_ui`, then Solve |
| "cargo run fails in webrtc-sys" | Solve | no ideate unless fix needs architecture choice |
| "brainstorm CueCode review UI then stub it" | Ideate → Product | then Solve with spec link |

## One-liner disambiguation

If unsure:

> Want me to riff on options, or dig in and fix/debug directly?
