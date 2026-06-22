import type { ReactNode } from 'react'

export function Callout({ icon = '⇄', children }: { icon?: string; children: ReactNode }) {
  return (
    <div data-callout>
      <span data-callout-icon aria-hidden="true">{icon}</span>
      <div data-callout-body>{children}</div>
    </div>
  )
}
