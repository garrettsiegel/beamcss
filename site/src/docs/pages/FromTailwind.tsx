import { Link } from 'react-router-dom'
import { CodeBlock } from '../CodeBlock'

export function FromTailwind() {
  return (
    <article data-docs-prose>
      <h1>Coming from Tailwind</h1>
      <p data-docs-lede>
        Most Beam utilities look exactly like Tailwind — <code>flex</code>, <code>gap-4</code>,{' '}
        <code>px-6</code>, <code>rounded-md</code>, <code>justify-between</code>. A handful of
        layout names differ because Beam uses the underlying CSS property vocabulary instead of
        Tailwind shorthand. This page is the complete map.
      </p>

      <h2 id="naming-differences">Naming differences</h2>
      <p>
        These are the utilities that <em>look</em> different. Everything else in Beam is the same
        class name you already know.
      </p>

      <table>
        <thead>
          <tr>
            <th>Tailwind</th>
            <th>Beam</th>
            <th>CSS property</th>
          </tr>
        </thead>
        <tbody>
          <tr><td><code>flex-col</code></td><td><code>direction-column</code></td><td><code>flex-direction: column</code></td></tr>
          <tr><td><code>flex-row</code></td><td><code>direction-row</code></td><td><code>flex-direction: row</code></td></tr>
          <tr><td><code>flex-wrap</code></td><td><code>wrap</code></td><td><code>flex-wrap: wrap</code></td></tr>
          <tr><td><code>flex-nowrap</code></td><td><code>nowrap</code></td><td><code>flex-wrap: nowrap</code></td></tr>
          <tr><td><code>items-center</code></td><td><code>align-center</code></td><td><code>align-items: center</code></td></tr>
          <tr><td><code>items-start</code></td><td><code>align-start</code></td><td><code>align-items: flex-start</code></td></tr>
          <tr><td><code>items-end</code></td><td><code>align-end</code></td><td><code>align-items: flex-end</code></td></tr>
          <tr><td><code>items-stretch</code></td><td><code>align-stretch</code></td><td><code>align-items: stretch</code></td></tr>
          <tr><td><code>items-baseline</code></td><td><code>align-baseline</code></td><td><code>align-items: baseline</code></td></tr>
          <tr><td><code>place-items-center</code></td><td><code>place-center</code></td><td><code>place-items: center</code></td></tr>
          <tr><td><code>grid-cols-3</code></td><td><code>cols-3</code></td><td><code>grid-template-columns: repeat(3,1fr)</code></td></tr>
          <tr><td><code>grid-cols-[200px_1fr]</code></td><td><code>cols-[200px_1fr]</code></td><td><code>grid-template-columns: 200px 1fr</code></td></tr>
          <tr><td><code>grid-rows-2</code></td><td><code>rows-2</code></td><td><code>grid-template-rows: repeat(2,1fr)</code></td></tr>
        </tbody>
      </table>

      <p>
        The logic: Tailwind uses compound shorthand (<code>flex-col</code>, <code>items-*</code>).
        Beam uses the CSS property name directly (<code>direction-*</code>, <code>align-*</code>).
        Once you know the pattern you can usually guess the Beam name from the CSS spec.
      </p>

      <h2 id="same-utilities">What stays the same</h2>
      <p>The vast majority of your Tailwind muscle memory transfers directly:</p>
      <table>
        <thead>
          <tr><th>Category</th><th>Examples (identical in both)</th></tr>
        </thead>
        <tbody>
          <tr><td>Display</td><td><code>flex · grid · block · inline-block · hidden · inline-flex · inline-grid</code></td></tr>
          <tr><td>Justify content</td><td><code>justify-center · justify-between · justify-start · justify-end · justify-around · justify-evenly</code></td></tr>
          <tr><td>Spacing</td><td><code>p-4 · px-6 · py-2 · pt-8 · gap-4 · gap-x-2 · m-auto · mx-4</code></td></tr>
          <tr><td>Sizing</td><td><code>w-full · h-screen · max-w-* · min-h-*</code></td></tr>
          <tr><td>Typography</td><td><code>font-bold · font-semibold · leading-* · tracking-* · uppercase · text-center</code></td></tr>
          <tr><td>Border</td><td><code>border · border-2 · border-solid · border-dashed · rounded-md · rounded-full</code></td></tr>
          <tr><td>Position</td><td><code>relative · absolute · fixed · sticky · top-* · inset-*</code></td></tr>
          <tr><td>Effects</td><td><code>opacity-75 · scale-105 · cursor-pointer · overflow-hidden · z-10</code></td></tr>
        </tbody>
      </table>

      <h2 id="colors">Colors: semantic tokens instead of scales</h2>
      <p>
        Tailwind uses numeric scales (<code>bg-blue-500</code>). Beam uses semantic design tokens
        from your config — names like <code>bg-surface</code>, <code>text-accent</code>,{' '}
        <code>border-line</code>. You define the token map once and reference meaning, not
        a hardcoded shade.
      </p>
      <CodeBlock title="Tailwind vs Beam — colors">{`<!-- Tailwind: hardcoded scale -->
<div class="bg-white text-gray-900 border-gray-200">

<!-- Beam: semantic tokens from your beam.config.ts -->
<div class="bg-surface text-fg border-line">`}</CodeBlock>
      <p>
        This means a dark-mode toggle or rebrand changes <em>one token</em>, not every class in
        your codebase.
      </p>

      <h2 id="what-you-gain">What you gain over Tailwind</h2>

      <h3 id="variant-grouping">1. Variant grouping</h3>
      <p>
        Factor a repeated variant prefix out of a class string. Both forms produce identical atomic
        CSS — the group is author-time sugar only.
      </p>
      <CodeBlock title="Tailwind vs Beam — hover state">{`<!-- Tailwind: prefix repeats on every utility -->
<button class="rounded-md px-4 py-2 bg-blue-500 text-white
  hover:bg-blue-700 hover:shadow-lg hover:scale-105">

<!-- Beam: prefix lives once, group expands at build time -->
<button class="rounded-md px-4 py-2 bg-accent text-on-accent
  hover:(bg-accent+12 shadow-lg scale-105)">`}</CodeBlock>
      <p>
        Variants stack — <code>tablet:hover:(bg-surface scale-105)</code> means "at tablet
        breakpoint AND on hover".
      </p>

      <h3 id="utility-grouping">2. Utility grouping</h3>
      <p>
        Related declarations stay together without repeating the property family prefix.
      </p>
      <CodeBlock title="Tailwind vs Beam — spacing">{`<!-- Tailwind -->
<article class="p-4 pt-6 pb-6 px-8">

<!-- Beam -->
<article class="padding:(4 top:6 bottom:6 x:8)">`}</CodeBlock>

      <h3 id="color-algebra">3. Color algebra</h3>
      <p>
        Lighten, darken, or set alpha on any token inline — no intermediate Tailwind arbitrary
        value needed.
      </p>
      <CodeBlock>{`bg-accent+12   → color-mix(in oklab, var(--color-accent), white 12%)
bg-accent-20   → color-mix(in oklab, var(--color-accent), black 20%)
bg-accent/50   → color-mix(in oklab, var(--color-accent) 50%, transparent)`}</CodeBlock>

      <h3 id="dynamic-vars">4. Dynamic CSS custom properties</h3>
      <p>
        Bind a CSS variable at runtime with a stable atomic class — no JIT purge issues, no
        safelisting.
      </p>
      <CodeBlock title="React">{`// Tailwind: dynamic class string, needs safelisting or JIT
<div style={{ width: \`\${pct}%\` }}>

// Beam: one stable class, value is driven by the CSS variable
<div className="w-(--progress)" style={{ '--progress': \`\${pct}%\` }}>`}</CodeBlock>

      <h2 id="real-world">Side by side: a real component</h2>
      <CodeBlock title="Card component — Tailwind vs Beam">{`<!-- Tailwind -->
<div class="flex flex-col items-center gap-4 p-6
  bg-white rounded-lg border border-gray-200
  hover:shadow-lg hover:border-blue-200">
  <h2 class="text-xl font-bold text-gray-900">Title</h2>
  <p class="text-sm text-gray-500">Description</p>
</div>

<!-- Beam -->
<div class="flex direction-column align-center gap-4 p-6
  bg-surface rounded-lg border border-line
  hover:(shadow-lg border-accent/20)">
  <h2 class="text-xl font-bold text-fg">Title</h2>
  <p class="text-sm text-muted">Description</p>
</div>`}</CodeBlock>

      <h2 id="shortcuts">Tip: recreate Tailwind names as shortcuts</h2>
      <p>
        If you want to keep writing <code>flex-col</code> and <code>items-center</code> during a
        migration, define them as shortcuts in your config:
      </p>
      <CodeBlock title="beam.config.ts">{`import { defineConfig } from 'beamcss'

export default defineConfig({
  shortcuts: {
    'flex-col':    'direction-column',
    'flex-row':    'direction-row',
    'items-center': 'align-center',
    'items-start':  'align-start',
    'items-end':    'align-end',
  },
})`}</CodeBlock>
      <p>
        See <Link to="/docs/configuration">Configuration</Link> for the full shortcuts and recipes
        API.
      </p>
    </article>
  )
}
