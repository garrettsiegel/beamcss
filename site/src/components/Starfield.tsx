import { useEffect, useRef } from 'react'

const ACCENT = '#7dd3c7'
// Exact "Standard" charset from asciiart.eu - index 0 = far/faint, last = close/bright
const CHARSET = ' .:+*#@'

interface Star {
  progress: number // 0 = just entered left, 1 = exiting right
  y: number
  speed: number   // progress units per second (faster = closer/brighter)
  opacity: number
  fontSize: number
}

function charAtProgress(progress: number): string {
  const t = Math.max(0, Math.min(1, progress))
  const idx = Math.min(CHARSET.length - 1, Math.floor(t * CHARSET.length))
  return CHARSET[idx]
}

function makeStar(h: number, randomProgress = true): Star {
  const tier = Math.random()
  // Three depth tiers: far (slow/dim/small) → mid → close (fast/bright/large)
  const speed =
    tier < 0.42 ? 0.04 + Math.random() * 0.05   // far
    : tier < 0.76 ? 0.12 + Math.random() * 0.11  // mid
    : 0.28 + Math.random() * 0.24                 // close

  const norm = Math.min(1, (speed - 0.04) / 0.48)
  return {
    progress: randomProgress ? Math.random() : 0,
    y: 8 + Math.random() * (h - 16),
    speed,
    opacity: 0.18 + norm * 0.68,
    fontSize: 9 + norm * 5,
  }
}

export function Starfield() {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // Capture as typed consts so closures keep the narrowed type
    const el: HTMLCanvasElement = canvas
    const cx: CanvasRenderingContext2D = ctx

    let w = 0
    let h = 0
    let stars: Star[] = []
    let rafId: number
    let lastTime = performance.now()

    function resize() {
      w = el.offsetWidth
      h = el.offsetHeight
      el.width = w * devicePixelRatio
      el.height = h * devicePixelRatio
      cx.scale(devicePixelRatio, devicePixelRatio)
    }

    function init() {
      stars = Array.from({ length: 90 }, () => makeStar(h, true))
    }

    function tick(now: number) {
      const dt = Math.min(0.05, (now - lastTime) / 1000)
      lastTime = now

      cx.clearRect(0, 0, w, h)

      for (const s of stars) {
        s.progress += s.speed * dt

        if (s.progress > 1.02) {
          Object.assign(s, makeStar(h, false))
          continue
        }

        const char = charAtProgress(1 - s.progress)
        if (char === ' ') continue

        cx.globalAlpha = s.opacity
        cx.fillStyle = ACCENT
        cx.font = `${s.fontSize}px "JetBrains Mono", ui-monospace, monospace`
        cx.fillText(char, s.progress * w, s.y)
      }

      cx.globalAlpha = 1
      rafId = requestAnimationFrame(tick)
    }

    resize()
    init()
    rafId = requestAnimationFrame(tick)

    const ro = new ResizeObserver(() => {
      resize()
      for (const s of stars) {
        if (s.y > h) s.y = Math.random() * (h - 16)
      }
    })
    ro.observe(el)

    return () => {
      cancelAnimationFrame(rafId)
      ro.disconnect()
    }
  }, [])

  return <canvas ref={canvasRef} data-starfield-canvas />
}
