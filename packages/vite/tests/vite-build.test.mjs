import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises"
import { dirname, join } from "node:path"
import { fileURLToPath } from "node:url"

import { build } from "vite"

import { beamcss } from "../dist/index.js"

const packageDir = dirname(dirname(fileURLToPath(import.meta.url)))
const tempRoot = join(packageDir, ".tmp")

await mkdir(tempRoot, { recursive: true })
const tempDir = await mkdtemp(join(tempRoot, "vite-"))

try {
  await writeFile(
    join(tempDir, "beam.config.ts"),
    `import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    space: { card: "1rem" },
    color: {
      accent: "#3b82f6",
      base: "#0b0b0c",
      fg: "#e8e8ea",
      surface: "#16161a",
    },
    radius: { md: "8px" },
    text: { lg: "20px" },
    font: { ui: "Inter, system-ui, sans-serif" },
    screens: { tablet: "48rem" },
  },
})
`,
  )
  await writeFile(
    join(tempDir, "index.html"),
    `<main class="stack(center gap-4) p-4 bg-surface hover:(bg-accent fg-fg) tablet:text-lg">Beam</main>`,
  )

  await build({
    root: tempDir,
    logLevel: "silent",
    plugins: [
      beamcss({
        config: join(tempDir, "beam.config.ts"),
        content: [join(tempDir, "index.html")],
      }),
    ],
  })

  const html = await readFile(join(tempDir, "dist", "index.html"), "utf8")

  assertIncludes(html, '<style data-beamcss="">')
  assertIncludes(html, "@layer beam.reset, beam.tokens, beam.base, beam.utilities;")
  assertIncludes(html, ".stack\\(center.gap-4\\){display:flex;}")
  assertIncludes(html, ".p-4{padding:4px;}")
  assertIncludes(html, ".hover\\:\\(bg-accent.fg-fg\\):hover{background:var(--color-accent);}")
  assertIncludes(html, "@media (min-width:48rem){.tablet\\:text-lg{font-size:var(--text-lg);}}")
} finally {
  await rm(tempDir, { force: true, recursive: true })
}

function assertIncludes(value, expected) {
  if (!value.includes(expected)) {
    throw new Error(`Expected built CSS to include:\n${expected}\n\nActual CSS:\n${value}`)
  }
}
