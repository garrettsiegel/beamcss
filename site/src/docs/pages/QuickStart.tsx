import { Link } from 'react-router-dom'
import { CodeBlock } from '../CodeBlock'

export function QuickStart() {
  return (
    <article data-docs-prose>
      <h1>Quick start</h1>
      <p data-docs-lede>
        Get from zero to styled in under two minutes. This guide uses Vite — for other
        setups see <Link to="/docs/installation">Installation</Link>.
      </p>

      <h2 id="step-1">1. Install</h2>
      <CodeBlock title="terminal">{`npm install beamcss @beamcss/vite`}</CodeBlock>

      <h2 id="step-2">2. Create your config</h2>
      <p>
        Add <code>beam.config.ts</code> to your project root. This is where your design
        tokens live — colors, spacing, type scale, breakpoints.
      </p>
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

      <h2 id="step-3">3. Add the Vite plugin</h2>
      <p>
        Tell the plugin where your config is and which directory to scan for class strings.
      </p>
      <CodeBlock title="vite.config.ts">{`import { defineConfig } from 'vite'
import { beamcss } from '@beamcss/vite'

export default defineConfig({
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src'],
    }),
  ],
})`}</CodeBlock>
      <p>
        The <code>content</code> option takes directory paths — Beam recursively scans
        them for <code>class</code> and <code>className</code> attributes in{' '}
        <code>.tsx</code>, <code>.jsx</code>, <code>.html</code>, <code>.vue</code>,{' '}
        and <code>.svelte</code> files.
      </p>

      <h2 id="step-4">4. Write your first component</h2>
      <p>
        Use your token names directly as utility values. No <code>@import</code> needed —
        Beam injects the compiled CSS automatically.
      </p>
      <CodeBlock title="src/App.tsx">{`export default function App() {
  return (
    <main className="grid place-center h-screen bg-base text-fg font-ui">
      <div className="flex direction-column align-center gap-4 p-8 bg-surface rounded-md">
        <h1 className="text-lg text-accent font-bold">Hello Beam</h1>
        <button className="rounded-md px-4 py-2 bg-accent text-on-accent
                           hover:(bg-accent+12 scale-105) transition">
          Get started
        </button>
      </div>
    </main>
  )
}`}</CodeBlock>

      <h2 id="step-5">5. Run the dev server</h2>
      <CodeBlock title="terminal">{`npm run dev`}</CodeBlock>
      <p>
        Beam scans <code>./src</code> on every save, compiles only the classes you use,
        and hot-reloads instantly — no full-page reload required.
      </p>

      <h2 id="next-steps">Next steps</h2>
      <ul>
        <li>
          <Link to="/docs/syntax">Writing styles</Link> — variant grouping,
          utility grouping, color algebra, dynamic values
        </li>
        <li>
          <Link to="/docs/configuration">Configuration</Link> — tokens, shortcuts,
          recipes, presets
        </li>
        <li>
          <Link to="/docs/installation">Installation</Link> — PostCSS, CLI, and
          other bundler setups
        </li>
      </ul>
    </article>
  )
}
