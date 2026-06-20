# @beamcss/vite

**[beamcss.dev](https://beamcss.dev)** — Vite plugin for [Beam CSS](https://www.npmjs.com/package/beamcss). Utility-first atomic CSS with variant grouping and a Rust compiler.

## Install

```sh
npm install beamcss @beamcss/vite
```

## Quick start

**1. Create `beam.config.ts`** in your project root:

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

**2. Add the plugin to `vite.config.ts`:**

```ts
import { defineConfig } from 'vite'
import { beamcss } from '@beamcss/vite'

export default defineConfig({
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src'],   // directory or file paths — not globs
    }),
  ],
})
```

**3. Write Beam classes in your components:**

```tsx
// App.tsx
export default function App() {
  return (
    <main className="grid place-center h-screen bg-base text-fg">
      <h1 className="text-lg text-accent font-bold">Hello Beam</h1>
      <button className="rounded-md px-4 py-2 bg-accent text-on-accent hover:(bg-accent+12 scale-105) transition">
        Get started
      </button>
    </main>
  )
}
```

Beam scans `./src`, compiles only what you use, and injects atomic CSS into the page automatically — no `@import` needed.

## Virtual module

Import compiled CSS as a virtual module if you need it in a specific bundle chunk:

```ts
import 'virtual:beamcss.css'
```

## HMR

Edits to watched source files trigger an incremental rebuild. The `virtual:beamcss.css` module is invalidated and the browser receives a hot update — no full page reload.

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `config` | `string` | `"beam.config.ts"` | Path to your Beam config file |
| `content` | `string[]` | `["."]` | Directories or files to scan for class strings |

> **Note:** `content` accepts directory paths and file paths, not glob patterns. Pass `['./src']` to scan your entire src folder recursively.

## Peer dependency

```
vite >= 5
```

## Links

- [beamcss.dev](https://beamcss.dev) — full docs & syntax reference
- [beamcss (core)](https://www.npmjs.com/package/beamcss)
- [GitHub](https://github.com/garrettsiegel/beamcss)
- License: MIT
