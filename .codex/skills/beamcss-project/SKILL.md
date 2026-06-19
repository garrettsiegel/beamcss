---
name: beamcss-project
description: Use when working in the Beam CSS repository, including planning, implementing, reviewing, or testing changes to the Rust core/compiler, CLI, Node package, Vite/PostCSS/MCP integrations, grammar, JSON5 config parsing, native binding, examples, snapshots, and project workflow.
---

# Beam CSS Project

## Start Here

Read `CLAUDE.md` before changing code and treat it as the source of truth for project architecture, commands, non-negotiables, out-of-bounds areas, and the per-slice workflow.

Use this skill to keep Beam CSS changes small, readable, and aligned with the repo's public API and supply-chain constraints.

## Code Style

- Keep files single-purpose. For new or actively refactored modules, aim for 200-250 lines unless a local pattern strongly justifies more.
- Split large legacy files only when the split directly supports the current change. Do not perform drive-by module churn.
- Prefer clear Rust and TypeScript modules with small, focused functions and explicit data flow.
- Write code a junior developer can step into and understand without hidden cleverness.
- Use existing repo patterns before introducing new abstractions.
- Keep comments concise. Prefer one-line, all-caps intent comments only when the code's reason is not obvious.

## Beam Constraints

- Treat the class-string grammar as public API. Any grammar change needs semver-level care and must be reflected in `docs/beam-css-spec.md` when appropriate.
- Preserve the JSON5-only config parsing contract. Do not execute TypeScript config files.
- Keep the napi-rs interface in `crates/beam_node/src/lib.rs` and `packages/beamcss/src/native.ts` in sync whenever either side changes.
- Preserve the fixed layer order: `beam.reset, beam.tokens, beam.base, beam.utilities`.
- Do not add Rust crates, top-level npm packages, postinstall scripts, or CI changes without explicit approval.
- Use every completed grammar feature in `examples/`.
- Add or update golden-file snapshots for compiler output changes.
- After `beam_core` or `beam_node` changes, rebuild the committed native binding before relying on JS/plugin tests that load it.

## Slice Workflow

Before code changes, state the slice contract:

- Intended slice
- Allowed files/functions
- Tests that must pass
- Explicit non-goals

After the slice, emit the receipt from `CLAUDE.md`:

- Intended slice
- Allowed files/functions
- Actual files/functions changed
- Behavior added
- Behavior removed
- Tests added/updated
- Explicit non-goals
- Rollback path

Then run the two gates in order:

1. Authorization gate: confirm actual changes match the allowed files/functions.
2. Quality gate: confirm the implementation is correct, readable, and covered by the selected tests.
