import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js"
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js"
import { pathToFileURL } from "node:url"
import { z } from "zod"

export interface BeamMcpOptions {
  name?: string
  version?: string
}

const utilityReference = `Beam utilities:

- flex direction-column gap-4: flex column with a gap
- flex direction-row align-center gap-2: flex row with centered items
- flex wrap align-center gap-2: wrapping flex row
- grid cols-1 tablet:cols-3 gap-4: responsive grid columns
- grid place-center: centered grid content

Utility groups:
- padding:(16 top:24)
- text:(16 bold center)
- border:(1 solid accent)`

const variantReference = `Beam variants:

- State: hover, focus, focus-visible, focus-within, active, disabled
- Structural: first, last, odd, even
- Theme: dark
- Group/peer: group-hover, group-focus, peer-checked, peer-focus
- Media: configured screens like tablet, desktop, wide, mobile-landscape; motion-safe; print
- Arbitrary selector: [&>svg]

Use grouped variants to factor repeated prefixes:

hover:(bg-accent text-on-accent scale-105)
tablet:(p-6 rounded-lg hover:(bg-surface scale-[1.02]))`

export function createBeamMcpServer(options: BeamMcpOptions = {}) {
  const server = new McpServer({
    name: options.name ?? "beamcss",
    version: options.version ?? "0.0.0",
  })

  server.registerTool(
    "beam_syntax_reference",
    {
      description: "Return Beam CSS syntax guidance for utilities, variants, and values.",
      inputSchema: z.object({
        topic: z
          .enum(["all", "variants", "utilities", "values", "install"])
          .default("all")
          .describe("Syntax topic to return."),
      }),
    },
    async ({ topic }) => ({
      content: [
        {
          type: "text",
          text: syntaxReference(topic),
        },
      ],
    }),
  )

  server.registerTool(
    "beam_scaffold_component",
    {
      description: "Create a small HTML/JSX component snippet using Beam classes.",
      inputSchema: z.object({
        kind: z
          .enum(["button", "card", "dashboard-panel", "form-row"])
          .describe("Component shape to scaffold."),
        jsx: z.boolean().default(false).describe("Use className instead of class."),
      }),
    },
    async ({ kind, jsx }) => ({
      content: [
        {
          type: "text",
          text: scaffoldComponent(kind, jsx),
        },
      ],
    }),
  )

  server.registerTool(
    "beam_token_summary",
    {
      description: "Summarize token names from a Beam config JSON object.",
      inputSchema: z.object({
        config: z
          .string()
          .describe("A JSON string containing a Beam config object with a tokens field."),
      }),
    },
    async ({ config }) => ({
      content: [
        {
          type: "text",
          text: summarizeTokens(config),
        },
      ],
    }),
  )

  return server
}

export async function startBeamMcpServer(options: BeamMcpOptions = {}) {
  const server = createBeamMcpServer(options)
  const transport = new StdioServerTransport()
  await server.connect(transport)
}

function syntaxReference(topic: string): string {
  const install = `Install surfaces:

- beamcss: core package and CLI
- @beamcss/vite: Vite plugin
- @beamcss/postcss: PostCSS plugin

Vite:

import { beamcss } from "@beamcss/vite"

beamcss({ config: "beam.config.ts", content: ["index.html", "src"] })`

  const values = `Beam value syntax:

- Literal spacing: p-4, gap-2
- Token name: gap-section, bg-surface, text-muted, rounded-md, text-lg
- Arbitrary static value: max-w-[42rem], bg-[oklch(72%_0.14_240)], bg-[color-mix(in_srgb,var(--color-surface),white_8%)]
- Dynamic CSS variable: w-(--progress), h-(--panel-height)`

  if (topic === "install") return install
  if (topic === "variants") return variantReference
  if (topic === "utilities") return utilityReference
  if (topic === "values") return values

  return [install, values, variantReference, utilityReference].join("\n\n---\n\n")
}

function scaffoldComponent(kind: string, jsx: boolean): string {
  const attr = jsx ? "className" : "class"
  const snippets: Record<string, string> = {
    button: `<button ${attr}="flex direction-row align-center gap-2 px-4 py-2 rounded-md bg-accent text-on-accent hover:(scale-105)">
  Deploy
</button>`,
    card: `<article ${attr}="flex direction-column gap-4 p-4 bg-surface rounded-lg border border-line">
  <h2 ${attr}="text-lg text-fg">Card title</h2>
  <p ${attr}="text-base text-muted">A compact card using Beam utilities.</p>
</article>`,
    "dashboard-panel": `<section ${attr}="grid cols-1 tablet:cols-3 gap-4">
  <article ${attr}="flex direction-column gap-2 p-4 bg-surface rounded-lg border border-line">
    <p ${attr}="text-sm text-muted">Build time</p>
    <strong ${attr}="text-xl text-success">38ms</strong>
  </article>
</section>`,
    "form-row": `<label ${attr}="flex direction-column gap-2">
  <span ${attr}="text-sm text-muted">Email</span>
  <input ${attr}="px-3 py-2 rounded-md border border-line bg-surface text-fg focus:(border-accent)" />
</label>`,
  }

  return snippets[kind]
}

function summarizeTokens(configJson: string): string {
  try {
    const config = JSON.parse(configJson) as {
      tokens?: Record<string, unknown>
    }
    const tokens = config.tokens ?? {}
    const lines = Object.entries(tokens).map(([family, value]) => {
      if (Array.isArray(value)) {
        return `- ${family}: ${value.length} steps`
      }
      if (value && typeof value === "object") {
        return `- ${family}: ${Object.keys(value).join(", ")}`
      }
      return `- ${family}: present`
    })

    return lines.length > 0 ? lines.join("\n") : "No tokens found."
  } catch (error) {
    return `Invalid JSON config: ${error instanceof Error ? error.message : String(error)}`
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  startBeamMcpServer().catch((error) => {
    console.error(error)
    process.exit(1)
  })
}
