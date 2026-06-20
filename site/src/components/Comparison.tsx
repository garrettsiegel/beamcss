export function Comparison() {
  return (
    <section className="py-2xl px-lg border-t border-line">
      <div className="container flex direction-column gap-2xl">

        {/* — variant grouping — */}
        <div className="flex direction-column gap-xl">
          <div className="flex direction-column gap-sm">
            <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
              variant grouping
            </span>
            <h2 className="text-2xl font-bold text-fg">Same output. Half the noise.</h2>
            <p className="text-base text-muted max-w-[42rem]">
              Write a variant prefix once and group the utilities it governs. The compiler
              expands it to identical atomic CSS — less markup, zero runtime overhead.
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
    bg-accent-20
    shadow-lg
    scale-105
  )</span><span className="text-muted">&quot;&gt;</span></code></pre>
            </div>
          </div>

          <p className="text-sm text-muted" style={{ fontFamily: 'var(--font-mono)' }}>
            → both compile to identical atomic CSS
          </p>
        </div>

        {/* — naming differences — */}
        <div className="flex direction-column gap-xl">
          <div className="flex direction-column gap-sm">
            <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
              css-first names
            </span>
            <h2 className="text-2xl font-bold text-fg">A few names are different — on purpose.</h2>
            <p className="text-base text-muted max-w-[42rem]">
              Layout utilities use the underlying CSS property vocabulary instead of Tailwind
              shorthand. <code>flex-col</code> becomes <code>direction-column</code>,{' '}
              <code>items-center</code> becomes <code>align-center</code>. Once you know the
              pattern, the CSS spec is your cheat sheet.{' '}
              <a href="/docs/from-tailwind" style={{ color: 'var(--color-accent)' }}>Full mapping →</a>
            </p>
          </div>

          <div className="flex direction-column tablet:direction-row gap-lg w-full">
            <div className="flex direction-column gap-md w-full tablet:w-[48%]">
              <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-muted">
                Tailwind
              </span>
              <pre className="bg-surface rounded-md p-lg border border-line text-base w-full"><code><span className="text-muted">&lt;div class=&quot;</span><span className="text-fg">flex </span><span className="text-warning">flex-col items-center</span><span className="text-fg">
  gap-4 p-6 bg-white
  rounded-lg border
  border-gray-200</span>
<span className="text-warning">  md:flex-row
  md:justify-between</span><span className="text-muted">&quot;&gt;</span></code></pre>
            </div>

            <div className="flex direction-column gap-md w-full tablet:w-[48%]">
              <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
                Beam
              </span>
              <pre className="bg-surface rounded-md p-lg border border-line text-base w-full"><code><span className="text-muted">&lt;div class=&quot;</span><span className="text-fg">flex </span><span className="text-success">direction-column align-center</span><span className="text-fg">
  gap-4 p-6 bg-surface
  rounded-lg border
  border-line</span>
<span className="text-success">  tablet:(
    direction-row
    justify-between
  )</span><span className="text-muted">&quot;&gt;</span></code></pre>
            </div>
          </div>

          <p className="text-sm text-muted" style={{ fontFamily: 'var(--font-mono)' }}>
            → same CSS output, semantic color tokens, grouped responsive variant
          </p>
        </div>

      </div>
    </section>
  )
}
