# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Beam CSS is a Rust-fast, utility-first CSS framework. It gives humans and coding agents a small, regular styling grammar that compiles to atomic CSS under cascade layers. The pitch: **Tailwind's authoring speed, without the wall of classes.** Tagline: *focused styles, zero scatter.*

Key differentiators over Tailwind: **variant grouping** (`hover:(bg-accent text-base)`), **utility grouping** (`padding:(16 top:24)`), **config composition** (`shortcuts`, `recipes`, `presets`, utility modules), and **dynamic values** (`w-(--var)` → `var(--var)`).

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

cargo run -p beam_cli --bin beam -- explain "flex direction-column gap-4 hover:(bg-accent text-on-accent)" \
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

---

## Architecture constraints (do not violate)

These apply to every agent session. Violations require explicit user re-approval, not a judgment call.

- **No new Rust crates or top-level npm packages** without explicit approval in the task plan.
- **Config parsing contract is JSON5-only.** `beam.config.ts` is extracted via brace-balanced JSON5 parsing — do not add TypeScript execution (`ts-node`, `tsx`, dynamic `require`, etc.).
- **napi-rs binding interface** (`crates/beam_node/src/lib.rs`) public signatures must stay in sync with `packages/beamcss/src/native.ts`. Change both together, never one alone.
- **`@layer` order is fixed:** `beam.reset, beam.tokens, beam.base, beam.utilities`. Do not reorder — it controls specificity.
- **Committed `.node` binary** (`packages/beamcss/native/`) is not rebuilt automatically. After any `beam_core`/`beam_node` change, run `pnpm --filter beamcss build:native`. JS plugin tests (`@beamcss/vite`, `@beamcss/postcss`) test against this binary — if it's stale, they silently test old behavior.
- **All compiler output changes require snapshot test updates.** Never delete a golden file without replacing it.

## Out of bounds (require separate re-approval to touch)

- `Cargo.toml` files (adding/removing crates or dependencies)
- `.github/` (CI configuration)
- `packages/beamcss/native/` (committed `.node` binaries — rebuild, don't edit)
- `fuzz/` targets
- Any semver-breaking grammar change without a version bump plan

---

## Agent workflow (per-slice loop)

Every task runs as a series of small, auditable slices. Each slice has a frozen contract before any code is written.

**Before starting a slice**, state:
- Intended slice (what this pass does)
- Allowed files/functions (exhaustive list)
- Tests that must pass
- Explicit non-goals (what this slice must NOT change)

**After completing a slice**, emit a receipt:
```
- intended slice:
- allowed files/functions:
- actual files/functions changed:
- behavior added:
- behavior removed:
- tests added/updated:
- explicit non-goals:
- rollback path:
```

**Two-gate review (in order):**
1. **Authorization gate** — does "actual files changed" match "allowed files"? If not, revert and re-scope. Don't review quality until authorization passes.
2. **Quality gate** — is the code correct and clean?

If a slice touches `Cargo.toml`, `.github/`, shared types, or the napi binding interface → stop and get explicit re-approval before proceeding.
