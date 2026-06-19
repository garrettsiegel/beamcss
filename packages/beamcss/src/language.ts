import type { BeamConfig, BeamPreset, BeamTokens } from "./index.js"
import { explain } from "./native.js"

export interface BeamCompletion {
  label: string
  detail: string
}

export interface BeamHover {
  className: string
  markdown: string
}

const layoutUtilities = [
  ["flex", "display flex"],
  ["grid", "display grid"],
  ["direction-column", "flex column"],
  ["direction-row", "flex row"],
  ["wrap", "flex wrap"],
  ["align-center", "align items center"],
  ["justify-center", "justify content center"],
  ["justify-between", "justify content between"],
  ["place-center", "place items center"],
] as const

export function suggestBeamClasses(config: BeamConfig): BeamCompletion[] {
  const resolved = resolveConfig(config)
  const completions: BeamCompletion[] = []

  if (moduleEnabled(resolved, "spacing")) {
    for (const value of ["0", "4", "8", "12", "16", "24"]) {
      for (const prefix of ["p", "px", "py", "m", "mx", "my", "gap"]) {
        completions.push({ label: `${prefix}-${value}`, detail: "pixel spacing" })
      }
    }

    for (const name of Object.keys(spacingTokens(resolved.tokens))) {
      for (const prefix of ["p", "px", "py", "m", "mx", "my", "gap"]) {
        completions.push({ label: `${prefix}-${name}`, detail: "spacing token" })
      }
    }

    completions.push({ label: "padding:()", detail: "padding utility group" })
    completions.push({ label: "margin:()", detail: "margin utility group" })
  }

  if (moduleEnabled(resolved, "colors")) {
    for (const name of Object.keys(resolved.tokens.color ?? {})) {
      completions.push({ label: `bg-${name}`, detail: "background color token" })
      completions.push({ label: `text-${name}`, detail: "text color token" })
      completions.push({ label: `border-${name}`, detail: "border color token" })
    }
  }

  if (moduleEnabled(resolved, "layout")) {
    for (const name of Object.keys(resolved.tokens.radius ?? {})) {
      completions.push({ label: `rounded-${name}`, detail: "radius token" })
    }

    for (const [label, detail] of layoutUtilities) {
      completions.push({ label, detail })
    }

    completions.push({ label: "border:()", detail: "border utility group" })
  }

  if (moduleEnabled(resolved, "typography")) {
    for (const name of Object.keys(resolved.tokens.text ?? {})) {
      completions.push({ label: `text-${name}`, detail: "font-size token" })
    }

    for (const name of Object.keys(resolved.tokens.font ?? {})) {
      completions.push({ label: `font-${name}`, detail: "font-family token" })
    }

    completions.push({ label: "text:()", detail: "text utility group" })
    completions.push({ label: "text-center", detail: "text alignment" })
  }

  if (moduleEnabled(resolved, "effects")) {
    completions.push({ label: "opacity-75", detail: "opacity utility" })
    completions.push({ label: "scale-105", detail: "scale transform" })
  }

  for (const name of Object.keys(resolved.shortcuts ?? {})) {
    completions.push({ label: name, detail: "shortcut" })
  }

  for (const [name, recipe] of Object.entries(resolved.recipes ?? {})) {
    completions.push({ label: name, detail: "recipe base" })
    for (const variant of Object.keys(recipe.variants ?? {})) {
      completions.push({ label: `${name}:${variant}`, detail: "recipe variant" })
    }
  }

  for (const screen of Object.keys(resolved.tokens.screens ?? {})) {
    completions.push({ label: `${screen}:`, detail: "responsive variant" })
    completions.push({ label: `${screen}:()`, detail: "responsive variant group" })
  }

  return completions.sort((left, right) => left.label.localeCompare(right.label))
}

function resolveConfig(config: BeamConfig): BeamConfig {
  const resolved: BeamConfig = { tokens: {} }

  for (const preset of config.presets ?? []) {
    mergePreset(resolved, preset)
  }

  mergeTokens(resolved.tokens, config.tokens)
  resolved.shortcuts = { ...(resolved.shortcuts ?? {}), ...(config.shortcuts ?? {}) }
  resolved.recipes = { ...(resolved.recipes ?? {}), ...(config.recipes ?? {}) }
  resolved.utilities = { ...(resolved.utilities ?? {}), ...(config.utilities ?? {}) }
  resolved.background = config.background ?? resolved.background
  resolved.foreground = config.foreground ?? resolved.foreground

  return resolved
}

function mergePreset(config: BeamConfig, preset: BeamPreset): void {
  mergeTokens(config.tokens, preset.tokens ?? {})
  config.shortcuts = { ...(config.shortcuts ?? {}), ...(preset.shortcuts ?? {}) }
  config.recipes = { ...(config.recipes ?? {}), ...(preset.recipes ?? {}) }
  config.utilities = { ...(config.utilities ?? {}), ...(preset.utilities ?? {}) }
  config.background = preset.background ?? config.background
  config.foreground = preset.foreground ?? config.foreground
}

function mergeTokens(target: BeamTokens, source: Partial<BeamTokens>): void {
  target.spacing = { ...spacingTokens(target), ...spacingTokens(source) }
  target.color = { ...(target.color ?? {}), ...(source.color ?? {}) }
  target.radius = { ...(target.radius ?? {}), ...(source.radius ?? {}) }
  target.text = { ...(target.text ?? {}), ...(source.text ?? {}) }
  target.font = { ...(target.font ?? {}), ...(source.font ?? {}) }
  target.screens = { ...(target.screens ?? {}), ...(source.screens ?? {}) }
}

function spacingTokens(tokens: Partial<BeamTokens>): Record<string, string> {
  return tokens.spacing ?? tokens.space ?? {}
}

function moduleEnabled(config: BeamConfig, module: string): boolean {
  return config.utilities?.[module] ?? true
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
