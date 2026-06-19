import assert from "node:assert/strict"
import { spawnSync } from "node:child_process"
import { mkdtempSync, readFileSync, rmSync } from "node:fs"
import { tmpdir } from "node:os"
import { join, resolve } from "node:path"

import { tailwindToBeamClassName } from "../dist/codemod.js"
import { suggestBeamClasses } from "../dist/language.js"

const grouped = tailwindToBeamClassName(
  "p-4 rounded-lg text-zinc-100 bg-zinc-900 hover:bg-blue-500 hover:text-white md:hover:scale-105",
)

assert.equal(
  grouped.className,
  "p-4 round-lg fg-zinc-100 bg-zinc-900 hover:(bg-blue-500 fg-white) md:hover:scale-105",
)
assert.equal(grouped.warnings.length, 0)

const arbitrary = tailwindToBeamClassName("[&>svg]:w-[1rem] [&>svg]:h-[1rem] font-sans")

assert.equal(arbitrary.className, "[&>svg]:(w-[1rem] h-[1rem]) font-ui")

const unknown = tailwindToBeamClassName("container prose")

assert.equal(unknown.className, "container prose")
assert.equal(unknown.warnings.length, 2)

const cli = spawnSync(
  process.execPath,
  ["dist/cli.js", "codemod", "hover:bg-blue-500 hover:text-white"],
  {
    cwd: new URL("..", import.meta.url),
    encoding: "utf8",
  },
)

assert.equal(cli.status, 0)
assert.equal(cli.stdout.trim(), "hover:(bg-blue-500 fg-white)")

const repoRoot = resolve(new URL("../../..", import.meta.url).pathname)
const tempDir = mkdtempSync(join(tmpdir(), "beam-cli-test-"))

try {
  const output = join(tempDir, "beam.css")
  const build = spawnSync(
    process.execPath,
    [
      "packages/beamcss/dist/cli.js",
      "build",
      "--config",
      "examples/dashboard/beam.config.ts",
      "--content",
      "examples/dashboard",
      "--out",
      output,
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  )

  assert.equal(build.status, 0, build.stderr)
  assert(readFileSync(output, "utf8").includes("@layer beam.reset, beam.tokens"))

  const check = spawnSync(
    process.execPath,
    [
      "packages/beamcss/dist/cli.js",
      "check",
      "--config",
      "examples/dashboard/beam.config.ts",
      "--content",
      "examples/dashboard",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  )

  assert.equal(check.status, 0, check.stderr)
  assert(check.stdout.includes("Beam check passed"))

  const explain = spawnSync(
    process.execPath,
    [
      "packages/beamcss/dist/cli.js",
      "explain",
      "hover:(bg-accent fg-on-accent)",
      "--config",
      "examples/dashboard/beam.config.ts",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  )

  assert.equal(explain.status, 0, explain.stderr)
  assert(explain.stdout.includes(":hover { background:var(--color-accent); }"))
} finally {
  rmSync(tempDir, { force: true, recursive: true })
}

const completions = suggestBeamClasses({
  tokens: {
    space: { card: "1rem" },
    color: { accent: "#3b82f6" },
    radius: { md: "8px" },
    text: { lg: "20px" },
    font: { ui: "Inter, sans-serif" },
    screens: { tablet: "48rem" },
  },
}).map((completion) => completion.label)

assert(completions.includes("p-4"))
assert(completions.includes("bg-accent"))
assert(completions.includes("round-md"))
assert(completions.includes("stack()"))
assert(completions.includes("tablet:()"))
