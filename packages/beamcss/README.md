# beamcss

**[beamcss.dev](https://beamcss.dev)** — Atomic CSS without the class wall.

Beam compiles utility class strings to atomic CSS under cascade layers. **Variant grouping** lets you factor repeated prefixes — `hover:(bg-accent text-on-accent scale-105)` instead of three separate `hover:` classes. **Utility grouping** compresses related declarations — `padding:(16 top:24)` expands to `p-16 pt-24`. Everything is author-time sugar that becomes plain atoms at ship time, deduped globally, zero runtime cost.

```html
<!-- Tailwind -->
<button class="rounded-md px-4 py-2 bg-blue-500 text-white hover:bg-blue-700 hover:shadow-lg hover:scale-105">

<!-- Beam — the hover prefix lives once -->
<button class="rounded-md px-4 py-2 bg-accent text-on-accent hover:(bg-accent+12 shadow-lg scale-105)">
```

---

## Install

```sh
# Vite projects
npm install beamcss @beamcss/vite

# PostCSS / webpack / Parcel / Next.js
npm install beamcss @beamcss/postcss
```

Node 18+ required. Uses a prebuilt napi-rs `.node` addon — no subprocess, sub-millisecond compile.

---

## Quick start

**1. Install**

```sh
npm install beamcss @beamcss/vite
```

**2. Create `beam.config.ts`**

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

**3. Add the Vite plugin to `vite.config.ts`**

```ts
import { defineConfig } from 'vite'
import { beamcss } from '@beamcss/vite'

export default defineConfig({
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src'],   // directory or file paths to scan for class strings
    }),
  ],
})
```

**4. Write markup**

```html
<main class="grid place-center h-screen bg-base text-fg font-ui">
  <section class="flex direction-column align-center gap-4 p-6 bg-surface rounded-md
                  hover:(bg-surface+8 scale-105)
                  tablet:(direction-row gap-8)">
    <h1 class="text-lg text-accent">Hello Beam</h1>
  </section>
</main>
```

That's it — Beam scans `./src`, compiles only what you use, and injects atomic CSS into the page. No `@import` or virtual module needed.

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
p-4                         <!-- 4px (pixel-first numeric) -->
bg-surface                  <!-- token name -->
w-[347px]                   <!-- arbitrary static value -->
w-(--sidebar-width)         <!-- dynamic: var(--sidebar-width) -->
```

### Color algebra

```html
bg-accent+12                <!-- lighten accent by 12% -->
bg-accent-20                <!-- darken accent by 20% -->
bg-surface/22               <!-- alpha 22% -->
```

---

## Exports

| Export | Description |
|---|---|
| `defineConfig(config)` | Typed config helper — returns the config as-is with full type inference |
| `vars(values)` | Converts an object to CSS custom property style props |
| `compile(config, classStrings)` | Synchronous native compile → `CompileResult` |
| `explain(config, classStrings)` | Synchronous parse-tree inspection → `ExplainResult` |
| `buildCss(options)` | Top-level build: tries native binding, falls back to Rust CLI |
| `scanFiles(options)` | Async glob scanner → class string array |
| `extractClassStrings(source)` | Sync class-string extractor from source text |
| `tailwindToBeamClassName(cls)` | Tailwind-to-Beam codemod utility |
| `describeBeamClass(cls, config)` | Hover/tooltip data for editor integrations |
| `suggestBeamClasses(prefix, config)` | Completion items for editor integrations |

**Types:** `BeamConfig`, `BeamTokens`, `BeamRecipe`, `BeamPreset`, `CompileResult`, `ExplainResult`, `BeamCompletion`, `BeamHover`, `CodemodResult`

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

## Output

Beam emits atomic CSS under four cascade layers, in fixed order:

```css
@layer beam.reset, beam.tokens, beam.base, beam.utilities;
```

Every class string compiles to one declaration per atom, globally deduped.

---

## Links

- [beamcss.dev](https://beamcss.dev) — full docs & syntax reference
- [GitHub](https://github.com/garrettsiegel/beamcss)
- [Vite plugin: @beamcss/vite](https://www.npmjs.com/package/@beamcss/vite)
- [PostCSS plugin: @beamcss/postcss](https://www.npmjs.com/package/@beamcss/postcss)
- License: MIT
