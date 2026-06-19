# @beamcss/vite

Vite plugin for [Beam CSS](https://www.npmjs.com/package/beamcss) — utility-first atomic CSS with variant grouping and a Rust compiler.

## Install

```sh
npm install beamcss @beamcss/vite
```

## Usage

```ts
// vite.config.ts
import { beamcss } from '@beamcss/vite'

export default {
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src/**/*.{html,tsx,jsx,vue,svelte,astro}'],
    }),
  ],
}
```

The plugin scans your source files for Beam class strings, compiles them to atomic CSS, and injects the result into every HTML page via a `<style data-beamcss>` tag.

## Virtual module

You can also import the compiled CSS as a virtual module if you want it in a specific bundle chunk:

```ts
import 'virtual:beamcss.css'
```

## HMR

Any edit to a watched source file triggers an incremental rebuild. The `virtual:beamcss.css` module is invalidated and the browser receives a hot update — no full page reload required.

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `config` | `string` | `"beam.config.ts"` | Path to your Beam config file |
| `content` | `string[]` | `["."]` | Glob patterns for source files to scan |

## Peer dependency

```
vite >= 5
```

## Links

- [beamcss (core)](https://www.npmjs.com/package/beamcss)
- [GitHub](https://github.com/garrettsiegel/beamcss)
- License: MIT
