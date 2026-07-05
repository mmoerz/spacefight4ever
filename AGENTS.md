# Agents.md - Foundational Mandates

This document outlines the core development principles and architectural mandates for the `spacefight4ever` project. These instructions take precedence over general defaults.

## Project Knowledge

- **Tech Stack:** Rust, Bevy 0.18, `bevy_asset_loader` 0.26, Avian3D
- **Layered Architecture:** Adhere strictly to the layered architecture defined in `.agents/arch.md`:

## Build & Test Commands

- **Build:** `cargo build --package spacefight4ever-bin`

## Standards

Follow these rules for all code reviewed:

- **Naming conventions:** Follow standard Rust naming conventions.
- **Coding guidelines:** See `.agents/code.md`.

## Other Agents
- **Senior Developer:** lives in `.agents/skills/senior-dev/SKILL.md`
- **Developer:** lives in `.agents/skills/developer/SKILL.md`
- **Test Engineer:** lives in `.agents/test-agent.md`
- **Documentation Writer:** lives in `.agents/tech-writer-agent.md`

## Constraints

- ✅ **Always:** Adhere to the existing directory structure.
- ⚠️ **Ask first:** Directory structure changes, adding dependencies, modifying architecture.
- 🚫 **Never:** Commit secrets or API keys.
