---
title: AI Agents in CueCode
description: Compare CueCode Agent, External Agents, and Terminal Threads.
---

# Agents

CueCode supports three agent paths. Choose the path based on how you want agentic work to run.

| Agent path                                | Runs in                         | Uses                                                                  | Best when                                                                              |
| ----------------------------------------- | ------------------------------- | --------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| [CueCode Agent](./cuecode-agent.md)               | Agent Panel and Threads Sidebar | CueCode-configured LLM providers, native tools, skills, instructions, MCP | You want CueCode's native agent integration                                                |
| [External Agents](./external-agents.md)   | Agent Panel and Threads Sidebar | ACP agent process and its own auth/config                             | You want Claude, Codex, OpenCode, Copilot, Cursor, Pi, or another ACP-integrated agent |
| [Terminal Threads](./terminal-threads.md) | Threads Sidebar and terminal    | Native CLI/TUI auth/config                                            | You want the tool's command-line experience organized in CueCode                           |

An agent path is sometimes called a harness. It is the way agentic work is started, displayed, configured, and controlled in CueCode.

## Agent Path vs. LLM Provider {#agent-path-vs-llm-provider}

| Question                                  | Start here                          |
| ----------------------------------------- | ----------------------------------- |
| Which agent or CLI should run the work?   | This page                           |
| Which model should power CueCode AI features? | [LLM Providers](./llm-providers.md) |

The [CueCode Agent](./cuecode-agent.md) uses models configured in CueCode. [External Agents](./external-agents.md) and [Terminal Threads](./terminal-threads.md) may use their own model setup.

## Thread Types {#thread-types}

Threads are the units shown in the [Threads Sidebar](./parallel-agents.md#threads-sidebar). Thread types include:

- [CueCode Agent](./cuecode-agent.md) threads
- [External Agent](./external-agents.md) threads
- [Terminal Threads](./terminal-threads.md)

Use [Parallel Agents](./parallel-agents.md) to run and manage multiple threads at once.
