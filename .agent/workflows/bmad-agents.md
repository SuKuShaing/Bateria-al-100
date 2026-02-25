---
description: Enables the AI assistant to act as a specific BMAD expert agent by loading its definition from the _bmad directory.
---

# BMAD Agent Interaction Workflow

This workflow allows you to call upon specialized BMAD expert agents (like PM, Architect, Dev, QA) directly in the conversation.

## When to use

- Anytime the user uses a slash command like `/bmm-pm`, `/bmm-dev`, `/bmm-architect`, etc.
- When the user asks to "speak with", "call", or "load" a specific BMAD agent.

## Steps

1. **Identify the Agent:**
    - Detect the requested agent ID (e.g., `bmm-pm`, `bmm-dev`, `core-bmad-master`).
2. **Locate the Definition:**
    - Search for the agent's markdown file in the `_bmad/` directory.
    - Common locations:
        - `_bmad/core/agents/[agent-id].md`
        - `_bmad/bmm/agents/[agent-id].md`
        - `_bmad/bmb/agents/[agent-id].md`

3. **Load the Persona:**
    - Use the `view_file` tool to read the contents of the identified markdown file.
    - Extract the `persona`, `role`, and `instructions` from the file.

4. **Adopt the Identity:**
    - Respond as the requested agent, adhering to its specific communication style, role, and capabilities defined in the file.
    - Maintain this persona until the task is complete or the user requests a different agent.

5. **Future Prevention:**
    - If an agent is not found, check `_bmad/_config/agent-manifest.csv` to see if it's a known agent and where its file is located.

## Registration for Auto-Detection

The following triggers are recognized:

- `/bmm-pm`, `/bmm-analyst`, `/bmm-architect`, `/bmm-dev`, `/bmm-qa`, `/bmm-ux-designer`, `/bmm-sm`, `/bmm-tech-writer`
- `/bmb-agent-builder`, `/bmb-module-builder`, `/bmb-workflow-builder`
- `/core-bmad-master`
- "llama al agente [nombre]"
- "actúa como [nombre de agente]"
