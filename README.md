# beamcss

Beam CSS is utility CSS for AI-generated interfaces. It gives humans and coding
agents a small, regular styling grammar they can validate, explain, and compile
to atomic CSS.

The pitch: **agent-native utility CSS, with Tailwind's authoring speed and
without the wall of classes.** Beam keeps inline utility authoring, then adds
grouped variants, layout primitives, machine-readable docs, `beam check`, and
`beam explain` so generated UI stays reviewable.

This repository is a pnpm monorepo with the first walking skeleton for the
compiler and package surfaces:

- `apps/site` — Vite, React, and TypeScript site for docs, demos, and launch work
- `packages/beamcss` — public umbrella package and CLI placeholder
- `packages/vite` — Vite plugin placeholder
- `packages/postcss` — PostCSS plugin placeholder
- `packages/mcp` — future MCP server placeholder
- `crates/beam_core` — Rust parser/compiler core
- `crates/beam_node` — napi-rs Node binding
- `examples/walking-skeleton` — first dogfood fixture
- `docs/marketing-plan.md` — positioning, launch checklist, and growth plan

## Development

```sh
pnpm install
pnpm build
pnpm test
```

Rust-only checks:

```sh
cargo test --workspace
cargo audit
```

## CLI Smoke Test

```sh
cargo run -p beam_cli --bin beam -- build \
  --config examples/walking-skeleton/beam.config.ts \
  --content examples/walking-skeleton \
  --out /tmp/beam-walking.css
```

## AI-Native Validation

Beam exposes compiler checks in a shape humans and agents can both use before
returning generated UI:

```sh
cargo run -p beam_cli --bin beam -- check \
  --config examples/walking-skeleton/beam.config.ts \
  --content examples/walking-skeleton \
  --format json
```

Use `beam explain` when an agent needs to inspect how a class string unfolds:

```sh
cargo run -p beam_cli --bin beam -- explain "stack(center gap-4) hover:(bg-accent fg-on-accent)" \
  --config examples/walking-skeleton/beam.config.ts \
  --format json
```

`check` is the preflight gate: generated markup should pass it before being
shown to a user. `explain` is the debugging surface for invalid classes,
grouped variants, primitives, emitted selectors, declarations, and cascade
layers.

The package CLI currently bridges to the Rust CLI when run from this repository:

```sh
pnpm --filter beamcss exec beam build \
  --config examples/walking-skeleton/beam.config.ts \
  --content examples/walking-skeleton \
  --out /tmp/beam-walking.css
```

`beam dev` runs the same build in watch mode and rewrites the output file when
the config or scanned source files change.

Create a fresh Vite starter:

```sh
cargo run -p beam_cli --bin beam -- init --template vite
```

Build and smoke-test the local native Node binding:

```sh
pnpm --filter beamcss build:native
```

## Examples

```sh
cargo run -p beam_cli --bin beam -- build \
  --config examples/dashboard/beam.config.ts \
  --content examples/dashboard \
  --out /tmp/beam-dashboard.css
```

## Current Compiler Slice

The first Rust slice accepts a config plus flat class strings, then emits token
variables and deduped atomic utilities under Beam cascade layers. The core now
also parses grouped variants, layout primitives, arbitrary values, and dynamic
`(--var)` values.
