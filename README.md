# beamcss

beamcss is an early CSS framework project with a Vite, React, and TypeScript
front end for docs, demos, and launch experiments.

## Development

```sh
npm run dev
npm run build
npm run pack:check
```

## Vercel

This repo includes `vercel.json` with the build command and output directory
Vercel needs:

- Build command: `npm run build`
- Output directory: `dist`
- Framework preset: Vite

If the Vercel project is already connected to this repository, push these files
and trigger a new deployment. Vercel should install from `package-lock.json`,
run the build command, and publish `dist`.

## Package Stub

The publishable framework entry point is still `index.js`. Replace the
placeholder export with the API you want to publish as the framework develops.
