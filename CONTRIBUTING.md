# Contributing to CueCode

CueCode is a fork of Zed maintained in this repository. Before making changes, read the workspace specs:

- [Spec index](../.cursor/specs/00-README.md)
- [Fork & rebrand](../.cursor/specs/core/03-fork-and-rebrand.md)
- [Master build plan](../.cursor/specs/delivery/build-plans/00-master-build-plan.md)

Run `./script/rebrand-check.sh` before opening a Phase 0 rebrand PR (covers packaging Tier 4 gates for snap, flatpak, and `.desktop` metadata). For a full automated QA-P0 smoke pass: `./script/qa-p0.sh` (uses an isolated `/tmp/cuecode-qa-*` profile). For headless agent + Ollama: `./script/qa-agent-ollama.sh` (also CI job `agent-ollama-linux`). For idle network audit (no zed.dev): `./script/network-idle-audit.sh` (CI job `network-idle-linux`). To verify gates and update the rebrand checklist: `./script/rebrand-progress.sh --full` — see [Progress tracker](../.cursor/specs/core/03-zed-reference-cleanup-phases.md#progress).

---

# Contribution guide

Thank you for helping us make CueCode better!

All activity in CueCode forums is subject to our [Code of
Conduct](https://cuecode.dev/code-of-conduct). Additionally, contributors must sign
our [Contributor License Agreement](https://cuecode.dev/cla) before their
contributions can be merged.

## Contribution ideas

CueCode is a large project with a number of priorities. We spend most of
our time working on what we believe the product needs, but we also love working
with the community to improve the product in ways we haven't thought of (or had time to get to yet!)

In particular we love PRs that are:

- Fixing or extending the docs.
- Fixing bugs.
- Small enhancements to existing features to make them work for more people (making things work on more platforms/modes/whatever).
- Small extra features, like keybindings or actions you miss from other editors or extensions.
- Part of a Community Program like [Let's Git Together](https://github.com/zed-industries/zed/issues/41541).

If you're looking for concrete ideas:

- [Triaged bugs with confirmed steps to reproduce](https://github.com/zed-industries/zed/issues?q=is%3Aissue%20state%3Aopen%20type%3ABug%20label%3Astate%3Areproducible).
- [Area labels](https://github.com/zed-industries/zed/labels?q=area%3A*) to browse bugs in a specific part of the product you care about (after clicking on an area label, add type:Bug to the search).

If you're thinking about proposing or building a larger feature, read the [CueCode feature process](./docs/src/development/feature-process.md) for how we think about feature design — what context to provide, what integration points to consider, and how to put together a strong proposal.

## Sending changes

The CueCode culture values working code and synchronous conversations over long
discussion threads.

The best way to get us to take a look at a proposed change is to send a pull
request. We will get back to you (though this sometimes takes longer than we'd
like, sorry).

Although we will take a look, we tend to only merge about half the PRs that are
submitted. If you'd like your PR to have the best chance of being merged:

- Make sure the change is **desired**: we're always happy to accept bugfixes,
  but features should be confirmed with us first if you aim to avoid wasted
  effort. If there isn't already a GitHub issue for your feature with staff
  confirmation that we want it, start with a GitHub discussion rather than a PR.
  - This especially applies to any changes proposed to the CueCode extension API.
- Include a clear description of **what you're solving**, and why it's important.
- Include **tests**. For UI changes, consider updating visual regression tests (see [Building CueCode for macOS](./docs/src/development/macos.md#visual-regression-tests)).
- If it changes the UI, attach **screenshots** or screen recordings.
- Make the PR about **one thing only**, e.g. if it's a bugfix, don't add two
  features and a refactoring on top of that.
- Keep AI assistance under your judgement and responsibility: it's unlikely
  we'll merge a vibe-coded PR that the author doesn't understand.

### AI Policy

We welcome the use of LLMs for coding, but we hold a high bar for all contributions, and **we expect a human in the loop who genuinely understands the work an LLM produces** on their behalf. For that reason, we **don't accept contributions from autonomous agents**. Pull requests that appear to violate this may be closed, sometimes without notice.

**Don't rely on LLMs to write the whole thing for you when communicating with the maintainers** (meaning replies to comments, PR descriptions, and alike). The readers are humans, and we'd like to hear from you, not from a model (we have models at home). If you're a non-native English speaker using an LLM to thoroughly edit or translate your messages to the maintainers, we'd encourage you to **put the machine translation in a quote block and include the original text in your native language after it**.

If you think it's helpful/necessary to **share context from a chat with an LLM**, please put the **relevant part of it** in a quote block (e.g., using `>`), **disclose it as AI-generated**, and add your own commentary explaining **why it's relevant and what you take from it**.

This policy was adapted from [ripgrep's AI policy](https://github.com/BurntSushi/ripgrep/blob/f0cec341ab95c25c691ad3d5754d4bd9eedde21f/AI_POLICY.md).

### Internal advice for reviewers

- If the fix/feature is obviously great, and the code is great. Hit merge.
- If the fix/feature is obviously great, and the code is nearly great. Send PR comments, or offer to pair to get things perfect.
- If the fix/feature is not obviously great, or the code needs rewriting from scratch. Close the PR with a thank you and some explanation.

If you need more feedback from us: the best way is to be responsive to
Github comments, or to offer up time to pair with us.

If you need help deciding how to fix a bug, or finish implementing a feature
that we've agreed we want, please open a PR early so we can discuss how to make
the change with code in hand.

### UI/UX checklist

When your changes affect UI, consult this checklist:

**Accessibility / Ergonomics**

- Do all keyboard shortcuts work as intended?
- Are shortcuts discoverable (tooltips, menus, docs)?
- Is it usable without a mouse (keyboard-only navigation)?
- Do all mouse actions work (drag, context menus, resizing, scrolling)?
- Does the feature look great in light and dark mode themes?
- Are hover states and focus indicators clear and consistent?

**Responsiveness**

- Does the UI scale gracefully on:
  - Narrow panes (e.g., side-by-side split views)?
  - Short panes (e.g., laptops with 13" displays)?
  - High-DPI / Retina displays?
- Does resizing panes or windows keep the UI usable and attractive?
- Do dialogs or modals stay centered and within viewport bounds?

**Platform Consistency**

- Is the feature fully usable on Windows, Linux, and macOS?
- Does it respect system-level settings (fonts, scaling, input methods)?

**Performance**

- All user interactions must have instant feedback.
  - If the user requests something slow (e.g. an LLM generation) there should be some indication of the work in progress.
- Does it handle large files, big projects, or heavy workloads without degrading?
- Frames must take no more than 8ms (120fps)

**Consistency**

- Does it match CueCode's design language (spacing, typography, icons)?
  - Make sure to visit [the icon design guidelines](https://github.com/zed-industries/zed/blob/main/crates/icons/README.md)
- Are terminology, labels, and tone consistent with the rest of CueCode?
- Are interactions consistent (e.g., how tabs close, how modals dismiss, how errors show)?

**Internationalization & Text**

- Are strings concise, clear, and unambiguous?
- Do we avoid internal CueCode jargon that only insiders would know?

**User Paths & Edge Cases**

- What does the happy path look like?
- What does the unhappy path look like? (errors, rejections, invalid states)
- How does it work in offline vs. online states?
- How does it work in unauthenticated vs. authenticated states?
- How does it behave if data is missing, corrupted, or delayed?
- Are error messages actionable and consistent with CueCode’s voice?

**Discoverability & Learning**

- Can a first-time user figure it out without docs?
- Is there an intuitive way to undo/redo actions?
- Are power features discoverable but not intrusive?
- Is there a path from beginner → expert usage (progressive disclosure)?

## Things we will (probably) not merge

Although there are few hard and fast rules, typically we don't merge:

- Anything that can be provided by an extension. For example a new language, or theme. For adding themes or support for a new language to CueCode, check out our [docs on developing extensions](https://cuecode.dev/docs/extensions/developing-extensions).
- Changes to the CueCode extension API submitted without prior discussion involving CueCode maintainers.
- New file icons. CueCode's default icon theme consists of icons that are hand-designed to fit together in a cohesive manner, please don't submit PRs with off-the-shelf SVGs.
- Features where (in our subjective opinion) the extra complexity isn't worth it for the number of people who will benefit.
- Giant refactorings.
- Non-trivial changes with no tests.
- Stylistic code changes that do not alter any app logic. Reducing allocations, removing `.unwrap()`s, fixing typos is great; making code "more readable" — maybe not so much.
- Anything that seems AI-generated without understanding the output.

## Bird's-eye view of CueCode

We suggest you keep the [CueCode glossary](docs/src/development/glossary.md) at your side when starting out. It lists and explains some of the structures and terms you will see throughout the codebase.

CueCode is made up of several smaller crates - let's go over those you're most likely to interact with:

- [`gpui`](/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for CueCode. **We recommend familiarizing yourself with the root level GPUI documentation.**
- [`editor`](/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields within CueCode. It also handles a display layer for LSP features such as Inlay Hints or code completions.
- [`project`](/crates/project) manages files and navigation within the filetree. It is also CueCode's side of communication with LSP.
- [`workspace`](/crates/workspace) handles local state serialization and groups projects together.
- [`vim`](/crates/vim) is a thin implementation of Vim workflow over `editor`.
- [`lsp`](/crates/lsp) handles communication with external LSP server.
- [`language`](/crates/language) drives `editor`'s understanding of language - from providing a list of symbols to the syntax map.
- [`collab`](/crates/collab) is the collaboration server itself, driving the collaboration features such as project sharing.
- [`rpc`](/crates/rpc) defines messages to be exchanged with collaboration server.
- [`theme`](/crates/theme) defines the theme system and provides a default theme.
- [`ui`](/crates/ui) is a collection of UI components and common patterns used throughout CueCode.
- [`cli`](/crates/cli) is the CLI crate which invokes the CueCode binary.
- [`cuecode`](/crates/cuecode) is where all things come together, and the `main` entry point for CueCode.

## Packaging CueCode

Check our [notes for packaging CueCode](https://cuecode.dev/docs/development/linux#notes-for-packaging-cuecode).
