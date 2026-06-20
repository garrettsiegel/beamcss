import { useState } from 'react'

interface CodeBlockProps {
  children: string
  /** Optional filename / shell label shown in the header strip */
  title?: string
  /** Hint shown before the code when there's no title (e.g. "terminal") */
  lang?: string
}

export function CodeBlock({ children, title }: CodeBlockProps) {
  const [copied, setCopied] = useState(false)
  const code = children.replace(/\n$/, '')

  async function copy() {
    try {
      await navigator.clipboard.writeText(code)
      setCopied(true)
      setTimeout(() => setCopied(false), 1500)
    } catch {
      /* clipboard unavailable — ignore */
    }
  }

  return (
    <div data-code-block>
      {title && <div data-code-title>{title}</div>}
      <div data-code-body>
        <button
          type="button"
          data-code-copy
          onClick={copy}
          aria-label="Copy code"
        >
          {copied ? 'Copied' : 'Copy'}
        </button>
        <pre>
          <code>{code}</code>
        </pre>
      </div>
    </div>
  )
}
