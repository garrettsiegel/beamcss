import { CodeBlock } from '../CodeBlock'

export function Tooling() {
  return (
    <article data-docs-prose>
      <h1>CLI &amp; integrations</h1>
      <p data-docs-lede>
        Beam ships a CLI, bundler plugins, a native Node binding, and agent-native surfaces - all
        backed by the same Rust compiler.
      </p>

      <h2 id="cli">CLI</h2>
      <p>Install globally or run via <code>npx beam &lt;command&gt;</code>.</p>
      <table>
        <thead><tr><th>Command</th><th>Purpose</th></tr></thead>
        <tbody>
          <tr><td><code>beam init</code></td><td>Scaffold config, install packages, wire the plugin (<code>--template vite</code>)</td></tr>
          <tr><td><code>beam build</code></td><td>Scan files, compile class strings, write CSS</td></tr>
          <tr><td><code>beam dev</code></td><td>Watch mode - rebuild on source/config change</td></tr>
          <tr><td><code>beam check</code></td><td>Validate every class string compiles; structured report, CI-friendly</td></tr>
          <tr><td><code>beam explain</code></td><td>Show how a class string parses and compiles</td></tr>
        </tbody>
      </table>
      <CodeBlock title="terminal">{`beam check --config ./beam.config.ts --content ./src --format json`}</CodeBlock>
      <CodeBlock title="output">{`{
  "valid": true,
  "class_string_count": 42,
  "errors": [],
  "warnings": []
}`}</CodeBlock>
      <p>Exit code <code>0</code> = clean, <code>1</code> = errors found.</p>

      <h2 id="vite-plugin">Vite plugin</h2>
      <p>
        Runs <code>beam build</code> during the Vite build, injects the CSS via a{' '}
        <code>&lt;style data-beamcss&gt;</code> tag, and supports HMR with incremental rebuilds.
      </p>
      <CodeBlock title="vite.config.ts">{`import { beamcss } from '@beamcss/vite'

export default {
  plugins: [
    beamcss({
      config: './beam.config.ts',
      content: ['./src'],
    }),
  ],
}`}</CodeBlock>

      <h2 id="postcss-plugin">PostCSS plugin</h2>
      <CodeBlock title="postcss.config.js">{`module.exports = {
  plugins: {
    '@beamcss/postcss': {
      config: './beam.config.ts',
      content: ['./src'],
    },
  },
}`}</CodeBlock>

      <h2 id="native-binding">Native Node binding</h2>
      <p>
        The <code>beamcss</code> package ships a prebuilt napi-rs <code>.node</code> addon - the
        same approach as Lightning CSS and Tailwind Oxide. <code>compile</code> and{' '}
        <code>explain</code> are synchronous, thread-safe, and stateless.
      </p>
      <CodeBlock title="Node">{`import { compile, explain } from 'beamcss'

const result = compile(config, [
  'flex direction-column align-center gap-4 hover:(bg-accent text-on-accent)',
])
console.log(result.css)    // full CSS string
console.log(result.errors) // CompileMessage[] - { class_name, message }`}</CodeBlock>

      <h2 id="agents">Agent-native surfaces</h2>
      <p>Beam is designed to work well with AI coding agents.</p>
      <ul>
        <li>
          <strong><code>beam check</code> as a preflight gate</strong> - run before showing
          AI-generated UI; a clean result means every class resolves against the tokens.
        </li>
        <li>
          <strong><code>beam explain</code> for debugging</strong> - returns the full parse tree
          (variants, atoms, selectors, declarations, layers) as structured JSON.
        </li>
        <li>
          <strong><code>@beamcss/mcp</code></strong> - exposes <code>compile</code>,{' '}
          <code>explain</code>, and <code>check</code> as MCP tools, callable directly via tool use.
        </li>
        <li>
          <strong>Tailwind → Beam codemod</strong> and <code>llms.txt</code> / <code>llms-full.txt</code>{' '}
          for machine-readable docs.
        </li>
      </ul>

      <h2 id="cascade-layers">Cascade layers</h2>
      <p>
        All output is emitted under named <code>@layer</code> rules for predictable specificity -
        you never fight <code>!important</code>. The order is fixed:
      </p>
      <CodeBlock title="CSS">{`@layer beam.reset, beam.tokens, beam.base, beam.utilities;`}</CodeBlock>
      <p>
        Every utility emits exactly one rule globally - <code>gap-4</code> appears once regardless
        of how many elements reference it.
      </p>
    </article>
  )
}
