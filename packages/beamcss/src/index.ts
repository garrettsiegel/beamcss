export type TokenMap = Record<string, string>

export interface BeamTokens {
  space?: TokenMap
  spacing?: TokenMap
  color?: TokenMap
  radius?: TokenMap
  text?: TokenMap
  font?: TokenMap
  screens?: TokenMap
}

export interface BeamConfig {
  presets?: BeamPreset[]
  tokens: BeamTokens
  shortcuts?: Record<string, string>
  recipes?: Record<string, BeamRecipe>
  utilities?: Record<string, boolean>
  /** Color token painted onto `body`'s background by the reset. Names a key in `tokens.color`. */
  background?: string
  /** Color token used for `body`'s text color by the reset. Names a key in `tokens.color`. */
  foreground?: string
}

export interface BeamPreset {
  tokens?: Partial<BeamTokens>
  shortcuts?: Record<string, string>
  recipes?: Record<string, BeamRecipe>
  utilities?: Record<string, boolean>
  background?: string
  foreground?: string
}

export interface BeamRecipe {
  base?: string
  variants?: Record<string, string>
}

export function defineConfig<const T extends BeamConfig>(config: T): T {
  return config
}

export function vars(values: Record<string, string | number>): Record<string, string> {
  return Object.fromEntries(
    Object.entries(values).map(([key, value]) => [`--${key}`, String(value)]),
  )
}

export { extractClassStrings, scanFiles } from "./scanner.js"
export { buildCss, buildCssNative, loadConfigSync, parseConfigSource, scanClassStringsSync } from "./cli-runner.js"
export { tailwindToBeamClassName } from "./codemod.js"
export type { CodemodResult, CodemodWarning } from "./codemod.js"
export { describeBeamClass, suggestBeamClasses } from "./language.js"
export type { BeamCompletion, BeamHover } from "./language.js"
export { compile, explain, loadNativeBinding } from "./native.js"
export type {
  CompileMessage,
  CompileResult,
  ExplainAtom,
  ExplainClassString,
  ExplainResult,
  ExplainToken,
} from "./native.js"
