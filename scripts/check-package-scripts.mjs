import { existsSync, readdirSync, readFileSync } from "node:fs"
import { join } from "node:path"

const repoRoot = process.cwd()
const blockedScripts = new Set([
  "preinstall",
  "install",
  "postinstall",
  "prepublish",
  "prepare",
])
const packageJsonFiles = [
  "package.json",
  ...workspacePackageJsonFiles("apps"),
  ...workspacePackageJsonFiles("packages"),
  ...workspacePackageJsonFiles("crates"),
]

const violations = []

for (const packageJson of packageJsonFiles) {
  const manifest = JSON.parse(readFileSync(join(repoRoot, packageJson), "utf8"))
  const scripts = manifest.scripts ?? {}

  for (const scriptName of Object.keys(scripts)) {
    if (blockedScripts.has(scriptName)) {
      violations.push(`${packageJson}: blocked lifecycle script "${scriptName}"`)
    }
  }
}

if (violations.length > 0) {
  console.error("Blocked npm lifecycle scripts found:")
  for (const violation of violations) {
    console.error(`- ${violation}`)
  }
  process.exit(1)
}

console.log(`checked ${packageJsonFiles.length} package manifests: no blocked lifecycle scripts`)

function workspacePackageJsonFiles(dir) {
  try {
    return readdirSync(join(repoRoot, dir), { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => join(dir, entry.name, "package.json"))
      .filter((packageJson) => existsSync(join(repoRoot, packageJson)))
  } catch {
    return []
  }
}
