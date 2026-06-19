import type { BeamConfig } from "./index.js"
import { explain } from "./native.js"

export interface BeamCompletion {
  label: string
  detail: string
}

export interface BeamHover {
  className: string
  markdown: string
}

const primitives = [
  ["stack", "layout primitive: flex column"],
  ["row", "layout primitive: flex row"],
  ["cluster", "layout primitive: wrapping flex row"],
  ["grid", "layout primitive: CSS grid"],
  ["place", "layout primitive: centered grid"],
] as const

export function suggestBeamClasses(config: BeamConfig): BeamCompletion[] {
  const completions: BeamCompletion[] = []

  for (const value of ["0", "4", "8", "12", "16", "24"]) {
    for (const prefix of ["p", "px", "py", "m", "mx", "my", "gap"]) {
      completions.push({ label: `${prefix}-${value}`, detail: "pixel spacing" })
    }
  }

  for (const name of Object.keys(config.tokens.space ?? {})) {
    for (const prefix of ["p", "px", "py", "m", "mx", "my", "gap"]) {
      completions.push({ label: `${prefix}-${name}`, detail: "space token" })
    }
  }

  for (const name of Object.keys(config.tokens.color ?? {})) {
    completions.push({ label: `bg-${name}`, detail: "background color token" })
    completions.push({ label: `fg-${name}`, detail: "text color token" })
    completions.push({ label: `bd-${name}`, detail: "border color token" })
  }

  for (const name of Object.keys(config.tokens.radius ?? {})) {
    completions.push({ label: `round-${name}`, detail: "radius token" })
  }

  for (const name of Object.keys(config.tokens.text ?? {})) {
    completions.push({ label: `text-${name}`, detail: "font-size token" })
  }

  for (const name of Object.keys(config.tokens.font ?? {})) {
    completions.push({ label: `font-${name}`, detail: "font-family token" })
  }

  for (const [label, detail] of primitives) {
    completions.push({ label, detail })
    completions.push({ label: `${label}()`, detail: `${detail} with modifiers` })
  }

  for (const screen of Object.keys(config.tokens.screens ?? {})) {
    completions.push({ label: `${screen}:`, detail: "responsive variant" })
    completions.push({ label: `${screen}:()`, detail: "responsive variant group" })
  }

  return completions.sort((left, right) => left.label.localeCompare(right.label))
}

export function describeBeamClass(config: BeamConfig, className: string): BeamHover {
  const result = explain(config, [className])
  const declarations = result.class_strings
    .flatMap((classString) => classString.tokens)
    .flatMap((token) => token.atoms)
    .map((atom) => `- \`${atom.selector}\` -> \`${atom.declaration}\` in \`${atom.layer}\``)
  const errors = result.errors.map((error) => `- ${error.class_name}: ${error.message}`)

  return {
    className,
    markdown:
      declarations.length > 0
        ? declarations.join("\n")
        : `No resolved Beam atoms.\n${errors.join("\n")}`,
  }
}
