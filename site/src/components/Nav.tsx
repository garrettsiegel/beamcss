import { Link } from 'react-router-dom'

export function Nav() {
  return (
    <nav data-site-nav className="flex justify-between align-center px-lg py-md border-b border-line">
      <Link
        to="/"
        data-beam-brand
        className="text-fg"
        aria-label="Beam CSS home"
      >
        <span style={{ fontFamily: 'Mionta, var(--font-ui)', fontSize: '1.15rem', letterSpacing: '0.01em' }}>BeamCSS</span>
      </Link>
      <div data-site-nav-links className="flex gap-lg align-center">
        <Link
          to="/docs"
          data-nav-link
          className="text-sm text-muted hover:(text-fg)"
        >
          Docs
        </Link>
        <a
          href="https://www.npmjs.com/package/beamcss"
          data-nav-link
          className="text-sm text-muted hover:(text-fg)"
          target="_blank"
          rel="noopener"
        >
          npm
        </a>
        <a
          href="https://github.com/garrettsiegel/beamcss"
          data-nav-link
          className="text-sm text-muted hover:(text-fg)"
          target="_blank"
          rel="noopener"
        >
          GitHub
        </a>
      </div>
    </nav>
  )
}
