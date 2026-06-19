import { existsSync } from "node:fs"
import { createRequire } from "node:module"
import { dirname, resolve } from "node:path"
import { fileURLToPath } from "node:url"

import type { BeamConfig } from "./index.js"

export interface CompileResult {
  css: string
  warnings: CompileMessage[]
  errors: CompileMessage[]
}

export interface CompileMessage {
  class_name: string
  message: string
}

export interface ExplainResult {
  class_strings: ExplainClassString[]
  warnings: CompileMessage[]
  errors: CompileMessage[]
}

export interface ExplainClassString {
  class_string: string
  tokens: ExplainToken[]
}

export interface ExplainToken {
  raw: string
  kind: "utility" | "group" | "invalid"
  variants: string[]
  base?: string
  atoms: ExplainAtom[]
  errors: CompileMessage[]
}

export interface ExplainAtom {
  class_name: string
  selector: string
  variants: string[]
  base: string
  declaration: string
  layer: "beam.base" | "beam.utilities"
  media: string[]
  pseudos: string[]
}

interface NativeBinding {
  compile(configJson: string, classStrings: string[]): string
  explain(configJson: string, classStrings: string[]): string
}

export function compile(config: BeamConfig, classStrings: string[]): CompileResult {
  const binding = loadNativeBinding()

  if (!binding) {
    throw new Error("Beam native binding is not available in this package build")
  }

  return JSON.parse(binding.compile(JSON.stringify(config), classStrings)) as CompileResult
}

export function explain(config: BeamConfig, classStrings: string[]): ExplainResult {
  const binding = loadNativeBinding()

  if (!binding) {
    throw new Error("Beam native binding is not available in this package build")
  }

  return JSON.parse(binding.explain(JSON.stringify(config), classStrings)) as ExplainResult
}

export function loadNativeBinding(): NativeBinding | undefined {
  const require = createRequire(import.meta.url)
  const here = dirname(fileURLToPath(import.meta.url))
  const platformName = nativePlatformName()
  const candidates = [
    resolve(here, "../beam_node.node"),
    resolve(here, "../native/beam_node.node"),
    resolve(here, `../native/beam_node.${platformName}.node`),
  ]

  for (const candidate of candidates) {
    if (!existsSync(candidate)) continue
    return require(candidate) as NativeBinding
  }

  return undefined
}

function nativePlatformName(): string {
  if (process.platform === "linux" && process.arch === "x64") {
    return "linux-x64-gnu"
  }
  if (process.platform === "win32" && process.arch === "x64") {
    return "win32-x64-msvc"
  }
  return `${process.platform}-${process.arch}`
}
