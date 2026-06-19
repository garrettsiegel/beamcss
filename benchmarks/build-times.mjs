import { spawnSync } from "node:child_process"
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { basename, dirname, join, resolve } from "node:path"
import { performance } from "node:perf_hooks"
import { fileURLToPath } from "node:url"

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)))
const { buildCss } = await import("../packages/beamcss/dist/cli-runner.js")
const repeats = numberFromEnv("BEAM_BENCH_REPEATS", 80)
const iterations = numberFromEnv("BEAM_BENCH_ITERATIONS", 7)
const keepFixture = process.env.BEAM_BENCH_KEEP === "1"
const fixtureDir = mkdtempSync(join(tmpdir(), "beamcss-bench-"))

try {
  writeFixtures(fixtureDir, repeats)

  run("pnpm", ["--filter", "beamcss", "build"], repoRoot)

  const beam = measure("Beam", iterations, () => {
    return Buffer.from(
      buildCss({
        config: join(fixtureDir, "beam.config.ts"),
        content: [join(fixtureDir, "beam.html")],
      }),
    )
  })

  const tailwind = measure("Tailwind", iterations, () => {
    run(
      tailwindBin(),
      [
        "-i",
        join(fixtureDir, "tailwind-input.css"),
        "-o",
        join(fixtureDir, "tailwind.css"),
      ],
      repoRoot,
    )
    return readFileSync(join(fixtureDir, "tailwind.css"))
  })

  printResult(beam, tailwind)

  if (keepFixture) {
    console.log(`\nFixture kept at ${fixtureDir}`)
  }
} finally {
  if (!keepFixture) {
    rmSync(fixtureDir, { force: true, recursive: true })
  }
}

function writeFixtures(dir, count) {
  mkdirSync(dir, { recursive: true })
  writeFileSync(
    join(dir, "beam.config.ts"),
    `import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    space: { card: "1rem", section: "2rem" },
    color: {
      base: "#0b0b0c",
      surface: "#16161a",
      panel: "#202027",
      fg: "#f4f4f5",
      muted: "#9ca3af",
      line: "#34343d",
      accent: "#3b82f6",
      success: "#22c55e",
      warning: "#f59e0b",
      "on-accent": "#ffffff",
    },
    radius: { sm: "4px", md: "8px", lg: "16px", full: "9999px" },
    text: { sm: "14px", base: "16px", lg: "20px", xl: "28px" },
    font: { ui: "Inter, system-ui, sans-serif", mono: "ui-monospace, monospace" },
    screens: {
      tablet: "48rem",
      desktop: "64rem",
      wide: "80rem",
      "mobile-landscape": "(max-width:47.999rem) and (orientation:landscape)",
    },
  },
})
`,
  )

  writeFileSync(join(dir, "beam.html"), repeatMarkup(beamCard, count))
  writeFileSync(join(dir, "tailwind.html"), repeatMarkup(tailwindCard, count))
  writeFileSync(
    join(dir, "tailwind-input.css"),
    `@import "tailwindcss";
@source "./tailwind.html";
`,
  )
}

function repeatMarkup(template, count) {
  return `<main>\n${Array.from({ length: count }, (_, index) => template(index)).join("\n")}\n</main>\n`
}

function beamCard(index) {
  return `<article class="stack(gap-4) p-4 bg-surface round-lg border bd-line hover:(bg-[color-mix(in_srgb,var(--color-surface),white_8%)] scale-105) tablet:(row(center between) p-6)">
  <header class="row(between center) gap-2">
    <h2 class="text-lg fg-fg">Card ${index}</h2>
    <span class="px-2 py-1 round-full bg-accent fg-on-accent text-sm">live</span>
  </header>
  <p class="text-base fg-muted max-w-[42rem]">Grouped variants and layout primitives keep repeated intent compact.</p>
  <div class="grid(cols-1 tablet:cols-3 gap-3)">
    <span class="p-3 bg-panel round-md fg-success">parse</span>
    <span class="p-3 bg-panel round-md fg-warning">emit</span>
    <span class="p-3 bg-panel round-md fg-accent">ship</span>
  </div>
</article>`
}

function tailwindCard(index) {
  return `<article class="flex flex-col gap-4 p-4 bg-zinc-900 rounded-lg border border-zinc-700 hover:bg-zinc-800 hover:scale-105 md:flex-row md:items-center md:justify-between md:p-6">
  <header class="flex flex-row justify-between items-center gap-2">
    <h2 class="text-lg text-zinc-100">Card ${index}</h2>
    <span class="px-2 py-1 rounded-full bg-blue-500 text-white text-sm">live</span>
  </header>
  <p class="text-base text-zinc-400 max-w-[42rem]">Grouped variants and layout primitives keep repeated intent compact.</p>
  <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
    <span class="p-3 bg-zinc-800 rounded-md text-green-500">parse</span>
    <span class="p-3 bg-zinc-800 rounded-md text-amber-500">emit</span>
    <span class="p-3 bg-zinc-800 rounded-md text-blue-500">ship</span>
  </div>
</article>`
}

function measure(name, count, build) {
  const samples = []
  let output = Buffer.alloc(0)

  for (let index = 0; index < count; index += 1) {
    const start = performance.now()
    output = build()
    samples.push(performance.now() - start)
  }

  return {
    name,
    samples,
    medianMs: median(samples),
    minMs: Math.min(...samples),
    maxMs: Math.max(...samples),
    bytes: output.byteLength,
  }
}

function run(command, args, cwd) {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  })

  if (result.error) {
    throw result.error
  }
  if (result.status !== 0) {
    throw new Error(
      `${basename(command)} ${args.join(" ")} failed\n${result.stderr || result.stdout}`,
    )
  }
}

function tailwindBin() {
  const name = process.platform === "win32" ? "tailwindcss.cmd" : "tailwindcss"
  return resolve(repoRoot, "node_modules", ".bin", name)
}

function median(values) {
  const sorted = [...values].sort((left, right) => left - right)
  const middle = Math.floor(sorted.length / 2)
  return sorted.length % 2 === 0
    ? (sorted[middle - 1] + sorted[middle]) / 2
    : sorted[middle]
}

function printResult(...results) {
  console.log(`Fixture cards: ${repeats}`)
  console.log(`Iterations: ${iterations}`)
  console.log("")
  console.log("| Tool | Median | Min | Max | Output |")
  console.log("| --- | ---: | ---: | ---: | ---: |")

  for (const result of results) {
    console.log(
      `| ${result.name} | ${formatMs(result.medianMs)} | ${formatMs(result.minMs)} | ${formatMs(
        result.maxMs,
      )} | ${formatBytes(result.bytes)} |`,
    )
  }
}

function formatMs(value) {
  return `${value.toFixed(1)}ms`
}

function formatBytes(bytes) {
  if (bytes < 1024) return `${bytes} B`
  return `${(bytes / 1024).toFixed(1)} KiB`
}

function numberFromEnv(name, fallback) {
  const value = Number(process.env[name])
  return Number.isFinite(value) && value > 0 ? Math.floor(value) : fallback
}
