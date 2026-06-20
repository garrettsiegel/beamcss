import { useEffect, useState } from 'react'
import { Link, Outlet, useLocation } from 'react-router-dom'
import { Nav } from '../components/Nav'
import { Footer } from '../components/Footer'
import { DocsSidebar } from './DocsSidebar'
import { OnThisPage } from './OnThisPage'
import { docsPages } from './docsNav'

export function DocsLayout() {
  const { pathname } = useLocation()
  const [menuOpen, setMenuOpen] = useState(false)

  // Scroll to top on page change and close the mobile drawer.
  useEffect(() => {
    window.scrollTo(0, 0)
    setMenuOpen(false)
  }, [pathname])

  const slug = pathname.split('/').filter(Boolean).pop()
  const index = docsPages.findIndex((p) => p.slug === slug)
  const prev = index > 0 ? docsPages[index - 1] : null
  const next = index >= 0 && index < docsPages.length - 1 ? docsPages[index + 1] : null

  return (
    <div className="bg-base text-fg">
      <Nav />

      <div data-docs-shell>
        <button
          type="button"
          data-docs-menu-toggle
          aria-expanded={menuOpen}
          onClick={() => setMenuOpen((v) => !v)}
        >
          {menuOpen ? 'Close menu' : 'Menu'}
        </button>

        <div data-docs-sidebar-col className={menuOpen ? 'is-open' : undefined}>
          <DocsSidebar onNavigate={() => setMenuOpen(false)} />
        </div>

        <main data-docs-main>
          <Outlet />

          <nav data-docs-pager aria-label="Pagination">
            {prev ? (
              <Link to={`/docs/${prev.slug}`} data-docs-pager-link data-dir="prev">
                <span data-docs-pager-dir>← Previous</span>
                <span data-docs-pager-title>{prev.title}</span>
              </Link>
            ) : (
              <span />
            )}
            {next ? (
              <Link to={`/docs/${next.slug}`} data-docs-pager-link data-dir="next">
                <span data-docs-pager-dir>Next →</span>
                <span data-docs-pager-title>{next.title}</span>
              </Link>
            ) : (
              <span />
            )}
          </nav>
        </main>

        <OnThisPage />
      </div>

      <Footer />
    </div>
  )
}
