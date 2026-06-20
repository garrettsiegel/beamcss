import { Link } from 'react-router-dom'
import { CodeBlock } from '../CodeBlock'

export function Introduction() {
  return (
    <article data-docs-prose>
      <h1>Introduction</h1>
      <p data-docs-lede>
        Beam is a utility-first CSS framework with a Rust compiler. It gives you
        Tailwind's authoring speed with one key improvement: <strong>variant grouping</strong>{' '}
        lets you factor repeated prefixes out of your markup, so class strings read as
        grouped intent instead of repetitive soup. Everything compiles to deduped atomic
        CSS under cascade layers - zero runtime cost.
      </p>

      <h2 id="the-idea">The idea</h2>
      <p>
        The same hover state in Tailwind repeats the prefix on every utility. In Beam, the
        prefix lives once and the group expands at build time to identical atomic CSS.
      </p>
      <CodeBlock title="Tailwind vs Beam">{`<!-- Tailwind -->
<button class="rounded-md px-4 py-2 bg-blue-500 text-white hover:bg-blue-700 hover:shadow-lg hover:scale-105">

<!-- Beam - same output, the hover prefix lives once -->
<button class="rounded-md px-4 py-2 bg-accent text-on-accent hover:(bg-accent+12 shadow-lg scale-105)">`}</CodeBlock>

      <h2 id="what-makes-it-different">What makes it different</h2>
      <ul>
        <li>
          <strong>Variant grouping</strong> - <code>hover:(bg-accent text-base)</code> factors a
          repeated variant across many utilities.
        </li>
        <li>
          <strong>Utility grouping</strong> - <code>padding:(16 top:24)</code> keeps related
          declarations in one clause.
        </li>
        <li>
          <strong>Config composition</strong> - <code>shortcuts</code>, <code>recipes</code>,{' '}
          <code>presets</code>, and tree-shakeable utility modules.
        </li>
        <li>
          <strong>Dynamic values</strong> - <code>w-(--var)</code> binds a CSS custom property at
          runtime with one stable atomic class.
        </li>
      </ul>

      <h2 id="next">Next</h2>
      <p>
        Ready to try it? Head to <Link to="/docs/installation">Installation</Link> to get a
        working setup in under a minute, then see <Link to="/docs/syntax">Writing styles</Link>{' '}
        for the full grammar.
      </p>
    </article>
  )
}
