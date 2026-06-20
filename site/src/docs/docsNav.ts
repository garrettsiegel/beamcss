import type { ComponentType } from 'react'
import { Introduction } from './pages/Introduction'
import { QuickStart } from './pages/QuickStart'
import { Installation } from './pages/Installation'
import { Configuration } from './pages/Configuration'
import { Syntax } from './pages/Syntax'
import { Utilities } from './pages/Utilities'
import { Tooling } from './pages/Tooling'
import { FromTailwind } from './pages/FromTailwind'

export interface DocPage {
  slug: string
  title: string
  group: string
  Component: ComponentType
}

/** Ordered single source of truth for routes, sidebar, and prev/next. */
export const docsPages: DocPage[] = [
  { slug: 'introduction', title: 'Introduction', group: 'Getting started', Component: Introduction },
  { slug: 'quick-start', title: 'Quick start', group: 'Getting started', Component: QuickStart },
  { slug: 'installation', title: 'Installation', group: 'Getting started', Component: Installation },
  { slug: 'configuration', title: 'Configuration', group: 'Getting started', Component: Configuration },
  { slug: 'syntax', title: 'Writing styles', group: 'Core concepts', Component: Syntax },
  { slug: 'utilities', title: 'Utilities reference', group: 'Core concepts', Component: Utilities },
  { slug: 'tooling', title: 'CLI & integrations', group: 'Tooling', Component: Tooling },
  { slug: 'from-tailwind', title: 'Coming from Tailwind', group: 'Guides', Component: FromTailwind },
]

/** Default docs landing slug. */
export const DEFAULT_DOC = 'introduction'

/** Groups in display order, each with its pages. */
export const docsGroups: { group: string; pages: DocPage[] }[] = (() => {
  const order: string[] = []
  const byGroup = new Map<string, DocPage[]>()
  for (const page of docsPages) {
    if (!byGroup.has(page.group)) {
      byGroup.set(page.group, [])
      order.push(page.group)
    }
    byGroup.get(page.group)!.push(page)
  }
  return order.map((group) => ({ group, pages: byGroup.get(group)! }))
})()
