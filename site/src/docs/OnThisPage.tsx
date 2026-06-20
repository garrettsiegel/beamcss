import { useEffect, useState } from 'react'
import { useLocation } from 'react-router-dom'

interface Heading {
  id: string
  text: string
}

export function OnThisPage() {
  const { pathname } = useLocation()
  const [headings, setHeadings] = useState<Heading[]>([])
  const [activeId, setActiveId] = useState<string>('')

  // Re-scan headings whenever the route changes (after the page renders).
  useEffect(() => {
    const nodes = Array.from(
      document.querySelectorAll<HTMLHeadingElement>('[data-docs-prose] h2[id]'),
    )
    setHeadings(nodes.map((n) => ({ id: n.id, text: n.textContent ?? '' })))
    setActiveId(nodes[0]?.id ?? '')
  }, [pathname])

  // Scroll-spy: highlight the heading nearest the top of the viewport.
  useEffect(() => {
    if (headings.length === 0) return
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((e) => e.isIntersecting)
          .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top)
        if (visible[0]) setActiveId(visible[0].target.id)
      },
      { rootMargin: '0px 0px -70% 0px', threshold: 0 },
    )
    for (const { id } of headings) {
      const el = document.getElementById(id)
      if (el) observer.observe(el)
    }
    return () => observer.disconnect()
  }, [headings])

  if (headings.length === 0) return null

  return (
    <aside data-docs-toc aria-label="On this page">
      <p data-docs-toc-heading>On this page</p>
      <ul>
        {headings.map((h) => (
          <li key={h.id}>
            <a
              href={`#${h.id}`}
              data-docs-toc-link
              className={activeId === h.id ? 'is-active' : undefined}
            >
              {h.text}
            </a>
          </li>
        ))}
      </ul>
    </aside>
  )
}
