# Changelog

All notable changes to Beam CSS are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Beam CSS uses [Semantic Versioning](https://semver.org/) starting at 0.1.0.

---

## [0.1.0] — 2026-06-19

First real release. All packages move from placeholder stubs to a working implementation.

### beamcss

#### Added
- Native Node binding via napi-rs (`beam_node`); prebuilt for darwin-arm64,
  darwin-x64, linux-x64-gnu, win32-x64-msvc. Sub-millisecond compile with no subprocess.
  Falls back to `cargo run` when the native binary is absent (dev-only).
- `compile(config, classStrings) → CompileResult` — synchronous, thread-safe.
- `explain(config, classStrings) → ExplainResult` — full parse-tree inspection with
  per-atom CSS property, value, and variant breakdown.
- `defineConfig()` — typed config helper; extracted via JSON5, no ts-node required.
- `vars()` — CSS-variable style-prop helper for dynamic bindings (`w-(--w)`).
- `buildCss(options)` — top-level build function used by Vite and PostCSS plugins.
- `scanFiles()` / `extractClassStrings()` — async glob scanner and sync extractor.
- `tailwindToBeamClassName()` — Tailwind-to-Beam codemod utility.
- `describeBeamClass()` / `suggestBeamClasses()` — language-server / completions API.
- `loadConfigSync()` / `parseConfigSource()` — config parsing helpers.
- CLI: `beam build`, `beam dev` (watch), `beam check`, `beam explain`, `beam init`.
- Cascade layers: `beam.reset`, `beam.tokens`, `beam.base`, `beam.utilities`.
- **Variant grouping**: `hover:(bg-accent text-on-accent scale-105)` — factor repeated prefixes.
- **Utility grouping**: `padding:(16 top:24)`, `text:(xl bold center)`, `border:(1 solid accent)`.
- **Color algebra**: `bg-accent+12` (lighten), `bg-accent-20` (darken), `bg-surface/22` (alpha).
- **Dynamic bindings**: `w-(--var)` → `var(--var)`.
- **Arbitrary values**: `w-[347px]`, `bg-[oklch(72%_0.14_240)]`.
- Shortcuts, recipes (base + variant component patterns), presets (composable config fragments).
- Utility modules: layout, spacing, colors, typography, effects.
- Pixel-first numeric spacing: `p-4` → `padding: 4px`.

### @beamcss/vite

#### Added
- Vite plugin: `beamcss({ config?, content? })`.
- Virtual module: `virtual:beamcss.css`.
- CSS injection via `<style data-beamcss>` in `transformIndexHtml`.
- Full HMR: source-file edits trigger incremental rebuild and module invalidation.
- Peer dependency: vite >= 5.

### @beamcss/postcss

#### Added
- PostCSS plugin: `beamcssPostcss({ config?, content? })`.
- Works with webpack, Parcel, Next.js, and any PostCSS-based build pipeline.
- `Once` hook: appends compiled CSS to the PostCSS root.
- Peer dependency: postcss >= 8.

### @beamcss/mcp

#### Added
- MCP server (stdio transport) for agent-native CSS generation.
- `beam_syntax_reference` tool: returns Beam syntax guidance by topic
  (`all`, `variants`, `utilities`, `values`, `install`).
- `beam_scaffold_component` tool: scaffolds `button`, `card`, `dashboard-panel`,
  `form-row` snippets in HTML or JSX.
- `beam_token_summary` tool: summarizes token names from a BeamConfig JSON string.
- Binary: `beamcss-mcp`.

---

[0.1.0]: https://github.com/garrettsiegel/beamcss/releases/tag/v0.1.0
