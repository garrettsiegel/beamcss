import { buildCss } from "beamcss/cli-runner"

export interface BeamPostcssOptions {
  config?: string
  content?: string[]
}

export default function beamcssPostcss(options: BeamPostcssOptions = {}) {
  return {
    postcssPlugin: "@beamcss/postcss",
    Once(root: { append?: (css: string) => void }) {
      root.append?.(buildCss(options))
    },
  }
}

;(beamcssPostcss as typeof beamcssPostcss & { postcss: true }).postcss = true
