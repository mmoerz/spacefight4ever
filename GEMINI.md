# GEMINI.md - Foundational Mandates

This document outlines the core development principles and architectural mandates for the `spacefight4ever` project. These instructions take precedence over general defaults.

## 1. Architectural Integrity
- **Layered Architecture:** Adhere strictly to the layered architecture defined in `Architecture.md`:
  - **Game Logic Layer:** Independent of UI, communicates via events.
  - **UI State Layer:** Listens to game events and updates UI state.
  - **UI Rendering Layer:** Bevy UI components and systems.
- **Modularity:** Maintain separation between `spacefight4ever-lib` (core logic), `spacefight4ever-bin` (entry point), and `spacefight4ever-test` (verification).

## 2. Engineering Standards
- **Bevy Idioms:** Use Bevy's ECS (Entities, Components, Systems) and Plugin architecture for all game features.
- **Type Safety:** Leverage Rust's type system to enforce game rules and state transitions.
- **Documentation:** Ensure all new modules and public APIs are documented. Update `Architecture.md` when structural changes occur.

## 3. Testing & Validation
- **Test-Driven:** New features and bug fixes MUST include corresponding tests in the `spacefight4ever-test` crate.
- **Empirical Verification:** Before fixing a bug, reproduce it with a test case in `spacefight4ever-test`.
- **Validation:** Always run tests using `cargo test` before considering a task complete.

## 4. UI Development
- **Consistency:** Follow the existing patterns in `spacefight4ever-lib/src/ui/`.
- **Widgets:** Reuse or extend widgets in `spacefight4ever-lib/src/ui/widgets/`.
- **Events:** Use Bevy events for communication between the game world and the UI.
