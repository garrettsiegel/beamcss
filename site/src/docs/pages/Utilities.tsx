import { Link } from 'react-router-dom'

export function Utilities() {
  return (
    <article data-docs-prose>
      <h1>Utilities reference</h1>
      <p data-docs-lede>
        Numeric values are pixels; named values resolve through tokens. Below is each utility
        family and the property it emits.
      </p>

      <h2 id="spacing">Spacing</h2>
      <table>
        <thead><tr><th>Utility</th><th>Property</th><th>Example</th></tr></thead>
        <tbody>
          <tr><td><code>p-* px-* py-* pt-* pr-* pb-* pl-*</code></td><td>padding</td><td><code>px-16</code> → <code>padding-inline:16px</code></td></tr>
          <tr><td><code>m-* mx-* my-* mt-* mr-* mb-* ml-*</code></td><td>margin</td><td><code>m-auto</code> → <code>margin:auto</code></td></tr>
          <tr><td><code>gap-* gap-x-* gap-y-*</code></td><td>gap</td><td><code>gap-4</code> → <code>gap:4px</code></td></tr>
        </tbody>
      </table>

      <h2 id="sizing">Sizing</h2>
      <table>
        <thead><tr><th>Utility</th><th>Property</th><th>Example</th></tr></thead>
        <tbody>
          <tr><td><code>w-* min-w-* max-w-*</code></td><td>width</td><td><code>w-full</code> → <code>width:100%</code></td></tr>
          <tr><td><code>h-* min-h-* max-h-*</code></td><td>height</td><td><code>h-screen</code> → <code>height:100vh</code></td></tr>
        </tbody>
      </table>
      <p>Special values: <code>full</code> = 100%, <code>screen</code> = 100vw/100vh, <code>auto</code> = auto.</p>

      <h2 id="colors">Colors</h2>
      <table>
        <thead><tr><th>Utility</th><th>Property</th><th>Example</th></tr></thead>
        <tbody>
          <tr><td><code>bg-*</code></td><td>background</td><td><code>bg-surface</code></td></tr>
          <tr><td><code>text-*</code> (token)</td><td>color</td><td><code>text-accent</code></td></tr>
          <tr><td><code>border-*</code> (token)</td><td>border-color</td><td><code>border-line</code></td></tr>
        </tbody>
      </table>
      <p>
        <code>text-*</code> handles both color and font-size: size tokens are checked first, then
        numeric → font-size, then color. Color algebra (<code>+12</code>, <code>-20</code>,{' '}
        <code>/22</code>) is covered in <Link to="/docs/syntax">Writing styles</Link>.
      </p>

      <h2 id="typography">Typography</h2>
      <table>
        <thead><tr><th>Utility</th><th>Output</th></tr></thead>
        <tbody>
          <tr><td><code>text-16</code></td><td><code>font-size:16px</code></td></tr>
          <tr><td><code>font-*</code> (token)</td><td><code>font-family:var(--font-*)</code></td></tr>
          <tr><td><code>font-bold/semibold/medium/normal/light</code></td><td><code>font-weight</code></td></tr>
          <tr><td><code>leading-*</code></td><td><code>line-height</code></td></tr>
          <tr><td><code>tracking-*</code></td><td><code>letter-spacing</code></td></tr>
          <tr><td><code>text-left/center/right</code></td><td><code>text-align</code></td></tr>
          <tr><td><code>uppercase · no-underline · list-none</code></td><td>transform / decoration / list-style</td></tr>
        </tbody>
      </table>

      <h2 id="layout">Layout</h2>
      <table>
        <thead><tr><th>Utility</th><th>Output</th></tr></thead>
        <tbody>
          <tr><td><code>flex · grid · block · inline-block · hidden</code></td><td><code>display</code></td></tr>
          <tr><td><code>direction-row/column · wrap · nowrap</code></td><td>flex-direction / flex-wrap</td></tr>
          <tr><td><code>align-center/start/end/stretch/baseline</code></td><td><code>align-items</code></td></tr>
          <tr><td><code>justify-center/between/start/end/around/evenly</code></td><td><code>justify-content</code></td></tr>
          <tr><td><code>place-center</code></td><td><code>place-items:center</code></td></tr>
          <tr><td><code>absolute · relative · fixed · sticky</code></td><td><code>position</code></td></tr>
          <tr><td><code>overflow-hidden/auto · overflow-x-auto · overflow-y-auto</code></td><td><code>overflow</code></td></tr>
          <tr><td><code>cursor-pointer · z-*</code></td><td>cursor / z-index</td></tr>
        </tbody>
      </table>

      <h2 id="border">Border</h2>
      <table>
        <thead><tr><th>Utility</th><th>Output</th></tr></thead>
        <tbody>
          <tr><td><code>border · border-2 · border-0</code></td><td><code>border-width</code> (+ solid style)</td></tr>
          <tr><td><code>border-t/b/l/r</code></td><td>one-side border</td></tr>
          <tr><td><code>border-solid/dashed/dotted/double/none</code></td><td><code>border-style</code></td></tr>
          <tr><td><code>rounded-*</code></td><td><code>border-radius:var(--radius-*)</code></td></tr>
        </tbody>
      </table>

      <h2 id="grid">Grid</h2>
      <table>
        <thead><tr><th>Utility</th><th>Output</th></tr></thead>
        <tbody>
          <tr><td><code>cols-3</code></td><td><code>grid-template-columns:repeat(3,1fr)</code></td></tr>
          <tr><td><code>cols-[200px_1fr]</code></td><td><code>grid-template-columns:200px 1fr</code></td></tr>
          <tr><td><code>rows-2</code></td><td><code>grid-template-rows:repeat(2,1fr)</code></td></tr>
        </tbody>
      </table>

      <h2 id="position">Position / inset</h2>
      <p>
        <code>top-*</code>, <code>right-*</code>, <code>bottom-*</code>, <code>left-*</code>,{' '}
        <code>inset-*</code>, <code>inset-x-*</code>, <code>inset-y-*</code> — all accept numeric
        (px), token, or arbitrary values.
      </p>

      <h2 id="effects">Effects</h2>
      <table>
        <thead><tr><th>Utility</th><th>Output</th></tr></thead>
        <tbody>
          <tr><td><code>scale-105</code></td><td><code>transform:scale(1.05)</code></td></tr>
          <tr><td><code>opacity-75</code></td><td><code>opacity:0.75</code></td></tr>
          <tr><td><code>shadow-*</code></td><td><code>box-shadow</code> (arbitrary value)</td></tr>
        </tbody>
      </table>
    </article>
  )
}
