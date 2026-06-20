import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// Minimal SSR build for prerendering - react only, no beamcss plugin
// (the client build already injected the compiled CSS into index.html).
export default defineConfig({
  plugins: [react()],
  build: {
    ssr: 'src/entry-server.tsx',
    outDir: 'dist/server',
    emptyOutDir: false,
  },
})
