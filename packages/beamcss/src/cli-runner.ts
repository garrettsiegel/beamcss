import { spawnSync } from "node:child_process"
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from "node:fs"
import { readFile } from "node:fs/promises"
import { tmpdir } from "node:os"
import { basename, dirname, extname, join, resolve } from "node:path"
import { fileURLToPath } from "node:url"
import JSON5 from "json5"

import type { BeamConfig } from "./index.js"
import { compile, loadNativeBinding } from "./native.js"
import { extractClassStrings, scanFiles } from "./scanner.js"

export interface BuildCssOptions {
  config?: string
  content?: string[]
}

export async function buildCssNative(options: BuildCssOptions = {}): Promise<string> {
  if (!loadNativeBinding()) {
    throw new Error("Beam native binding is not available in this package build")
  }

  const config = await loadConfig(options.config ?? "beam.config.ts")
  const classStrings = await scanFiles(options.content ?? ["."])
  const result = compile(config, classStrings)

  if (result.errors.length > 0) {
    throw new Error(
      result.errors.map((error) => `${error.class_name}: ${error.message}`).join("\n"),
    )
  }

  return result.css
}

export function buildCss(options: BuildCssOptions = {}): string {
  if (loadNativeBinding()) {
    return buildCssNativeSync(options)
  }

  return buildCssWithRustCli(options)
}

async function loadConfig(path: string): Promise<BeamConfig> {
  return parseConfigSource(await readFile(path, "utf8"), path)
}

export function loadConfigSync(path: string): BeamConfig {
  return parseConfigSource(readFileSync(path, "utf8"), path)
}

export function parseConfigSource(source: string, path = "beam.config.ts"): BeamConfig {
  const objectSource = path.endsWith(".json") ? source : extractConfigObject(source)
  return JSON5.parse(objectSource) as BeamConfig
}

function buildCssNativeSync(options: BuildCssOptions): string {
  const configPath = options.config ?? "beam.config.ts"
  const config = loadConfigSync(configPath)
  const classStrings = scanClassStringsSync(options.content ?? ["."])
  const result = compile(config, classStrings)

  if (result.errors.length > 0) {
    throw new Error(
      result.errors.map((error) => `${error.class_name}: ${error.message}`).join("\n"),
    )
  }

  return result.css
}

function buildCssWithRustCli(options: BuildCssOptions): string {
  const repoRoot = findRepoRoot(dirname(fileURLToPath(import.meta.url)))

  if (!repoRoot) {
    throw new Error("Beam native package loading is not bundled yet; run from the Beam CSS repo")
  }

  const tempDir = mkdtempSync(join(tmpdir(), "beamcss-"))
  const output = join(tempDir, "beam.css")

  try {
    const args = [
      "run",
      "-p",
      "beam_cli",
      "--bin",
      "beam",
      "--",
      "build",
      "--config",
      options.config ?? "beam.config.ts",
      "--out",
      output,
    ]

    for (const path of options.content ?? ["."]) {
      args.push("--content", path)
    }

    const result = spawnSync("cargo", args, {
      cwd: repoRoot,
      encoding: "utf8",
    })

    if (result.error) {
      throw result.error
    }
    if (result.status !== 0) {
      throw new Error(result.stderr.trim() || result.stdout.trim() || "Beam build failed")
    }

    return readFileSync(output, "utf8")
  } finally {
    rmSync(tempDir, { force: true, recursive: true })
  }
}

function extractConfigObject(source: string): string {
  const defineConfigIndex = source.indexOf("defineConfig")
  const exportDefaultIndex = source.indexOf("export default")
  const start =
    defineConfigIndex >= 0
      ? source.indexOf("(", defineConfigIndex) + 1
      : exportDefaultIndex >= 0
        ? exportDefaultIndex + "export default".length
        : -1

  if (start <= 0) {
    throw new Error("expected `export default defineConfig({...})`")
  }

  let depth = 0
  let objectStart = -1
  let quote: string | undefined
  let escaped = false

  for (let index = start; index < source.length; index += 1) {
    const char = source[index]
    if (quote) {
      if (escaped) {
        escaped = false
      } else if (char === "\\") {
        escaped = true
      } else if (char === quote) {
        quote = undefined
      }
      continue
    }

    if (char === '"' || char === "'" || char === "`") {
      quote = char
    } else if (char === "{") {
      if (objectStart === -1) objectStart = index
      depth += 1
    } else if (char === "}") {
      depth -= 1
      if (depth === 0 && objectStart !== -1) {
        return source.slice(objectStart, index + 1)
      }
    }
  }

  throw new Error("missing config object")
}

export function scanClassStringsSync(paths: string[]): string[] {
  const supportedExtensions = new Set([".html", ".jsx", ".tsx", ".vue", ".svelte", ".astro"])
  const ignoredDirs = new Set([
    ".git",
    "node_modules",
    "dist",
    "target",
    ".vite",
    ".next",
    "coverage",
  ])
  const files = new Set<string>()

  function collect(path: string) {
    if (!existsSync(path)) return
    const details = statSync(path)
    if (details.isFile()) {
      if (supportedExtensions.has(extname(path))) files.add(path)
      return
    }
    if (!details.isDirectory() || ignoredDirs.has(basename(path))) return
    for (const entry of readdirSync(path)) {
      collect(join(path, entry))
    }
  }

  for (const path of paths.length > 0 ? paths : ["."]) {
    collect(path)
  }

  return [...files]
    .sort()
    .flatMap((file) => extractClassStrings(readFileSync(file, "utf8")))
}

export function findRepoRoot(start: string): string | undefined {
  let current = resolve(start)
  for (;;) {
    try {
      if (
        readFileSync(resolve(current, "Cargo.toml"), "utf8").includes("crates/beam_cli") &&
        readFileSync(resolve(current, "crates/beam_cli/Cargo.toml"), "utf8")
      ) {
        return current
      }
    } catch {
      // Keep walking upward.
    }

    const parent = dirname(current)
    if (parent === current) {
      return undefined
    }
    current = parent
  }
}
