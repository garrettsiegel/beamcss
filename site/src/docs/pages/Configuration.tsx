import { Link } from 'react-router-dom'
import { CodeBlock } from '../CodeBlock'

export function Configuration() {
  return (
    <article data-docs-prose>
      <h1>Configuration</h1>
      <p data-docs-lede>
        All configuration lives in a single <code>beam.config.ts</code>. Beam extracts it at
        build time with a brace-balanced JSON5 parser — no <code>ts-node</code> or dynamic import
        required.
      </p>

      <h2 id="tokens">Tokens</h2>
      <p>
        Tokens are the design system's source of truth. They compile to CSS custom properties and
        are referenced by name in utilities.
      </p>
      <CodeBlock title="beam.config.ts">{`import { defineConfig } from 'beamcss'

export default defineConfig({
  tokens: {
    // Named spacing values. Numeric utilities like \`gap-4\` are always 4px.
    spacing: { card: '1rem', section: '2rem' },

    // Color palette. Referenced by bg-*, text-*, border-*.
    color: {
      base: '#0b0b0c',
      surface: '#16161a',
      fg: '#e8e8ea',
      muted: '#6b7280',
      accent: '#3b82f6',
      'on-accent': '#ffffff',
    },

    radius: { sm: '4px', md: '8px', lg: '16px', full: '9999px' },
    text: { sm: '14px', base: '16px', lg: '20px', xl: '28px' },
    font: { ui: 'Inter, system-ui, sans-serif', mono: 'ui-monospace, monospace' },

    // Breakpoints become variant prefixes (tablet:, desktop:).
    screens: {
      tablet: '48rem',
      desktop: '64rem',
      'mobile-landscape': '(max-width:47.999rem) and (orientation:landscape)',
    },
  },

  // Token names for the body reset's background and color.
  background: 'base',
  foreground: 'fg',
})`}</CodeBlock>

      <p>Each token category compiles to prefixed CSS custom properties:</p>
      <table>
        <thead>
          <tr><th>Category</th><th>Variable prefix</th><th>Example</th></tr>
        </thead>
        <tbody>
          <tr><td><code>spacing</code></td><td><code>--space-*</code></td><td><code>--space-card: 1rem</code></td></tr>
          <tr><td><code>color</code></td><td><code>--color-*</code></td><td><code>--color-accent: #3b82f6</code></td></tr>
          <tr><td><code>radius</code></td><td><code>--radius-*</code></td><td><code>--radius-md: 8px</code></td></tr>
          <tr><td><code>text</code></td><td><code>--text-*</code></td><td><code>--text-lg: 20px</code></td></tr>
          <tr><td><code>font</code></td><td><code>--font-*</code></td><td><code>--font-ui: Inter, ...</code></td></tr>
          <tr><td><code>screens</code></td><td><code>--screen-*</code></td><td><code>--screen-tablet: 48rem</code></td></tr>
        </tbody>
      </table>

      <h2 id="shortcuts">Shortcuts</h2>
      <p>
        Named aliases for class strings. Shortcuts expand before compilation, so they accept any
        valid Beam syntax including variant groups.
      </p>
      <CodeBlock title="beam.config.ts">{`shortcuts: {
  card: 'flex direction-column gap-4 p-card bg-surface rounded-md',
  center: 'grid place-center',
  'sr-only': 'absolute w-[1px] h-[1px] overflow-hidden',
}`}</CodeBlock>
      <CodeBlock>{`<article class="card hover:(bg-surface+8 scale-105)">
<div class="center h-screen">`}</CodeBlock>
      <p>
        The shortcut class name becomes the CSS selector — <code>hover:card</code> applies all of
        <code>card</code>'s atoms under <code>:hover</code>.
      </p>

      <h2 id="recipes">Recipes</h2>
      <p>
        First-class component variants. A recipe has a <code>base</code> applied always and named{' '}
        <code>variants</code> applied selectively. Recipes replace <code>cva</code>,{' '}
        <code>tailwind-variants</code>, and similar runtime helpers.
      </p>
      <CodeBlock title="beam.config.ts">{`recipes: {
  button: {
    base: 'px-4 py-2 rounded-md hover:scale-105',
    variants: {
      primary: 'bg-accent text-on-accent hover:bg-accent+12',
      secondary: 'bg-surface border border-line hover:bg-surface+8',
      ghost: 'hover:bg-surface',
    },
  },
}`}</CodeBlock>
      <CodeBlock>{`<button class="button">Default</button>
<button class="button:primary">Primary</button>
<button class="hover:button:primary">Hover activates primary</button>`}</CodeBlock>

      <h2 id="presets">Presets</h2>
      <p>
        Plain config fragments merged before local config. Local tokens, shortcuts, recipes, and
        utility flags always win over preset values. Presets are plain objects — no plugins API,
        no side effects.
      </p>
      <CodeBlock title="beam.config.ts">{`export default defineConfig({
  presets: [
    {
      tokens: {
        spacing: { section: '2rem', page: '4rem' },
        color: { brand: '#ff6b35' },
      },
      shortcuts: { center: 'grid place-center' },
    },
  ],
  // Local tokens override preset tokens of the same key:
  tokens: { color: { brand: '#0070f3' } },
})`}</CodeBlock>

      <h2 id="utility-modules">Utility modules</h2>
      <p>
        Tree-shake utility families you don't use. All modules are enabled by default when{' '}
        <code>utilities</code> is omitted.
      </p>
      <CodeBlock title="beam.config.ts">{`utilities: {
  layout: true,      // flex, grid, position, overflow, sizing, border
  spacing: true,     // p, m, gap
  colors: true,      // bg, text (color), border (color)
  typography: true,  // font, text (size + align), leading, tracking
  effects: true,     // opacity, scale, shadow
}`}</CodeBlock>
      <p>
        Disabled utilities produce a compile error rather than silently emitting nothing — so
        misconfiguration is visible at build time, not in the browser.
      </p>

      <p>
        Next: <Link to="/docs/syntax">Writing styles</Link> covers the full class-string grammar.
      </p>
    </article>
  )
}
