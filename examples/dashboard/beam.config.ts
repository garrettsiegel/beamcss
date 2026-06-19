import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    space: { card: "1rem", section: "2rem" },
    color: {
      base: "#0b0b0c",
      surface: "#16161a",
      panel: "#202027",
      fg: "#f4f4f5",
      muted: "#9ca3af",
      line: "#34343d",
      accent: "#3b82f6",
      success: "#22c55e",
      warning: "#f59e0b",
      "on-accent": "#ffffff",
    },
    radius: {
      sm: "4px",
      md: "8px",
      lg: "16px",
      full: "9999px",
    },
    text: {
      sm: "14px",
      base: "16px",
      lg: "20px",
      xl: "28px",
    },
    font: {
      ui: "Inter, system-ui, sans-serif",
      mono: "ui-monospace, monospace",
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
