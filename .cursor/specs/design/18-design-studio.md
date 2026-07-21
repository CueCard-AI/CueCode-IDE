# Design Studio {#design-studio}

> **Status:** Stub — full spec later  
> **Related:** [17-layout-studio](./17-layout-studio.md) (IDE chrome layout; different product) · [16-planning-hub](./16-planning-hub.md) (Agent Linear) · public pitch: `/cuecode#design`

| Field | Value |
|-------|-------|
| **Product name** | **Design Studio** |
| **One-liner** | A shared canvas in the IDE for look and feel: you and the agent sketch, critique, and iterate while you build (Claude Design energy). |
| **Not this** | **Layout Studio** rearranges panels. Design Studio is about the product UI/visuals you are shipping. |

---

## Intent {#intent}

Coding agents today “design” via chat screenshots and pasted Figma links. CueCode already bets on spatial surfaces (Agent Linear, Layout Studio). **Design Studio** is the missing room: a place where look/feel lives next to Plan and Implement, and the agent can push on the same artifact as the human.

Happy path sketch (not locked):

1. Open Design Studio beside editor / agent.
2. Sketch or drop a frame (or link a ticket from Agent Linear).
3. Agent proposes variants or implements against the design.
4. You annotate / critique on the canvas.
5. Agent patches code; preview or frames update.

---

## Open questions (for full spec) {#open-questions}

- Artifact format (native canvas vs HTML preview vs both)?
- How designs attach to Agent Linear tickets?
- Web-first MVP vs GPUI/native preview story?
- Agent tools: read design, edit frames, export to code?
- Multiplayer / review later, or solo agent loop first?

---

## Do not do yet {#do-not}

- Ship public claims beyond **Coming soon** on `/cuecode`.
- Invent protocol names, file formats, or dock JSON in this stub.
- Confuse with Layout Studio naming in UI or marketing.

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-07-13 | Stub created from product direction; full UX/tools/spec deferred |
