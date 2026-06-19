#!/usr/bin/env node

import { spawnSync } from "node:child_process"
import { mkdirSync, writeFileSync } from "node:fs"
import { dirname } from "node:path"
import { fileURLToPath } from "node:url"

import { tailwindToBeamClassName } from "./codemod.js"
import { buildCss, findRepoRoot, loadConfigSync, scanClassStringsSync } from "./cli-runner.js"
import { compile, explain, loadNativeBinding } from "./native.js"

const args = process.argv.slice(2)

if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
  console.log("beamcss")
  console.log("")
  console.log("Commands:")
  console.log("  beam init [--template basic|vite]")
  console.log("  beam build [--config beam.config.ts] [--content .] [--out beam.css]")
  console.log("  beam check [--config beam.config.ts] [--content .] [--format text|json]")
  console.log('  beam codemod "<tailwind class string>" [--format text|json]')
  console.log('  beam explain "<class string>" [--config beam.config.ts] [--format text|json]')
  console.log("  beam dev   [--config beam.config.ts] [--content .] [--out beam.css]")
  process.exit(0)
}

if (args[0] === "codemod") {
  const className = args[1]
  const format = optionValue(args, "--format") ?? "text"

  if (!className) {
    console.error('beam: codemod expects a class string, for example `beam codemod "hover:bg-blue-500"`')
    process.exit(1)
  }

  const result = tailwindToBeamClassName(className)
  if (format === "json") {
    console.log(JSON.stringify(result, null, 2))
  } else {
    console.log(result.className)
    for (const warning of result.warnings) {
      console.error(`${warning.className}: ${warning.message}`)
    }
  }
  process.exit(result.warnings.length > 0 ? 2 : 0)
}

if (args[0] === "build") {
  try {
    const output = optionValue(args, "--out") ?? optionValue(args, "-o") ?? "beam.css"
    mkdirSync(dirname(output), { recursive: true })
    writeFileSync(output, buildCss(buildOptions(args)))
    console.log(`wrote ${output}`)
    process.exit(0)
  } catch (error) {
    console.error(`beam: ${error instanceof Error ? error.message : String(error)}`)
    process.exit(1)
  }
}

if (loadNativeBinding() && args[0] === "check") {
  try {
    const options = buildOptions(args)
    const result = compile(loadConfigSync(options.config ?? "beam.config.ts"), scanClassStringsSync(options.content ?? ["."]))
    const format = optionValue(args, "--format") ?? "text"
    if (format === "json") {
      console.log(JSON.stringify(result, null, 2))
    } else if (result.errors.length === 0) {
      console.log("Beam check passed")
    } else {
      for (const error of result.errors) {
        console.error(`${error.class_name}: ${error.message}`)
      }
    }
    process.exit(result.errors.length === 0 ? 0 : 1)
  } catch (error) {
    console.error(`beam: ${error instanceof Error ? error.message : String(error)}`)
    process.exit(1)
  }
}

if (loadNativeBinding() && args[0] === "explain") {
  try {
    const className = args[1]
    if (!className) {
      console.error(
        'beam: explain expects a class string, for example `beam explain "flex direction-column gap-4"`',
      )
      process.exit(1)
    }
    const config = loadConfigSync(optionValue(args, "--config") ?? "beam.config.ts")
    const result = explain(config, [className])
    const format = optionValue(args, "--format") ?? "text"
    if (format === "json") {
      console.log(JSON.stringify(result, null, 2))
    } else {
      for (const token of result.class_strings.flatMap((classString) => classString.tokens)) {
        console.log(`${token.raw} (${token.kind})`)
        for (const atom of token.atoms) {
          console.log(`  ${atom.selector} { ${atom.declaration}; }`)
        }
        for (const error of token.errors) {
          console.error(`  ${error.class_name}: ${error.message}`)
        }
      }
    }
    process.exit(result.errors.length === 0 ? 0 : 1)
  } catch (error) {
    console.error(`beam: ${error instanceof Error ? error.message : String(error)}`)
    process.exit(1)
  }
}

const repoRoot = findRepoRoot(dirname(fileURLToPath(import.meta.url)))

if (!repoRoot) {
  console.error("beam: packaged native CLI is not bundled yet")
  console.error("beam: run from the Beam CSS repo or use the Rust CLI directly for now")
  process.exit(1)
}

const result = spawnSync("cargo", ["run", "-p", "beam_cli", "--bin", "beam", "--", ...args], {
  cwd: repoRoot,
  stdio: ["inherit", "pipe", "pipe"],
  encoding: "utf8",
})

if (result.error) {
  console.error(`beam: failed to run Rust CLI: ${result.error.message}`)
  process.exit(1)
}

if (result.stdout) process.stdout.write(result.stdout)
if (result.stderr) process.stderr.write(result.stderr)

process.exit(result.status ?? 1)

function optionValue(values: string[], name: string): string | undefined {
  const index = values.indexOf(name)
  return index === -1 ? undefined : values[index + 1]
}

function buildOptions(values: string[]) {
  const content: string[] = []
  for (let index = 1; index < values.length; index += 1) {
    if (values[index] === "--content") {
      content.push(values[index + 1])
      index += 1
    }
  }

  return {
    config: optionValue(values, "--config") ?? optionValue(values, "-c"),
    content: content.length > 0 ? content : undefined,
  }
}
