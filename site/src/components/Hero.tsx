import { Starfield } from './Starfield'

export function Hero() {
  return (
    <section data-hero-stage className="px-lg border-b border-line">
      <div data-hero-grid className="container">
        <div data-hero-topline>
          <span data-mission-label style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-accent">
            utility-first css / v0.1.1
          </span>
          <span style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-muted">
            grouped syntax / atomic output
          </span>
        </div>

        <div data-wordmark-stage>
          <Starfield />
          <h1 data-hero-wordmark className="font-bold text-fg" aria-label="BeamCSS">
            <span data-wordmark-line>BEAM</span>
            <span data-wordmark-line>CSS</span>
          </h1>
          <span data-vertical-meta style={{ fontFamily: 'var(--font-mono)' }}>
            focused styles / zero scatter
          </span>
        </div>

        <div data-hero-bottom>
          <div data-hero-copy className="flex direction-column gap-md">
            <p data-hero-kicker className="text-lg text-fg">
              Tailwind's speed, without the class wall.
            </p>

            <p data-hero-lede className="text-base text-muted">
              A utility-first CSS framework with a Rust compiler. Write a repeated
              variant once and Beam expands it into clean, atomic CSS - nothing ships
              to the browser but the styles.
            </p>

            <div data-telemetry-row>
              <span data-pill>grouped variants</span>
              <span data-pill>Rust compiler</span>
              <span data-pill>zero runtime</span>
            </div>
          </div>

          <div data-hero-utility className="flex direction-column gap-md">
            <div data-install-command className="flex align-center gap-md rounded-md px-lg py-md border border-line">
              <span className="text-sm text-accent font-bold">$</span>
              <code style={{ fontFamily: 'var(--font-mono)' }} className="text-sm text-fg">
                npm install beamcss @beamcss/vite
              </code>
            </div>

            <div data-hero-actions className="flex gap-md align-center">
              <a
                href="https://github.com/garrettsiegel/beamcss#readme"
                data-launch-button
                data-liquid-metal="primary"
                className="px-lg py-sm bg-accent text-on-accent rounded-md text-sm font-medium hover:(bg-accent+12)"
                target="_blank"
                rel="noopener"
              >
                Get started
              </a>
              <a
                href="https://github.com/garrettsiegel/beamcss"
                data-secondary-button
                data-liquid-metal="secondary"
                className="px-lg py-sm border border-line text-fg rounded-md text-sm font-medium hover:(bg-surface)"
                target="_blank"
                rel="noopener"
              >
                View on GitHub
              </a>
            </div>
          </div>

          <div data-syntax-strip>
            <pre className="text-base"><code><span className="text-muted">&lt;button className=&quot;</span><span className="text-fg">rounded-md px-4 py-2 </span><span className="text-accent">hover:(bg-accent text-on-accent)</span><span className="text-muted">&quot;&gt;</span></code></pre>
          </div>
        </div>
      </div>
    </section>
  )
}
