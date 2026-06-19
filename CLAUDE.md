# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Beam CSS is a Rust-fast, utility-first CSS framework. It gives humans and coding agents a small, regular styling grammar that compiles to atomic CSS under cascade layers. The pitch: **Tailwind's authoring speed, without the wall of classes.** Tagline: *focused styles, zero scatter.*

Key differentiators over Tailwind: **variant grouping** (`hover:(bg-accent fg-base)`), **layout primitives** (`stack`, `row`, `cluster`, `grid`, `place`), and **dynamic values** (`w-(--var)` → `var(--var)`).

Read `docs/beam-css-spec.md` for the grammar (the class-string syntax is the public API) and `docs/ROADMAP.md` for what's done and what's next.

## Commands

```sh
# Install dependencies
pnpm install

# Build everything (all JS packages + Rust workspace)
pnpm build

# Run all tests (JS + Rust)
pnpm test

# Type-check only
pnpm typecheck

# Lint
pnpm lint

# Run benchmarks
pnpm benchmark
```

**Rust-only:**
```sh
cargo test --workspace
cargo audit
cargo build -p beam_cli --release
```

**Run a single Rust test:**
```sh
cargo test -p beam_core <test_name>
cargo test -p beam_cli <test_name>
```

**Run a single JS package's tests:**
```sh
pnpm --filter beamcss test
pnpm --filter @beamcss/vite test
```

**Build the native Node binding:**
```sh
pnpm --filter beamcss build:native
```

**CLI smoke tests:**
```sh
cargo run -p beam_cli --bin beam -- build \
  --config examples/walking-skeleton/beam.config.ts \
  --content examples/walking-skeleton \
  --out /tmp/beam-walking.css

cargo run -p beam_cli --bin beam -- check \
  --config examples/walking-skeleton/beam.config.ts \
  --content examples/walking-skeleton \
  --format json

cargo run -p beam_cli --bin beam -- explain "stack(center gap-4) hover:(bg-accent fg-on-accent)" \
  --config examples/walking-skeleton/beam.config.ts \
  --format json
```

**Fuzzing:**
```sh
cargo fuzz run fuzz_compile
```

## Architecture

This is a **pnpm + Cargo monorepo**. The JS packages live in `packages/`, the Rust crates in `crates/`, and example apps in `examples/`.

### Rust crates (`crates/`)

- **`beam_core`** — parser and compiler. Entry points: `compile(config, class_strings) -> CompileResult` and `explain(config, class_strings) -> ExplainResult`. This is where the grammar lives. Output is atomic CSS under `@layer beam.reset, beam.tokens, beam.base, beam.utilities`.
- **`beam_node`** — napi-rs Node binding exposing `beam_core` as a native `.node` addon (same approach as Lightning CSS / Tailwind Oxide).
- **`beam_cli`** — standalone binary. Commands: `beam init`, `beam build`, `beam dev` (watch), `beam check`, `beam explain`. Scans source files for `class=` / `className=` attributes and delegates to `beam_core`.

The workspace enforces `#![forbid(unsafe_code)]` globally.

### JS packages (`packages/`)

- **`beamcss`** — umbrella package users install. Exports:
  - `index.ts` — `defineConfig`, `BeamConfig` types
  - `native.ts` — loads the `.node` binding via napi-rs; falls back gracefully
  - `cli-runner.ts` — `buildCss()` tries native binding first, then shells out to `cargo run` (dev-only fallback for use inside this repo)
  - `scanner.ts` — JS implementation of class-string extraction from source files
  - `codemod.ts` — Tailwind-to-Beam codemod
  - `language.ts` — language server / completions data
  - `cli.ts` — thin Node CLI wrapper
- **`@beamcss/vite`** — Vite plugin. Integrates `buildCss` into the Vite build pipeline with HMR.
- **`@beamcss/postcss`** — PostCSS plugin for non-Vite bundlers.
- **`@beamcss/mcp`** — MCP server (agent-native surface, Phase 4).

### How the native fallback works

`packages/beamcss/src/native.ts` attempts to load a prebuilt `.node` addon from `packages/beamcss/native/`. If unavailable, `cli-runner.ts`'s `buildCssWithRustCli()` locates the repo root by walking up until it finds a `Cargo.toml` that references `crates/beam_cli`, then shells out to `cargo run`.

> **Gotcha:** the prebuilt `.node` in `packages/beamcss/native/` is committed and does **not** rebuild from `cargo test`. After any `beam_core`/`beam_node` change, run `pnpm --filter beamcss build:native` before the JS/plugin tests (`@beamcss/vite`, `@beamcss/postcss`, codemod) — otherwise they run against the stale binding and either miss new behavior or fail on it. The Rust tests use the source directly, so they pass even when the binding is stale; the two can silently diverge.

### Config parsing

`beam.config.ts` uses a TypeScript object literal. Both the Rust CLI and the Node packages share the same extraction logic: find `defineConfig(` or `export default`, extract the brace-balanced object, then parse it as JSON5. This is intentional — no TS execution needed.

## Non-negotiables

- **The class-string grammar (spec §8) is the public API.** Treat changes to it with semver seriousness.
- **`#![forbid(unsafe_code)]`** is enforced at the workspace level — don't remove it.
- **No `postinstall` scripts** anywhere. Supply-chain security is a first-class concern (`cargo audit` + `npm audit` run in CI).
- **Every grammar feature must be used in `examples/` before it's "done."**
- **Golden-file snapshot tests** for every compiler output change.
- Conventional Commits.
