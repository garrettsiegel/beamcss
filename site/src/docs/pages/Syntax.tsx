import { Link } from 'react-router-dom'
import { CodeBlock } from '../CodeBlock'

export function Syntax() {
  return (
    <article data-docs-prose>
      <h1>Writing styles</h1>
      <p data-docs-lede>
        The class-string grammar is Beam's public API. It's small and regular: utilities, variant
        groups, utility groups, and four kinds of values.
      </p>

      <h2 id="variant-grouping">Variant grouping</h2>
      <p>
        The signature feature. Factor any repeated variant prefix out of a class string with{' '}
        <code>variant:(utilities)</code>. Both forms compile to identical atomic CSS — the group
        is author-time sugar that never reaches the browser.
      </p>
      <CodeBlock>{`<!-- without grouping -->
<nav class="hover:bg-accent hover:text-on-accent hover:scale-105 hover:shadow-lg">

<!-- with grouping -->
<nav class="hover:(bg-accent text-on-accent scale-105 shadow-lg)">`}</CodeBlock>

      <h3 id="stacking">Stacking</h3>
      <p>Chain variants with <code>:</code> — all conditions must hold. Read outer → inner.</p>
      <CodeBlock>{`<!-- at tablet breakpoint AND on hover -->
<div class="tablet:hover:(bg-surface scale-105)">

<!-- dark mode AND focused -->
<input class="dark:focus:(bg-surface border-accent)">`}</CodeBlock>

      <h3 id="nesting">Nesting</h3>
      <p>Groups can contain further groups:</p>
      <CodeBlock>{`<section class="tablet:(
  direction-row
  justify-between
  align-center
  hover:(bg-surface+8 scale-[1.02])
)">`}</CodeBlock>
      <p>
        Unfolds to <code>tablet:direction-row</code>, <code>tablet:justify-between</code>,{' '}
        <code>tablet:align-center</code>, <code>tablet:hover:bg-surface+8</code>,{' '}
        <code>tablet:hover:scale-[1.02]</code>.
      </p>

      <h3 id="responsive">Responsive &amp; arbitrary selectors</h3>
      <p>
        Breakpoint names from <code>tokens.screens</code> become variant prefixes. Any CSS selector
        works too, written in square brackets with <code>&amp;</code> as the subject.
      </p>
      <CodeBlock>{`<div class="direction-column tablet:(direction-row gap-8) desktop:gap-12">

<!-- target child SVGs -->
<span class="[&>svg]:(w-[1rem] h-[1rem] text-muted)">

<!-- style based on a sibling checkbox state -->
<label class="[input:checked~&]:(text-accent font-bold)">`}</CodeBlock>

      <h2 id="utility-grouping">Utility grouping</h2>
      <p>
        Reduce noise for related multi-part declarations. Here the prefix is a CSS property family
        name, not a variant condition.
      </p>
      <CodeBlock>{`<div class="padding:(16 top:24 bottom:24 x:8)">  <!-- p-16 pt-24 pb-24 px-8 -->
<h1 class="text:(xl bold center)">             <!-- text-xl font-bold text-center -->
<div class="border:(1 solid accent)">          <!-- border-1 border-solid border-accent -->`}</CodeBlock>
      <p>
        Side keys: <code>top</code>/<code>t</code>, <code>right</code>/<code>r</code>,{' '}
        <code>bottom</code>/<code>b</code>, <code>left</code>/<code>l</code>, <code>x</code>,{' '}
        <code>y</code>. Text keys: <code>size:</code>, <code>color:</code>, <code>weight:</code>,{' '}
        <code>align:</code>, <code>leading:</code>, <code>tracking:</code>.
      </p>

      <h2 id="values">Values</h2>
      <p>Four kinds, in resolution order:</p>
      <CodeBlock>{`<!-- numeric: a bare number is always pixels (0 is unitless) -->
p-4      → padding: 4px
w-250    → width: 250px
text-16  → font-size: 16px

<!-- token: a non-numeric value resolves through the token map -->
gap-card    → gap: var(--space-card)
bg-surface  → background: var(--color-surface)
rounded-md  → border-radius: var(--radius-md)

<!-- arbitrary: one-off escape hatch, spaces become underscores -->
w-[347px]  h-[100dvh]  cols-[200px_1fr]  bg-[oklch(72%_0.14_240)]`}</CodeBlock>

      <h3 id="dynamic">Dynamic values</h3>
      <p>
        Write <code>utility-(--var-name)</code> to read a CSS custom property at runtime. The
        compiled class is stable regardless of the runtime value — no safelists, no dynamic class
        generation.
      </p>
      <CodeBlock title="React">{`import { vars } from 'beamcss'

// the class .w-(--w) is always the same; --w drives the value at runtime
<div className="w-(--w) h-(--h)" style={{ '--w': \`\${progress}%\`, '--h': \`\${rowH}px\` }} />

// vars() is a typed helper: vars({ w: '75%' }) -> { '--w': '75%' }
<div className="w-(--w) bg-accent" style={vars({ w: \`\${pct}%\` })} />`}</CodeBlock>

      <h2 id="color-algebra">Color algebra</h2>
      <p>
        Adjust any token color inline. <code>+N</code> lightens, <code>-N</code> darkens (both via{' '}
        <code>color-mix(in oklab, …)</code>), and <code>/N</code> sets alpha.
      </p>
      <CodeBlock>{`bg-accent+12   → color-mix(in oklab, var(--color-accent), white 12%)
bg-accent-20   → color-mix(in oklab, var(--color-accent), black 20%)
bg-success/22  → color-mix(in oklab, var(--color-success) 22%, transparent)`}</CodeBlock>

      <p>
        Next: the <Link to="/docs/utilities">Utilities reference</Link>.
      </p>
    </article>
  )
}
