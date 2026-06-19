import { copyFileSync, existsSync, mkdirSync } from "node:fs"
import { dirname, resolve } from "node:path"
import { fileURLToPath } from "node:url"

const packageDir = dirname(dirname(fileURLToPath(import.meta.url)))
const repoRoot = resolve(packageDir, "../..")
const targetDir = resolve(repoRoot, "target/release")
const nativeDir = resolve(packageDir, "native")

const source = sourcePath(process.platform)
const targetName = `beam_node.${process.env.BEAM_NATIVE_TAG ?? nativePlatformName()}.node`

if (!existsSync(source)) {
  throw new Error(`Native artifact not found: ${source}`)
}

mkdirSync(nativeDir, { recursive: true })
copyFileSync(source, resolve(nativeDir, targetName))
copyFileSync(source, resolve(nativeDir, "beam_node.node"))

console.log(`copied ${source} -> native/${targetName}`)

function sourcePath(platform) {
  if (platform === "win32") {
    return resolve(targetDir, "beam_node.dll")
  }
  if (platform === "darwin") {
    return resolve(targetDir, "libbeam_node.dylib")
  }
  return resolve(targetDir, "libbeam_node.so")
}

function nativePlatformName() {
  if (process.platform === "linux" && process.arch === "x64") {
    return "linux-x64-gnu"
  }
  if (process.platform === "win32" && process.arch === "x64") {
    return "win32-x64-msvc"
  }
  return `${process.platform}-${process.arch}`
}
