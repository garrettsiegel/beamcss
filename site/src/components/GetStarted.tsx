import { Link } from 'react-router-dom'

export function GetStarted() {
  return (
    <section style={{ borderTop: '2px solid var(--color-accent)' }} className="py-2xl px-lg bg-surface">
      <div className="container flex direction-column gap-xl">

        <div className="flex direction-column gap-sm">
          <h2 className="text-2xl font-bold text-fg">Up in 60 seconds.</h2>
          <p className="text-base text-muted">Install, add the plugin, write classes. Beam compiles on every save.</p>
        </div>

        <pre style={{ fontFamily: 'var(--font-mono)', maxWidth: '38rem' }} className="bg-base rounded-md px-lg py-md border border-line text-base"><code><span className="text-muted">$ </span><span className="text-fg">npm install beamcss @beamcss/vite</span></code></pre>

        <div className="flex gap-md align-center">
          <Link
            to="/docs/installation"
            data-liquid-metal="primary"
            className="px-xl py-md bg-accent text-on-accent rounded-md font-medium hover:(bg-accent+12 scale-[1.02])"
          >
            Read the docs →
          </Link>
          <a
            href="https://github.com/garrettsiegel/beamcss/tree/main/examples"
            data-liquid-metal="secondary"
            className="px-xl py-md border border-line text-fg rounded-md font-medium hover:(bg-panel)"
            target="_blank"
            rel="noopener"
          >
            See examples
          </a>
        </div>

      </div>
    </section>
  )
}
