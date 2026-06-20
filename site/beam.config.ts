import { defineConfig } from 'beamcss'

export default defineConfig({
  presets: [
    {
      shortcuts: {
        container: 'w-full max-w-[64rem] mx-[auto]',
      },
    },
  ],
  tokens: {
    spacing: {
      xs: '4px',
      sm: '12px',
      md: '20px',
      lg: '32px',
      xl: '56px',
      '2xl': '96px',
    },
    color: {
      base: '#0e1116',
      surface: '#171d24',
      panel: '#202832',
      fg: '#f4f1ea',
      muted: '#9aa6ad',
      accent: '#7dd3c7',
      'on-accent': '#071013',
      success: '#9ad8aa',
      warning: '#e7b96a',
      line: '#2b3440',
    },
    radius: {
      sm: '4px',
      md: '8px',
    },
    text: {
      sm: '13px',
      base: '16px',
      lg: '20px',
      xl: '28px',
      '2xl': '40px',
      '3xl': '56px',
      '4xl': '3.75rem',
    },
    font: {
      ui: 'Inter, system-ui, sans-serif',
      heading: '"Space Grotesk", Inter, system-ui, sans-serif',
      mono: '"JetBrains Mono", ui-monospace, monospace',
    },
    screens: {
      tablet: '48rem',
      desktop: '64rem',
    },
  },
  utilities: {
    layout: true,
    spacing: true,
    colors: true,
    typography: true,
    effects: true,
  },
  background: 'base',
  foreground: 'fg',
})
