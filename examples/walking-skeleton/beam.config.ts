import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    space: { card: "1rem", section: "1.5rem" },
    color: {
      base: "#0b0b0c",
      surface: "#16161a",
      fg: "#e8e8ea",
      muted: "#6b7280",
      accent: "#3b82f6",
      "on-accent": "#ffffff",
    },
    radius: {
      md: "8px",
    },
    text: {
      base: "16px",
      lg: "20px",
      xl: "28px",
    },
    font: {
      ui: "Inter, system-ui, sans-serif",
    },
    screens: {
      tablet: "48rem",
      desktop: "64rem",
      wide: "80rem",
      "mobile-landscape": "(max-width:47.999rem) and (orientation:landscape)",
    },
  },
  background: "base",
  foreground: "fg",
})
