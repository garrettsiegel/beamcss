# beamcss

[![npm](https://img.shields.io/npm/v/beamcss.svg?style=flat)](https://www.npmjs.com/package/beamcss)
[![license](https://img.shields.io/npm/l/beamcss.svg?style=flat)](https://github.com/garrettsiegel/beamcss/blob/main/LICENSE)
[![node](https://img.shields.io/node/v/beamcss.svg?style=flat)](https://nodejs.org)

**[beamcss.dev](https://beamcss.dev)** — Tailwind's authoring speed, without the wall of classes.

```html
<!-- Tailwind — the hover prefix repeated three times -->
<button class="rounded-md px-4 py-2 bg-blue-500 text-white hover:bg-blue-700 hover:shadow-lg hover:scale-105">

<!-- Beam — group it once -->
<button class="rounded-md px-4 py-2 bg-accent text-on-accent hover:(bg-accent+12 shadow-lg scale-105)">
```

## Why Beam?

- **Variant grouping** — `hover:(bg-accent text-on-accent scale-105)` instead of repeating `hover:` on every class
- **Utility grouping** — `padding:(16 top:24)` expands to `p-16 pt-24`; `text:(lg bold center)` to three atoms
- **Rust compiler** — sub-millisecond builds via a prebuilt napi-rs `.node` addon, no subprocess
- **Token-first** — one config file drives both CSS custom properties and type-safe utility names
- **Dynamic values** — `w-(--sidebar-width)` compiles to `var(--sidebar-width)`, no safelist needed
- **Color algebra** — `bg-accent+12` (lighten), `bg-accent/50` (alpha), `bg-surface~accent` (mix)
- **Works everywhere** — Vite plugin, PostCSS plugin, and a standalone CLI

---

## Install

```sh
# Vite
npm install beamcss @beamcss/vite

# PostCSS / webpack / Parcel / Next.js
npm install beamcss @beamcss/postcss
```

---

## Quick start

**1. Create `beam.config.ts`**

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

**2. Add the plugin**

```ts
// vite.config.ts
import { defineConfig } from 'vite'
import { beamcss } from '@beamcss/vite'

export default defineConfig({
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src'],   // directory or file paths to scan
    }),
  ],
})
```

**3. Write markup**

```html
<main class="grid place-center h-screen bg-base text-fg font-ui">
  <section class="flex direction-column align-center gap-4 p-6 bg-surface rounded-md
                  hover:(bg-surface+8 scale-105)
                  tablet:(direction-row gap-8)">
    <h1 class="text-lg text-accent">Hello Beam</h1>
  </section>
</main>
```

Beam scans `./src`, compiles only what you use, and injects atomic CSS into the page. No `@import` or virtual module needed.

---

## Syntax

### Variant grouping

```html
<!-- group any number of utilities under one variant -->
hover:(bg-accent text-on-accent scale-105)

<!-- stack and nest variants -->
tablet:(p-6 rounded-lg hover:(bg-surface scale-[1.02]))
```

### Utility grouping

```html
padding:(16 top:24 x:8)    <!-- p-16 pt-24 px-8 -->
text:(xl bold center)       <!-- text-xl font-bold text-center -->
border:(1 solid accent)     <!-- border border-solid border-accent -->
```

### Values

```html
p-4                         <!-- 4px (numeric = pixels) -->
bg-surface                  <!-- token name -->
w-[347px]                   <!-- arbitrary static value -->
w-(--sidebar-width)         <!-- dynamic: var(--sidebar-width) -->
```

### Color algebra

```html
bg-accent+12                <!-- lighten by 12% -->
bg-accent-20                <!-- darken by 20% -->
bg-surface/22               <!-- alpha 22% -->
bg-surface~accent           <!-- mix two tokens -->
```

---

## Configuration reference

```ts
defineConfig({
  presets: [],              // composable config fragments
  tokens: {
    spacing: {},            // named spacing tokens
    color: {},              // color tokens
    radius: {},             // border-radius tokens
    text: {},               // font-size tokens
    font: {},               // font-family tokens
    screens: {},            // breakpoint tokens (and arbitrary media queries)
  },
  shortcuts: {},            // named class-string aliases
  recipes: {},              // component variants: { base, variants }
  utilities: {              // enable/disable utility families
    layout: true,
    spacing: true,
    colors: true,
    typography: true,
    effects: true,
  },
  background: 'base',       // color token → body background
  foreground: 'fg',         // color token → body text color
})
```

---

## CLI

```sh
beam init                   # scaffold beam.config.ts
beam build                  # compile and write CSS
beam dev                    # watch mode
beam check                  # preflight validation (exit 1 on errors)
beam explain "hover:(bg-accent text-on-accent)"   # inspect parse tree
```

---

## API

| Export | Description |
|---|---|
| `defineConfig(config)` | Typed config helper — returns config as-is with full type inference |
| `vars(values)` | Converts an object to CSS custom property style props |
| `compile(config, classStrings)` | Synchronous native compile → `CompileResult` |
| `explain(config, classStrings)` | Synchronous parse-tree inspection → `ExplainResult` |
| `buildCss(options)` | Build entry point: native binding with Rust CLI fallback |
| `scanFiles(paths)` | Async directory scanner → class string array |
| `extractClassStrings(source)` | Extract class strings from a source string |
| `tailwindToBeamClassName(cls)` | Tailwind-to-Beam codemod utility |
| `describeBeamClass(cls, config)` | Hover/tooltip data for editor integrations |
| `suggestBeamClasses(prefix, config)` | Completion items for editor integrations |

**Types:** `BeamConfig`, `BeamTokens`, `BeamRecipe`, `BeamPreset`, `CompileResult`, `ExplainResult`, `BeamCompletion`, `BeamHover`, `CodemodResult`

---

## Links

- [beamcss.dev](https://beamcss.dev) — full docs & syntax reference
- [GitHub](https://github.com/garrettsiegel/beamcss)
- [@beamcss/vite](https://www.npmjs.com/package/@beamcss/vite) — Vite plugin
- [@beamcss/postcss](https://www.npmjs.com/package/@beamcss/postcss) — PostCSS plugin
- License: MIT
