import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { beamcss } from '@beamcss/vite'

export default defineConfig({
  plugins: [
    react(),
    beamcss({
      config: './beam.config.ts',
      // Scan real source for class strings. The docs example pages
      // (src/docs/pages/*) contain illustrative class="..." snippets that are
      // NOT real site classes — listing dirs explicitly excludes them so the
      // compiler doesn't try to resolve example utilities like `bg-blue-500`.
      content: [
        './index.html',
        './src/components',
        './src/pages',
        './src/docs/DocsLayout.tsx',
        './src/docs/DocsSidebar.tsx',
        './src/docs/OnThisPage.tsx',
        './src/docs/CodeBlock.tsx',
      ],
    }),
  ],
})
