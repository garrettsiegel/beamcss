# @beamcss/postcss

PostCSS plugin for [Beam CSS](https://www.npmjs.com/package/beamcss) — works with webpack, Parcel, Next.js, and any PostCSS-based build pipeline.

## Install

```sh
npm install beamcss @beamcss/postcss
```

## Usage

```js
// postcss.config.mjs
import beamcss from '@beamcss/postcss'

export default {
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src/**/*.{html,tsx,jsx,vue,svelte}'],
    }),
  ],
}
```

```js
// postcss.config.cjs
module.exports = {
  plugins: [
    require('@beamcss/postcss')({
      config: './beam.config.ts',
      content: ['./src/**/*.{html,tsx,jsx}'],
    }),
  ],
}
```

The plugin runs as a PostCSS `Once` plugin: it scans your source files for Beam class strings, compiles them to atomic CSS, and appends the result to the PostCSS root.

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `config` | `string` | `"beam.config.ts"` | Path to your Beam config file |
| `content` | `string[]` | `["."]` | Glob patterns for source files to scan |

## Peer dependency

```
postcss >= 8
```

## Links

- [beamcss (core)](https://www.npmjs.com/package/beamcss)
- [GitHub](https://github.com/garrettsiegel/beamcss)
- License: MIT
