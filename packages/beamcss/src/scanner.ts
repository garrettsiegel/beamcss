import { readdir, readFile, stat } from "node:fs/promises"
import { extname, basename, join } from "node:path"

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

export async function scanFiles(paths: string[]): Promise<string[]> {
  const files = await collectFiles(paths.length > 0 ? paths : ["."])
  const classStrings: string[] = []

  for (const file of files) {
    classStrings.push(...extractClassStrings(await readFile(file, "utf8")))
  }

  return classStrings
}

export function extractClassStrings(source: string): string[] {
  const classStrings: string[] = []

  for (const attribute of ["class", "className"]) {
    let offset = 0
    while (offset < source.length) {
      const start = source.indexOf(attribute, offset)
      if (start === -1) break
      if (!hasAttributeBoundary(source, start, attribute.length)) {
        offset = start + attribute.length
        continue
      }

      const value = readAttributeValue(source, start + attribute.length)
      if (value) {
        classStrings.push(value.value)
        offset = value.next
      } else {
        offset = start + attribute.length
      }
    }
  }

  return classStrings
}

async function collectFiles(paths: string[]): Promise<string[]> {
  const files = new Set<string>()

  for (const path of paths) {
    for (const file of await collectPath(path)) {
      files.add(file)
    }
  }

  return [...files].sort()
}

async function collectPath(path: string): Promise<string[]> {
  const details = await stat(path).catch(() => undefined)
  if (!details) return []

  if (details.isFile()) {
    return supportedExtensions.has(extname(path)) ? [path] : []
  }

  if (!details.isDirectory() || ignoredDirs.has(basename(path))) {
    return []
  }

  const files = await Promise.all(
    (await readdir(path)).map((entry) => collectPath(join(path, entry))),
  )
  return files.flat()
}

function hasAttributeBoundary(source: string, start: number, length: number): boolean {
  const before = source[start - 1]
  const after = source[start + length]
  return !isIdent(before) && !isIdent(after)
}

function isIdent(value: string | undefined): boolean {
  return !!value && /[A-Za-z0-9_-]/.test(value)
}

function readAttributeValue(
  source: string,
  start: number,
): { value: string; next: number } | undefined {
  let index = skipWhitespace(source, start)
  if (source[index] !== "=") return undefined
  index += 1
  index = skipWhitespace(source, index)

  const closesBrace = source[index] === "{"
  if (closesBrace) {
    index += 1
    index = skipWhitespace(source, index)
  }

  const quote = source[index]
  if (quote !== '"' && quote !== "'" && quote !== "`") return undefined
  index += 1
  const valueStart = index
  let escaped = false

  while (index < source.length) {
    const char = source[index]
    if (escaped) {
      escaped = false
    } else if (char === "\\") {
      escaped = true
    } else if (char === quote) {
      let next = index + 1
      if (closesBrace) {
        next = skipWhitespace(source, next)
        if (source[next] === "}") next += 1
      }
      return { value: source.slice(valueStart, index), next }
    }
    index += 1
  }

  return undefined
}

function skipWhitespace(source: string, start: number): number {
  let index = start
  while (/\s/.test(source[index] ?? "")) {
    index += 1
  }
  return index
}
