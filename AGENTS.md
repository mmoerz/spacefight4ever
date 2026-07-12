# Agents.md - Foundational Mandates

This document outlines the core development principles and architectural mandates for the `spacefight4ever` project. These instructions take precedence over general defaults.

## Project Knowledge

- **Tech Stack:** in `.agents/tech-stack.md`
- **Layered Architecture:** All new code must follow the layered architecture defined in `.agents/arch.md`.

## Build & Test Commands

- **Build:** `cargo build --package spacefight4ever-bin`

## Standards
These standards apply to all generated, modified, and reviewed code.

- **Naming conventions:** Follow standard Rust naming conventions.
- **Coding guidelines:** See `.agents/code.md`.
- Prefer existing project patterns
- Minimize the scope of changes.

## Specialist Instructions
When appropriate, consult:

- **Senior Developer:** lives in `.agents/skills/senior-dev/SKILL.md`
- **Developer:** lives in `.agents/skills/developer/SKILL.md`
- **Test Engineer:** lives in `.agents/skills/test-engineer/SKILL.md`
- **Documentation Writer:** lives in `.agents/skills/tech-docu-writer/SKILL.md`

## Constraints

- ✅ **Always:** 
  - Follow the existing directory structure.
  - Preserve architectural boundaries.

- ⚠️ **Ask first:** 
  - Changing directory structure
  - Adding dependencies
  - Modifying architecture.
  
- 🚫 **Never:** 
  - Commit secrets or API keys.
  - Perform unrelated refactors
  - Introduce new frameworks or build tooling without approval
