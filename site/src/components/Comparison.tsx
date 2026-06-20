export function Comparison() {
  return (
    <section className="py-2xl px-lg border-t border-line">
      <div className="container flex direction-column gap-xl">

        <div className="flex direction-column gap-sm">
          <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
            the pitch
          </span>
          <h2 className="text-2xl font-bold text-fg">Tailwind's speed. Without the class wall.</h2>
          <p className="text-base text-muted max-w-[42rem]">
            Write a repeated variant once and Beam expands it for you. Same atomic CSS,
            half the noise in your markup — and nothing extra ships to the browser.
          </p>
        </div>

        <div className="flex direction-column tablet:direction-row gap-lg w-full">
          <div className="flex direction-column gap-md w-full tablet:w-[48%]">
            <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-muted">
              Tailwind
            </span>
            <pre className="bg-surface rounded-md p-lg border border-line text-base w-full"><code><span className="text-muted">&lt;button class=&quot;</span><span className="text-fg">rounded-md
  px-4 py-2
  bg-blue-500
  text-white</span>
<span className="text-warning">  hover:bg-blue-700
  hover:shadow-lg
  hover:scale-105</span><span className="text-muted">&quot;&gt;</span></code></pre>
          </div>

          <div className="flex direction-column gap-md w-full tablet:w-[48%]">
            <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
              Beam
            </span>
            <pre className="bg-surface rounded-md p-lg border border-line text-base w-full"><code><span className="text-muted">&lt;button class=&quot;</span><span className="text-fg">rounded-md
  px-4 py-2
  bg-accent
  text-on-accent</span>
<span className="text-success">  hover:(
    bg-accent+12
    shadow-lg
    scale-105
  )</span><span className="text-muted">&quot;&gt;</span></code></pre>
          </div>
        </div>

        <p className="text-sm text-muted" style={{ fontFamily: 'var(--font-mono)' }}>
          → both compile to identical atomic CSS
        </p>

      </div>
    </section>
  )
}
