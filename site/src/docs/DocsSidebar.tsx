import { NavLink } from 'react-router-dom'
import { docsGroups } from './docsNav'

interface DocsSidebarProps {
  /** Called when a link is chosen - used to close the mobile drawer. */
  onNavigate?: () => void
}

export function DocsSidebar({ onNavigate }: DocsSidebarProps) {
  return (
    <nav data-docs-sidebar aria-label="Documentation">
      {docsGroups.map(({ group, pages }) => (
        <div key={group} data-docs-sidebar-group>
          <p data-docs-sidebar-heading>{group}</p>
          <ul>
            {pages.map((page) => (
              <li key={page.slug}>
                <NavLink
                  to={`/docs/${page.slug}`}
                  data-docs-sidebar-link
                  onClick={onNavigate}
                  className={({ isActive }) => (isActive ? 'is-active' : undefined)}
                >
                  {page.title}
                </NavLink>
              </li>
            ))}
          </ul>
        </div>
      ))}
    </nav>
  )
}
