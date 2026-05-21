# CLAUDE.md — Beam CSS

Context for Claude Code. Read this, then `docs/beam-css-spec.md` (syntax — the source of truth) and `ROADMAP.md` (build order) before doing anything.

## What this is

Beam is a Rust-fast, utility-first CSS framework that compiles to atomic CSS. The pitch: **Tailwind's authoring speed, without the wall of classes.** Tagline: *focused styles, zero scatter.* Site: beamcss.dev.

It keeps what made Tailwind win (inline, colocated, terse, no naming) and fixes its three real pains:
1. **Class soup** — solved by *variant grouping* (`hover:(bg-accent fg-base)`) and *layout primitives* (`stack`, `row`, `grid`, `cluster`, `place`).
2. **Memorization** — primitives collapse the common layout incantations; atomic names stay Tailwind-familiar.
3. **Dynamic values** — first-class via `w-(--var)` reading a CSS custom property set inline.

## Current state

Placeholder npm package only (`beamcss` published to hold the name; `beamcss.dev` and the GitHub repo exist). No engine yet. We are at Phase 0 of `ROADMAP.md`.

## Architecture decisions (already made)

- **Rust core** for the parser/compiler, exposed to Node via **napi-rs** (native addon, like Lightning CSS / Tailwind Oxide). WASM build later for a web playground.
- **pnpm monorepo.** Packages: `beamcss` (umbrella: engine + CLI), `@beamcss/vite`, `@beamcss/postcss`, `@beamcss/mcp` (later).
- Output is **atomic CSS under cascade layers** (`@layer beam.reset, beam.tokens, beam.base, beam.utilities`) for deterministic specificity. Author-time grouping/primitives are sugar that *unfold* to atoms; nothing survives to runtime cost.
- **License: MIT** for the core (current `package.json` says `UNLICENSED` — fix this first). Monetization lives in separate services, never docs traffic.

## Non-negotiables (enforce these in every change)

- **The class-string grammar (spec §8) is the public API.** Treat changes to it with semver seriousness.
- **Security is supply-chain first:** npm provenance, 2FA, no `postinstall` scripts, minimal deps, `cargo audit` + `npm audit` in CI. Fuzz the parser (cargo-fuzz) from the first commit — it must never panic or hang. Target `#![forbid(unsafe_code)]`.
- **AI-native, not docs-dependent:** `llms.txt`/`llms-full.txt` and an MCP server are first-class deliverables.
- **Dogfood:** every feature gets used in `examples/` before it's "done."

## Build order (see ROADMAP.md for detail)

Phase 0 repo foundations → Phase 1 core engine (start with a *walking skeleton*: config → a few utilities → atomic CSS, before building the full grammar) → Phase 2 integration (bindings, Vite/PostCSS plugins, CLI) → Phase 3 hardening (tests, fuzz, adversarial review, benchmarks) → Phase 4 docs + AI surface → Phase 5 launch.

**Do first:** set MIT license, restructure to the pnpm monorepo, add `SECURITY.md` + CI, then Phase 1 Slice 1 (the walking skeleton).

## Conventions

- Conventional Commits.
- Every grammar feature ships with golden-file output snapshot tests.
- Don't break the walking skeleton — widen from a thing that works; never build the whole grammar before something compiles end to end.

## Brand voice

Confident, not arrogant. Positioned as the next step after Tailwind, never "the Tailwind destroyer" externally. Lowercase, clean, a little frontier-tech.