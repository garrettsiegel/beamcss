import { Link } from 'react-router-dom'
import { CodeBlock } from '../CodeBlock'

export function Installation() {
  return (
    <article data-docs-prose>
      <h1>Installation</h1>
      <p data-docs-lede>
        Install two packages, drop in a config, add the plugin. Beam compiles on every save.
      </p>

      <h2 id="packages">Packages</h2>
      <table>
        <thead>
          <tr><th>Package</th><th>Purpose</th></tr>
        </thead>
        <tbody>
          <tr><td><code>beamcss</code></td><td>Core compiler, CLI, config types</td></tr>
          <tr><td><code>@beamcss/vite</code></td><td>Vite plugin with HMR</td></tr>
          <tr><td><code>@beamcss/postcss</code></td><td>PostCSS plugin for other bundlers</td></tr>
        </tbody>
      </table>

      <h2 id="vite">Vite</h2>
      <p>The recommended setup. Install the core package and the Vite plugin:</p>
      <CodeBlock title="terminal">{`npm install beamcss @beamcss/vite`}</CodeBlock>

      <p>Create <code>beam.config.ts</code> in your project root:</p>
      <CodeBlock title="beam.config.ts">{`import { defineConfig } from 'beamcss'

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
})`}</CodeBlock>

      <p>Add the plugin to your Vite config:</p>
      <CodeBlock title="vite.config.ts">{`import { beamcss } from '@beamcss/vite'

export default {
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src/**/*.{html,tsx,jsx,vue,svelte}'],
    }),
  ],
}`}</CodeBlock>

      <p>Now write markup — the plugin scans your files and injects the compiled CSS:</p>
      <CodeBlock title="index.html">{`<main class="grid place-center h-screen bg-base text-fg font-ui">
  <section class="flex direction-column align-center gap-4 p-6 bg-surface rounded-md
                  hover:(bg-surface+8 scale-105)
                  tablet:(direction-row justify-between gap-8)">
    <h1 class="text-lg text-accent">Hello Beam</h1>
  </section>
</main>`}</CodeBlock>

      <h2 id="postcss">PostCSS</h2>
      <p>
        For webpack, Parcel, Rollup, Next.js, and other PostCSS-based setups, install the PostCSS
        plugin instead:
      </p>
      <CodeBlock title="terminal">{`npm install beamcss @beamcss/postcss`}</CodeBlock>
      <CodeBlock title="postcss.config.js">{`module.exports = {
  plugins: {
    '@beamcss/postcss': {
      config: './beam.config.ts',
      content: ['./src/**/*.{html,tsx,jsx}'],
    },
  },
}`}</CodeBlock>

      <h2 id="cli">CLI</h2>
      <p>
        No bundler? Use the CLI directly via <code>npx</code> or a global install. Scaffold a
        project, then build or watch:
      </p>
      <CodeBlock title="terminal">{`# scaffold config + plugin wiring
npx beam init --template vite

# one-off build
npx beam build --config ./beam.config.ts --content './src/**/*.{html,tsx,jsx}' --out ./dist/beam.css

# watch mode
npx beam dev --config ./beam.config.ts --content './src/**/*.{html,tsx,jsx}' --out ./public/beam.css`}</CodeBlock>

      <p>
        See <Link to="/docs/configuration">Configuration</Link> to flesh out your design tokens,
        or <Link to="/docs/tooling">CLI &amp; integrations</Link> for the full command reference.
      </p>
    </article>
  )
}
