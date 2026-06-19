import { copyFileSync, existsSync, mkdirSync, readdirSync } from "node:fs"
import { dirname, extname, resolve } from "node:path"
import { fileURLToPath } from "node:url"

const packageDir = dirname(dirname(fileURLToPath(import.meta.url)))
const nativeDir = resolve(packageDir, "native")
const sourceDir = resolve(process.argv.slice(2).find((a) => a !== "--") ?? "native-artifacts")
const expectedArtifacts = new Set([
  "beam_node.darwin-arm64.node",
  "beam_node.darwin-x64.node",
  "beam_node.linux-x64-gnu.node",
  "beam_node.win32-x64-msvc.node",
])

if (!existsSync(sourceDir)) {
  throw new Error(`Native artifact directory not found: ${sourceDir}`)
}

mkdirSync(nativeDir, { recursive: true })

const staged = []
for (const filePath of walk(sourceDir)) {
  const fileName = filePath.split(/[\\/]/).at(-1)
  if (!fileName || extname(fileName) !== ".node") continue
  if (!expectedArtifacts.has(fileName)) continue

  copyFileSync(filePath, resolve(nativeDir, fileName))
  staged.push(fileName)
}

const missing = [...expectedArtifacts].filter((fileName) => !staged.includes(fileName))
if (missing.length > 0) {
  throw new Error(`Missing native artifacts: ${missing.join(", ")}`)
}

console.log(`staged native artifacts: ${staged.sort().join(", ")}`)

function* walk(dir) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const entryPath = resolve(dir, entry.name)
    if (entry.isDirectory()) {
      yield* walk(entryPath)
    } else if (entry.isFile()) {
      yield entryPath
    }
  }
}
