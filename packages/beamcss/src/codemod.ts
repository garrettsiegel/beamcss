export interface CodemodResult {
  className: string
  warnings: CodemodWarning[]
}

export interface CodemodWarning {
  className: string
  message: string
}

interface ConvertedToken {
  variants: string[]
  base: string
  warning?: CodemodWarning
}

const textSizeNames = new Set(["xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl"])
const variantNames = new Set([
  "hover",
  "focus",
  "focus-visible",
  "focus-within",
  "active",
  "disabled",
  "first",
  "last",
  "odd",
  "even",
  "dark",
  "group-hover",
  "group-focus",
  "peer-checked",
  "peer-focus",
  "motion-safe",
  "print",
  "sm",
  "md",
  "lg",
  "xl",
])

export function tailwindToBeamClassName(className: string): CodemodResult {
  const converted = splitClassName(className).map(convertToken)
  return {
    className: foldVariants(converted),
    warnings: converted.flatMap((token) => (token.warning ? [token.warning] : [])),
  }
}

function convertToken(raw: string): ConvertedToken {
  const parts = splitVariantParts(raw)
  const base = parts.pop() ?? raw
  const variants = parts.filter((part) => variantNames.has(part) || isArbitraryVariant(part))
  const unknownVariants = parts.filter(
    (part) => !variantNames.has(part) && !isArbitraryVariant(part),
  )
  const mapped = mapBase(base)
  const warning =
    unknownVariants.length > 0
      ? {
          className: raw,
          message: `unsupported variants: ${unknownVariants.join(", ")}`,
        }
      : mapped.warning

  return {
    variants,
    base: mapped.base,
    warning,
  }
}

function mapBase(base: string): { base: string; warning?: CodemodWarning } {
  if (base === "rounded") return { base: "round-md" }
  if (base.startsWith("rounded-")) return { base: `round-${base.slice("rounded-".length)}` }
  if (base.startsWith("border-") && base !== "border-0") {
    const value = base.slice("border-".length)
    if (/^\d+$/.test(value)) return { base }
    return { base: `bd-${value}` }
  }
  if (base.startsWith("text-")) {
    const value = base.slice("text-".length)
    return textSizeNames.has(value) ? { base } : { base: `fg-${value}` }
  }
  if (base === "font-sans") return { base: "font-ui" }
  if (base === "font-mono") return { base: "font-mono" }
  if (base === "flex-col") return { base: "direction-column" }
  if (base === "flex-row") return { base: "direction-row" }
  if (base === "items-center") return { base: "align-center" }
  if (base === "justify-between") return { base: "justify-between" }
  if (base === "justify-center") return { base: "justify-center" }
  if (isBeamCompatibleBase(base)) return { base }

  return {
    base,
    warning: {
      className: base,
      message: "left unchanged; no conservative Beam mapping is known",
    },
  }
}

function isBeamCompatibleBase(base: string): boolean {
  return /^(p|px|py|pt|pr|pb|pl|m|mx|my|mt|mr|mb|ml|gap|gap-x|gap-y|bg|fg|bd|round|text|font|w|h|min-w|min-h|max-w|max-h|scale|cols|rows)-/.test(
    base,
  )
}

function foldVariants(tokens: ConvertedToken[]): string {
  const output: string[] = []
  const grouped = new Map<string, { variants: string[]; bases: string[]; index: number }>()

  for (const token of tokens) {
    if (token.variants.length === 0) {
      output.push(token.base)
      continue
    }

    const key = token.variants.join(":")
    const group = grouped.get(key)
    if (group) {
      group.bases.push(token.base)
    } else {
      grouped.set(key, {
        variants: token.variants,
        bases: [token.base],
        index: output.length,
      })
      output.push("")
    }
  }

  for (const group of grouped.values()) {
    output[group.index] =
      group.bases.length === 1
        ? `${group.variants.join(":")}:${group.bases[0]}`
        : `${group.variants.join(":")}:(${group.bases.join(" ")})`
  }

  return output.filter(Boolean).join(" ")
}

function splitClassName(className: string): string[] {
  const tokens: string[] = []
  let start: number | undefined
  let bracketDepth = 0

  for (let index = 0; index < className.length; index += 1) {
    const char = className[index]
    if (start === undefined && !/\s/.test(char)) start = index
    if (char === "[") bracketDepth += 1
    if (char === "]") bracketDepth = Math.max(0, bracketDepth - 1)
    if (/\s/.test(char) && bracketDepth === 0 && start !== undefined) {
      tokens.push(className.slice(start, index))
      start = undefined
    }
  }

  if (start !== undefined) tokens.push(className.slice(start))
  return tokens
}

function splitVariantParts(token: string): string[] {
  const parts: string[] = []
  let start = 0
  let bracketDepth = 0

  for (let index = 0; index < token.length; index += 1) {
    const char = token[index]
    if (char === "[") bracketDepth += 1
    if (char === "]") bracketDepth = Math.max(0, bracketDepth - 1)
    if (char === ":" && bracketDepth === 0) {
      parts.push(token.slice(start, index))
      start = index + 1
    }
  }

  parts.push(token.slice(start))
  return parts
}

function isArbitraryVariant(value: string): boolean {
  return value.startsWith("[") && value.endsWith("]")
}
