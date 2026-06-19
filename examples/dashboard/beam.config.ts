import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    spacing: { card: "1rem", section: "2rem" },
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
  shortcuts: {
    card: "flex direction-column gap-2 p-4 bg-surface rounded-lg border border-line",
  },
  recipes: {
    button: {
      base: "px-4 py-2 rounded-md hover:scale-105",
      variants: {
        secondary: "bg-panel text-fg hover:bg-panel+8",
        primary: "bg-accent text-on-accent hover:bg-accent+12",
      },
    },
  },
  utilities: {
    layout: true,
    spacing: true,
    colors: true,
    typography: true,
    effects: true,
  },
  background: "base",
  foreground: "fg",
})
