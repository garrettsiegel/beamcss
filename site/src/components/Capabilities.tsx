const grammar = [
  {
    title: 'Variant grouping',
    snippet: 'hover:(bg-accent text-on-accent)',
    desc: 'Factor a repeated prefix across many utilities - hover, focus, responsive, dark.',
  },
  {
    title: 'Utility grouping',
    snippet: 'padding:(16 top:24)',
    desc: 'Keep related declarations in one clause instead of scattered atoms.',
  },
  {
    title: 'Dynamic values',
    snippet: 'w-(--w)',
    desc: 'Bind a CSS variable at runtime with one stable class. Zero runtime cost.',
  },
]

const platform = [
  {
    title: 'Rust core',
    desc: 'Compiles in milliseconds via a native napi-rs binding - no subprocess, no cold start.',
  },
  {
    title: 'Zero runtime',
    desc: 'Output is static atomic CSS under cascade layers. No JavaScript ships to the browser.',
  },
  {
    title: 'Composable config',
    desc: 'Tokens, shortcuts, recipes, and presets compose design systems across projects.',
  },
]

export function Capabilities() {
  return (
    <section className="py-2xl px-lg border-t border-line">
      <div className="container flex direction-column gap-xl">

        <div className="flex direction-column gap-sm">
          <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
            how it works
          </span>
          <h2 className="text-2xl font-bold text-fg">Write the grammar. Ship plain CSS.</h2>
          <p className="text-base text-muted max-w-[42rem]">
            A small, regular grammar compiles to deduped atomic CSS under cascade layers.
            Parse → unfold → emit. Nothing runs in the browser.
          </p>
        </div>

        {/* Proof: the compiled output + platform benefits */}
        <div className="flex direction-column tablet:direction-row gap-xl w-full">
          <pre style={{ minWidth: 0 }} className="bg-surface rounded-md p-xl border border-line text-base w-full tablet:w-[56%]"><code><span className="text-muted">@layer </span><span className="text-accent">beam.utilities</span><span className="text-muted"> {'{'}{'\n'}</span><span className="text-fg">  .flex </span><span className="text-muted">{'{'} </span><span className="text-success">display: flex</span><span className="text-muted"> {'}'}{'\n'}</span><span className="text-fg">  .bg-accent </span><span className="text-muted">{'{'}{'\n'}</span><span className="text-success">    background: var(--color-accent){'\n'}</span><span className="text-muted">  {'}'}{'\n'}</span><span className="text-fg">  .hover\:\(bg-accent\+12\):hover </span><span className="text-muted">{'{'}{'\n'}</span><span className="text-success">    background: color-mix({'\n'}      in oklab,{'\n'}      var(--color-accent), white 12%{'\n'}    ){'\n'}</span><span className="text-muted">  {'}'}{'\n'}{'}'}</span></code></pre>

          <div className="flex direction-column gap-xl w-full tablet:w-[40%]">
            {platform.map(({ title, desc }) => (
              <div key={title} className="flex direction-column gap-xs">
                <span className="text-base font-semibold text-fg">{title}</span>
                <span className="text-base text-muted">{desc}</span>
              </div>
            ))}
          </div>
        </div>

        {/* The grammar, as compact cards (no wall of code) */}
        <div className="grid cols-1 tablet:cols-3 gap-md">
          {grammar.map(({ title, snippet, desc }) => (
            <div
              key={title}
              data-capability-card
              className="flex direction-column gap-sm p-lg bg-surface rounded-md border border-line hover:(border-accent)"
            >
              <h3 className="text-lg font-semibold text-fg">{title}</h3>
              <code style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
                {snippet}
              </code>
              <p className="text-sm text-muted">{desc}</p>
            </div>
          ))}
        </div>

      </div>
    </section>
  )
}
