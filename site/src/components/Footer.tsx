import { Link } from 'react-router-dom'
import { docsPages } from '../docs/docsNav'

const REPO = 'https://github.com/garrettsiegel/beamcss'

export function Footer() {
  const year = new Date().getFullYear()

  return (
    <footer data-site-footer>
      <div className="container">
        <div data-footer-top>
          <div data-footer-brand>
            <Link to="/" data-footer-wordmark aria-label="Beam CSS home">
              BeamCSS
            </Link>
            <p className="text-sm text-muted">
              Focused styles, zero scatter. A Rust-fast, utility-first CSS framework with
              grouped syntax and atomic output.
            </p>
          </div>

          <nav data-footer-cols aria-label="Footer">
            <div data-footer-col>
              <p data-footer-heading>Docs</p>
              <ul>
                {docsPages.map((page) => (
                  <li key={page.slug}>
                    <Link to={`/docs/${page.slug}`}>{page.title}</Link>
                  </li>
                ))}
              </ul>
            </div>

            <div data-footer-col>
              <p data-footer-heading>Resources</p>
              <ul>
                <li>
                  <a href="https://www.npmjs.com/package/beamcss" target="_blank" rel="noopener">
                    npm package
                  </a>
                </li>
                <li>
                  <a href={REPO} target="_blank" rel="noopener">
                    GitHub
                  </a>
                </li>
                <li>
                  <a href={`${REPO}/tree/main/examples`} target="_blank" rel="noopener">
                    Examples
                  </a>
                </li>
                <li>
                  <a href={`${REPO}/blob/main/docs/beam-css-spec.md`} target="_blank" rel="noopener">
                    Spec
                  </a>
                </li>
              </ul>
            </div>

            <div data-footer-col>
              <p data-footer-heading>Project</p>
              <ul>
                <li>
                  <a href={`${REPO}#readme`} target="_blank" rel="noopener">
                    Readme
                  </a>
                </li>
                <li>
                  <a href={`${REPO}/blob/main/docs/ROADMAP.md`} target="_blank" rel="noopener">
                    Roadmap
                  </a>
                </li>
                <li>
                  <a href={`${REPO}/issues`} target="_blank" rel="noopener">
                    Issues
                  </a>
                </li>
                <li>
                  <a href={`${REPO}/blob/main/LICENSE`} target="_blank" rel="noopener">
                    License
                  </a>
                </li>
              </ul>
            </div>
          </nav>
        </div>

        <div data-footer-bottom>
          <span className="text-sm text-muted">© {year} Beam CSS · MIT License</span>
          <span className="text-sm text-muted" style={{ fontFamily: 'var(--font-mono)' }}>
            v0.1.1
          </span>
        </div>
      </div>
    </footer>
  )
}
