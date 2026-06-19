import { buildCss } from "beamcss/cli-runner"

export interface BeamViteOptions {
  config?: string
  content?: string[]
}

interface BeamViteContext {
  addWatchFile?: (id: string) => void
}

interface HmrContext {
  modules?: unknown[]
  server?: {
    moduleGraph?: {
      getModuleById?: (id: string) => unknown
      invalidateModule?: (module: unknown) => void
    }
    ws?: { send?: (payload: unknown) => void }
  }
}

export function beamcss(options: BeamViteOptions = {}) {
  const virtualId = "virtual:beamcss.css"
  const resolvedVirtualId = `\0${virtualId}`
  let css = ""

  return {
    name: "@beamcss/vite",
    async buildStart(this: BeamViteContext) {
      this.addWatchFile?.(options.config ?? "beam.config.ts")
      for (const path of options.content ?? ["."]) {
        this.addWatchFile?.(path)
      }
      css = buildCss(options)
    },
    resolveId(id: string) {
      if (id === virtualId) return resolvedVirtualId
      return undefined
    },
    load(id: string) {
      if (id === resolvedVirtualId) return css
      return undefined
    },
    transformIndexHtml() {
      return [
        {
          tag: "style",
          attrs: { "data-beamcss": "" },
          children: css,
          injectTo: "head",
        },
      ]
    },
    async handleHotUpdate(context: HmrContext) {
      css = buildCss(options)
      const module = context.server?.moduleGraph?.getModuleById?.(resolvedVirtualId)
      if (module) {
        context.server?.moduleGraph?.invalidateModule?.(module)
      }
      context.server?.ws?.send?.({
        type: "update",
        updates: [
          {
            acceptedPath: virtualId,
            path: virtualId,
            timestamp: Date.now(),
            type: "js-update",
          },
        ],
      })
      return context.modules
    },
  }
}
