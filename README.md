# Beam CSS

**Atomic CSS without the class wall.**

Beam is a utility-first CSS framework with a Rust compiler. It gives you Tailwind's authoring speed with one key improvement: **variant grouping** lets you factor repeated prefixes out of your markup so class strings read as grouped intent instead of repetitive soup. Everything compiles to deduped atomic CSS under cascade layers — zero runtime cost.

```html
<!-- Tailwind -->
<button class="rounded-md px-4 py-2 bg-blue-500 text-white hover:bg-blue-700 hover:shadow-lg hover:scale-105">

<!-- Beam — same output, the hover prefix lives once -->
<button class="rounded-md px-4 py-2 bg-accent text-on-accent hover:(bg-accent+12 shadow-lg scale-105)">
```

---

## Table of contents

- [Quick start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
  - [Tokens](#tokens)
  - [Shortcuts](#shortcuts)
  - [Recipes](#recipes)
  - [Presets](#presets)
  - [Utility modules](#utility-modules)
- [Utilities reference](#utilities-reference)
- [Variant grouping](#variant-grouping)
- [Utility grouping](#utility-grouping)
- [Values: numeric, token, arbitrary, and dynamic](#values-numeric-token-arbitrary-and-dynamic)
- [Color system](#color-system)
- [Cascade layers](#cascade-layers)
- [CLI](#cli)
- [Vite plugin](#vite-plugin)
- [PostCSS plugin](#postcss-plugin)
- [Native Node binding](#native-node-binding)
- [Agent-native surfaces](#agent-native-surfaces)
- [Architecture](#architecture)
- [Development](#development)

---

## Quick start

```sh
npm install beamcss @beamcss/vite
```

Create `beam.config.ts`:

```ts
import { defineConfig } from 'beamcss'

export default defineConfig({
  tokens: {
    color: {
      base: '#0b0b0c',
      surface: '#16161a',
      fg: '#e8e8ea',
      accent: '#3b82f6',
      'on-accent': '#ffffff',
    },
    radius: { md: '8px' },
    text: { base: '16px', lg: '20px' },
    font: { ui: 'Inter, system-ui, sans-serif' },
    screens: { tablet: '48rem' },
  },
  background: 'base',
  foreground: 'fg',
})
```

Add the Vite plugin:

```ts
// vite.config.ts
import { beamcss } from '@beamcss/vite'

export default {
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src/**/*.{html,tsx,jsx,vue,svelte}'],
    }),
  ],
}
```

Write markup:

```html
<main class="grid place-center h-screen bg-base text-fg font-ui">
  <section class="flex direction-column align-center gap-4 p-6 bg-surface rounded-md
                  hover:(bg-surface+8 scale-105)
                  tablet:(direction-row justify-between gap-8)">
    <h1 class="text-lg text-accent">Hello Beam</h1>
  </section>
</main>
```

---

## Installation

| Package | Purpose |
|---|---|
| `beamcss` | Core compiler, CLI, config types |
| `@beamcss/vite` | Vite plugin with HMR |
| `@beamcss/postcss` | PostCSS plugin for other bundlers |

```sh
# Vite projects
npm install beamcss @beamcss/vite

# PostCSS / webpack / other
npm install beamcss @beamcss/postcss
```

---

## Configuration

All configuration lives in a single `beam.config.ts` file. Beam extracts the config at build time with a brace-balanced JSON5 parser — no `ts-node` or dynamic import required.

### Tokens

Tokens are the design system's source of truth. They compile to CSS custom properties and are referenced by name in utilities.

```ts
import { defineConfig } from 'beamcss'

export default defineConfig({
  tokens: {
    // Named spacing values. Numeric utilities like `gap-4` are always 4px.
    spacing: {
      card: '1rem',
      section: '2rem',
    },

    // Color palette. Referenced by `bg-*`, `text-*`, `border-*`.
    color: {
      base: '#0b0b0c',
      surface: '#16161a',
      fg: '#e8e8ea',
      muted: '#6b7280',
      accent: '#3b82f6',
      'on-accent': '#ffffff',
    },

    // Border radius. Referenced by `rounded-*`.
    radius: {
      sm: '4px',
      md: '8px',
      lg: '16px',
      full: '9999px',
    },

    // Font sizes. Referenced by `text-*`.
    text: {
      sm: '14px',
      base: '16px',
      lg: '20px',
      xl: '28px',
    },

    // Font families. Referenced by `font-*`.
    font: {
      ui: 'Inter, system-ui, sans-serif',
      mono: 'ui-monospace, monospace',
    },

    // Responsive breakpoints. Used as variant prefixes.
    screens: {
      tablet: '48rem',           // -> @media (min-width: 48rem)
      desktop: '64rem',
      // Full media query strings are also accepted:
      'mobile-landscape': '(max-width:47.999rem) and (orientation:landscape)',
    },
  },

  // Token names for the body reset's background and color.
  background: 'base',
  foreground: 'fg',
})
```

Each token category compiles to prefixed CSS custom properties:

| Token category | CSS variable prefix | Example |
|---|---|---|
| `spacing` | `--space-*` | `--space-card: 1rem` |
| `color` | `--color-*` | `--color-accent: #3b82f6` |
| `radius` | `--radius-*` | `--radius-md: 8px` |
| `text` | `--text-*` | `--text-lg: 20px` |
| `font` | `--font-*` | `--font-ui: Inter, ...` |
| `screens` | `--screen-*` | `--screen-tablet: 48rem` |

---

### Shortcuts

Named aliases for class strings. Shortcuts expand before compilation, so they accept any valid Beam syntax including variant groups.

```ts
shortcuts: {
  card: 'flex direction-column gap-4 p-card bg-surface rounded-md',
  center: 'grid place-center',
  'sr-only': 'absolute w-[1px] h-[1px] overflow-hidden',
}
```

Usage:

```html
<article class="card hover:(bg-surface+8 scale-105)">
<div class="center h-screen">
```

The shortcut class name becomes the CSS selector. `hover:card` applies all of `card`'s atoms under `:hover`.

---

### Recipes

First-class component variants. A recipe has a `base` class string applied always and named `variants` applied selectively. Recipes replace `cva`, `tailwind-variants`, and similar runtime helpers.

```ts
recipes: {
  button: {
    base: 'px-4 py-2 rounded-md hover:scale-105',
    variants: {
      primary: 'bg-accent text-on-accent hover:bg-accent+12',
      secondary: 'bg-surface border border-line hover:bg-surface+8',
      ghost: 'hover:bg-surface',
    },
  },
  badge: {
    base: 'px-2 py-1 rounded-full text-sm',
    variants: {
      success: 'bg-success/22 text-success',
      warning: 'bg-warning/22 text-warning',
    },
  },
}
```

Usage:

```html
<!-- Recipe base only -->
<button class="button">Default</button>

<!-- Recipe base + variant (compiler expands both automatically) -->
<button class="button:primary">Primary</button>
<button class="button:secondary">Secondary</button>
```

Recipes work inside variant groups:

```html
<button class="hover:button:primary">Hover activates the primary variant</button>
```

---

### Presets

Plain config fragments merged before local config. Local tokens, shortcuts, recipes, and utility flags always win over preset values.

```ts
export default defineConfig({
  presets: [
    // Inline preset object:
    {
      tokens: {
        spacing: { section: '2rem', page: '4rem' },
        color: { brand: '#ff6b35' },
      },
      shortcuts: {
        center: 'grid place-center',
      },
    },
  ],
  // Local tokens override preset tokens of the same key:
  tokens: {
    color: { brand: '#0070f3' },  // overrides preset's brand color
  },
})
```

Presets are plain objects — no plugins API, no side effects. They merge tokens, shortcuts, recipes, and utility module flags.

---

### Utility modules

Tree-shake utility families you don't use. All modules are enabled by default when `utilities` is omitted.

```ts
utilities: {
  layout: true,      // flex, grid, position, overflow, sizing, border
  spacing: true,     // p, m, gap
  colors: true,      // bg, text (color), border (color)
  typography: true,  // font, text (size + align), leading, tracking
  effects: true,     // opacity, scale, shadow
}
```

Disabled utilities produce a compile error rather than silently emitting nothing:

```
utility module `colors` is disabled
```

This makes misconfiguration visible at build time rather than in the browser.

---

## Utilities reference

### Spacing

Numeric values are **pixels**. Named values resolve through `tokens.spacing`.

| Utility | Property | Example | Output |
|---|---|---|---|
| `p-*` | `padding` | `p-4` | `padding:4px` |
| `px-*` | `padding-inline` | `px-16` | `padding-inline:16px` |
| `py-*` | `padding-block` | `py-8` | `padding-block:8px` |
| `pt-*` | `padding-top` | `pt-card` | `padding-top:var(--space-card)` |
| `pr-*` | `padding-right` | | |
| `pb-*` | `padding-bottom` | | |
| `pl-*` | `padding-left` | | |
| `m-*` | `margin` | `m-auto` | `margin:auto` |
| `mx-*` | `margin-inline` | | |
| `my-*` | `margin-block` | | |
| `mt-* mr-* mb-* ml-*` | margin sides | | |
| `gap-*` | `gap` | `gap-4` | `gap:4px` |
| `gap-x-*` | `column-gap` | | |
| `gap-y-*` | `row-gap` | | |

### Sizing

| Utility | Property | Example | Output |
|---|---|---|---|
| `w-*` | `width` | `w-full` | `width:100%` |
| `h-*` | `height` | `h-screen` | `height:100vh` |
| `min-w-*` | `min-width` | `min-w-[0]` | `min-width:0` |
| `min-h-*` | `min-height` | `min-h-screen` | `min-height:100vh` |
| `max-w-*` | `max-width` | `max-w-[42rem]` | `max-width:42rem` |
| `max-h-*` | `max-height` | | |

Special values: `full` = `100%`, `screen` = `100vw`/`100vh`, `auto` = `auto`.

### Colors

`text-*` handles both text color and font-size. Disambiguation: size tokens checked first, then numeric → `font-size`, then color token/value.

| Utility | Property | Example | Output |
|---|---|---|---|
| `bg-*` | `background` | `bg-surface` | `background:var(--color-surface)` |
| `text-*` | `color` | `text-accent` | `color:var(--color-accent)` |
| `text-*` | `font-size` | `text-lg` | `font-size:var(--text-lg)` |
| `border-*` | `border-color` | `border-line` | `border-color:var(--color-line)` |

### Typography

| Utility | Property | Example | Output |
|---|---|---|---|
| `text-16` | `font-size` | | `font-size:16px` |
| `font-*` | `font-family` | `font-ui` | `font-family:var(--font-ui)` |
| `font-bold` | `font-weight` | | `font-weight:700` |
| `font-semibold` | `font-weight` | | `font-weight:600` |
| `font-medium` | `font-weight` | | `font-weight:500` |
| `font-normal` | `font-weight` | | `font-weight:400` |
| `font-light` | `font-weight` | | `font-weight:300` |
| `leading-*` | `line-height` | `leading-tight` | `line-height:1.1` |
| `tracking-*` | `letter-spacing` | `tracking-widest` | `letter-spacing:0.1em` |
| `text-left/center/right` | `text-align` | | |
| `uppercase` | `text-transform` | | `text-transform:uppercase` |
| `no-underline` | `text-decoration` | | `text-decoration:none` |
| `list-none` | `list-style` | | `list-style:none` |

### Layout

| Utility | Output |
|---|---|
| `flex` | `display:flex` |
| `grid` | `display:grid` |
| `inline-block` | `display:inline-block` |
| `block` | `display:block` |
| `hidden` | `display:none` |
| `direction-column` | `flex-direction:column` |
| `direction-row` | `flex-direction:row` |
| `wrap` | `flex-wrap:wrap` |
| `nowrap` | `flex-wrap:nowrap` |
| `align-center` | `align-items:center` |
| `align-start` | `align-items:flex-start` |
| `align-end` | `align-items:flex-end` |
| `align-stretch` | `align-items:stretch` |
| `align-baseline` | `align-items:baseline` |
| `justify-center` | `justify-content:center` |
| `justify-between` | `justify-content:space-between` |
| `justify-start` | `justify-content:flex-start` |
| `justify-end` | `justify-content:flex-end` |
| `justify-around` | `justify-content:space-around` |
| `justify-evenly` | `justify-content:space-evenly` |
| `place-center` | `place-items:center` |
| `absolute / relative / fixed / sticky` | `position` |
| `overflow-hidden / overflow-auto` | `overflow` |
| `overflow-x-auto / overflow-y-auto` | `overflow-x/y` |
| `cursor-pointer` | `cursor:pointer` |
| `z-*` | `z-index` (numeric or arbitrary) |

### Border

| Utility | Output |
|---|---|
| `border` | `border-width:1px;border-style:solid` |
| `border-2` | `border-width:2px;border-style:solid` |
| `border-t / border-b / border-l / border-r` | one-side border |
| `border-0` | `border-width:0` |
| `border-solid / dashed / dotted / double / none` | `border-style` |
| `border-*` (color token) | `border-color:var(--color-*)` |
| `rounded-*` | `border-radius:var(--radius-*)` |

### Grid

| Utility | Output |
|---|---|
| `cols-3` | `grid-template-columns:repeat(3,1fr)` |
| `cols-[200px_1fr]` | `grid-template-columns:200px 1fr` |
| `rows-2` | `grid-template-rows:repeat(2,1fr)` |

### Position / inset

`top-*`, `right-*`, `bottom-*`, `left-*`, `inset-*`, `inset-x-*`, `inset-y-*` — all accept numeric (px), token, or arbitrary values.

### Effects

| Utility | Output |
|---|---|
| `scale-105` | `transform:scale(1.05)` |
| `opacity-75` | `opacity:0.75` |
| `shadow-*` | `box-shadow` (arbitrary value) |

---

## Variant grouping

The signature feature. Factor any repeated variant prefix out of a class string with `variant:(utilities)`.

**Without grouping:**
```html
<nav class="hover:bg-accent hover:text-on-accent hover:scale-105 hover:shadow-lg">
```

**With grouping:**
```html
<nav class="hover:(bg-accent text-on-accent scale-105 shadow-lg)">
```

Both compile to identical atomic CSS. The group is author-time sugar only — it never reaches the browser.

### Stacking variants

Chain variants with `:`. All conditions must hold. Read outer → inner:

```html
<!-- at tablet breakpoint AND on hover -->
<div class="tablet:hover:(bg-surface scale-105)">

<!-- dark mode AND focused -->
<input class="dark:focus:(bg-surface border-accent)">
```

### Nesting groups

Groups can contain further groups:

```html
<section class="tablet:(
  direction-row
  justify-between
  align-center
  hover:(bg-surface+8 scale-[1.02])
)">
```

Unfolds to: `tablet:direction-row`, `tablet:justify-between`, `tablet:align-center`, `tablet:hover:bg-surface+8`, `tablet:hover:scale-[1.02]`.

### Responsive variants

Breakpoint names from `tokens.screens` become variant prefixes:

```html
<div class="direction-column tablet:(direction-row gap-8) desktop:gap-12">
```

### Arbitrary selectors

Any CSS selector as a variant, written in square brackets with `&` as the subject:

```html
<!-- target child SVGs -->
<span class="[&>svg]:(w-[1rem] h-[1rem] text-muted)">

<!-- style based on a sibling checkbox state -->
<label class="[input:checked~&]:(text-accent font-bold)">
```

### Supported variant forms

| Form | Condition |
|---|---|
| `hover:` `focus:` `active:` `disabled:` | Pseudo-state |
| `first:` `last:` `odd:` `even:` | Structural pseudo |
| `dark:` `motion-safe:` `print:` | Media / feature query |
| `tablet:` `desktop:` *(any screen name)* | Responsive breakpoint |
| `[&>svg]:` `[.parent_&]:` | Arbitrary CSS selector |
| `variant:variant:` | Stacked (all must hold) |

---

## Utility grouping

Reduce noise for related multi-part declarations. Different from variant grouping: the prefix is a CSS property family name, not a variant condition.

### Padding and margin

```html
<div class="padding:(16 top:24 bottom:24 x:8)">
```

Expands to: `p-16 pt-24 pb-24 px-8`

Side keys: `top` (or `t`), `right` (`r`), `bottom` (`b`), `left` (`l`), `x`, `y`.

```html
<div class="margin:(auto x:16)">
```

### Text

```html
<h1 class="text:(xl bold center)">
```

Expands to: `text-xl font-bold text-center`

Plain values: size token or number → `text-*`; `left/center/right` → text-align; weight words (`bold`, `medium`, `semibold`, `light`, etc.) → `font-*`.

Keys: `size:`, `color:`, `weight:`, `align:`, `leading:`, `tracking:`.

### Border

```html
<div class="border:(1 solid accent)">
```

Expands to: `border-1 border-solid border-accent`

---

## Values: numeric, token, arbitrary, and dynamic

### Numeric (pixel-first)

A bare number is always pixels:

```
p-4     → padding: 4px
gap-12  → gap: 12px
w-250   → width: 250px
text-16 → font-size: 16px
```

Zero is unitless: `p-0` → `padding: 0`.

### Token names

A non-numeric value resolves through the appropriate token map:

```
gap-card      → gap: var(--space-card)
bg-surface    → background: var(--color-surface)
text-accent   → color: var(--color-accent)
rounded-md    → border-radius: var(--radius-md)
text-lg       → font-size: var(--text-lg)
font-ui       → font-family: var(--font-ui)
```

### Arbitrary values

Escape hatch for one-off values not in tokens. Compiled once to a stable class name.

```html
w-[347px]
h-[100dvh]
p-[2rem]
rounded-[10px]
cols-[200px_1fr]
bg-[oklch(72%_0.14_240)]
bg-[linear-gradient(120deg,#000,#111)]
```

Spaces inside arbitrary values are written as underscores: `bg-[rgb(0_0_0_/_50%)]`.

### Dynamic (CSS variable binding)

Write `utility-(--var-name)` to read a CSS custom property at runtime. The compiled class is stable regardless of the runtime value — no safelists, no dynamic class generation, no JIT overhead.

```jsx
// The class .w-\(--w\) is always the same; --w drives the value at runtime
<div className="w-(--w) h-(--h)" style={{ '--w': `${progress}%`, '--h': `${rowH}px` }} />
```

Use `vars()` as a typed helper for setting CSS variables in the `style` prop:

```jsx
import { vars } from 'beamcss'

<div className="w-(--w) bg-accent" style={vars({ w: `${pct}%` })} />
// vars({ w: '75%' }) -> { '--w': '75%' }
```

This is the answer to "Tailwind can't do runtime values cleanly." One atomic class, zero runtime framework, the value is a CSS custom property.

---

## Color system

Beam supports the full modern CSS color gamut anywhere a color value is accepted.

### Token colors

```html
bg-accent          → background: var(--color-accent)
text-muted         → color: var(--color-muted)
border-line        → border-color: var(--color-line)
```

### Color algebra

Append `+N` to lighten by `N%` using `color-mix(in oklab, ...)`, or `-N` to darken:

```html
bg-accent+12   → background: color-mix(in oklab, var(--color-accent), white 12%)
bg-surface+8   → background: color-mix(in oklab, var(--color-surface), white 8%)
bg-accent-20   → background: color-mix(in oklab, var(--color-accent), black 20%)
```

Append `/N` for alpha (transparency):

```html
bg-success/22  → background: color-mix(in oklab, var(--color-success) 22%, transparent)
```

### Arbitrary colors

Any modern CSS color syntax works inside brackets:

```html
bg-[#ff0000]
bg-[rgb(255_0_0)]
bg-[hsl(220_80%_56%)]
bg-[oklch(72%_0.14_240)]
bg-[color(display-p3_0.2_0.7_0.5)]
bg-[color-mix(in_srgb,var(--color-surface),white_8%)]
text-[rgb(255_255_255_/_80%)]
```

---

## Cascade layers

All Beam output is emitted under named `@layer` rules. This gives predictable specificity — you never fight `!important`. Layer order is fixed:

```css
@layer beam.reset, beam.tokens, beam.base, beam.utilities;
```

| Layer | Contents |
|---|---|
| `beam.reset` | Minimal reset: `box-sizing`, font smoothing, `body`, form elements, headings |
| `beam.tokens` | `:root { --color-*, --space-*, --radius-*, ... }` CSS custom properties |
| `beam.base` | Reserved; currently empty |
| `beam.utilities` | One atomic rule per unique `(class, declaration)` pair |

Every utility emits exactly one rule globally — `gap-4` appears once regardless of how many elements reference it.

Within `beam.utilities`, rules are sorted by class name for deterministic output. Responsive (`@media`) rules follow non-responsive rules.

---

## CLI

Install globally or use via `npx`:

```sh
npm install -g beamcss
# or
npx beam <command>
```

### `beam build`

Scan source files, compile class strings, write CSS output:

```sh
beam build \
  --config ./beam.config.ts \
  --content './src/**/*.{html,tsx,jsx}' \
  --out ./dist/beam.css
```

### `beam dev`

Watch mode — rebuilds and rewrites the output file when source or config changes:

```sh
beam dev \
  --config ./beam.config.ts \
  --content './src/**/*.{html,tsx,jsx}' \
  --out ./public/beam.css
```

### `beam check`

Validate that every class string in the scanned files compiles without errors. Returns a structured report. Designed as a CI gate and preflight for AI-generated markup.

```sh
beam check \
  --config ./beam.config.ts \
  --content './src/**/*.{html,tsx,jsx}' \
  --format json
```

Output:

```json
{
  "valid": true,
  "class_string_count": 42,
  "errors": [],
  "warnings": []
}
```

Exit code `0` = clean, `1` = errors found.

### `beam explain`

Inspect exactly how a class string parses and compiles. Shows every atom's selector, declaration, layer, and media query. Invaluable for debugging grouped variants and color algebra.

```sh
beam explain "flex direction-column hover:(bg-accent text-on-accent scale-105)" \
  --config ./beam.config.ts \
  --format json
```

```json
{
  "class_strings": [{
    "tokens": [
      {
        "kind": "utility",
        "atoms": [{ "declaration": "display:flex", "layer": "beam.utilities" }]
      },
      {
        "kind": "group",
        "variants": ["hover"],
        "atoms": [
          {
            "selector": ".hover\\:\\(bg-accent.text-on-accent.scale-105\\):hover",
            "declaration": "background:var(--color-accent)"
          },
          {
            "selector": ".hover\\:\\(bg-accent.text-on-accent.scale-105\\):hover",
            "declaration": "color:var(--color-on-accent)"
          },
          {
            "selector": ".hover\\:\\(bg-accent.text-on-accent.scale-105\\):hover",
            "declaration": "transform:scale(1.05)"
          }
        ]
      }
    ]
  }]
}
```

### `beam init`

Scaffold a new project:

```sh
beam init               # interactive
beam init --template vite
```

Creates `beam.config.ts`, installs packages, and wires up the plugin.

---

## Vite plugin

```ts
// vite.config.ts
import { beamcss } from '@beamcss/vite'

export default {
  plugins: [
    beamcss({
      // Path to beam.config.ts
      config: './beam.config.ts',
      // Glob patterns for files to scan for class strings
      content: ['./src/**/*.{html,tsx,jsx,vue,svelte,astro}'],
    }),
  ],
}
```

The plugin:

- Runs `beam build` during the Vite build
- Injects the CSS into the page via a `<style data-beamcss>` tag
- Supports HMR — editing a source file triggers an incremental rebuild

---

## PostCSS plugin

For webpack, Parcel, Rollup, Next.js, and other PostCSS-based setups:

```js
// postcss.config.js
module.exports = {
  plugins: {
    '@beamcss/postcss': {
      config: './beam.config.ts',
      content: ['./src/**/*.{html,tsx,jsx}'],
    },
  },
}
```

---

## Native Node binding

The `beamcss` package ships a prebuilt native `.node` addon compiled with napi-rs. This is the same approach as Lightning CSS and Tailwind Oxide — sub-millisecond compilation without spawning a subprocess.

```ts
import { compile, explain } from 'beamcss'

const result = compile(config, [
  'flex direction-column align-center gap-4 hover:(bg-accent text-on-accent)',
])

console.log(result.css)    // full CSS string
console.log(result.errors) // CompileMessage[] — { class_name, message }
```

```ts
import { explain } from 'beamcss'

const result = explain(config, ['hover:(bg-accent text-on-accent)'])
// result.class_strings[0].tokens[1].atoms[0].declaration
// -> "background:var(--color-accent)"
```

Both functions are synchronous and thread-safe. There is no global mutable state.

---

## Agent-native surfaces

Beam is designed to work well with AI coding agents.

### `beam check` as a preflight gate

Run `beam check --format json` before showing AI-generated UI to a user. A clean result (`"valid": true`, `"errors": []`) means every class string resolves against the design tokens — no typos, no undefined token references, no unsupported utilities.

### `beam explain` for debugging

When a generated class string needs verification, `beam explain` returns the full parse tree — variants, atoms, selectors, declarations, layers, and media queries — in structured JSON. No text parsing required.

### MCP server

`@beamcss/mcp` exposes `compile`, `explain`, and `check` as MCP tools. Agents can call them directly via tool use without shell access.

---

## Architecture

Beam is a **pnpm + Cargo monorepo**.

```
beamcss/
├── crates/
│   ├── beam_core/   # Rust parser + compiler
│   ├── beam_node/   # napi-rs Node binding
│   └── beam_cli/    # Standalone binary
├── packages/
│   ├── beamcss/     # Umbrella npm package
│   ├── vite/        # @beamcss/vite
│   ├── postcss/     # @beamcss/postcss
│   └── mcp/         # @beamcss/mcp
└── examples/
    ├── walking-skeleton/
    └── dashboard/
```

### Compilation pipeline

```
Source files
    ↓  scan class= / className= attributes
Class strings
    ↓  parse_classlist()          — parse into token tree
    ↓  expand_token()             — expand shortcuts and recipes
    ↓  unfold_token()             — distribute variants, flatten groups
Atom list  (variant-chain, base, selector-class)
    ↓  declaration_for_base()     — resolve CSS declaration
Atomic rules  (class, declaration, layer, wrappers, pseudos)
    ↓  BTreeSet deduplication + deterministic ordering
    ↓  emit_css()                 — write CSS under @layer beam.*
```

### How shortcuts and recipes expand

1. `parse_token()` produces a `ClassToken` — either a `Utility` or a `Group`.
2. `expand_token()` checks if the base matches a shortcut or recipe.
3. Shortcuts re-parse the aliased class string and return a `Group` with the same outer variant chain.
4. Recipes combine `base` and optionally a named variant's class string, then re-parse as a group.
5. The expanded token goes through `unfold_token()` like any other token.

Recursive shortcuts/recipes are detected and reported as errors. Maximum expansion depth is 16.

### Output determinism

Rules are sorted by `(class_name, declaration)` within each layer, producing identical output for the same input regardless of source file order. Responsive (`@media`) rules always follow non-responsive rules within `beam.utilities`.

---

## Development

### Prerequisites

- Rust stable (`rustup`)
- Node.js 18+
- pnpm 9+

### Setup

```sh
git clone https://github.com/garrettsiegel/beamcss
cd beamcss
pnpm install
pnpm build
```

### Commands

```sh
# Run all tests (Rust + JS)
pnpm test

# Type-check only
pnpm typecheck

# Rust tests only
cargo test --workspace

# Run a single Rust test
cargo test -p beam_core emits_tokens_and_utilities

# Rebuild the native Node binding after beam_core changes
pnpm --filter beamcss build:native

# CLI smoke tests
cargo run -p beam_cli --bin beam -- build \
  --config examples/walking-skeleton/beam.config.ts \
  --content examples/walking-skeleton \
  --out /tmp/beam-out.css

cargo run -p beam_cli --bin beam -- explain \
  "flex direction-column hover:(bg-accent text-on-accent)" \
  --config examples/walking-skeleton/beam.config.ts \
  --format json

# Security audits
cargo audit
pnpm audit --audit-level moderate
```

### Contributor notes

- **The grammar is the public API.** Changes to class-string parsing in `beam_core::parser` are semver-breaking.
- **`#![forbid(unsafe_code)]`** is enforced at the workspace level.
- **No `postinstall` scripts** anywhere — supply-chain security is first-class.
- **Snapshot tests** in `crates/beam_core/tests/` must pass; update golden fixture files when compiler output changes.
- **Rebuild the native binding** (`pnpm --filter beamcss build:native`) after any `beam_core` or `beam_node` change, before running JS plugin tests.

---

*Beam CSS — focused styles, zero scatter.*
