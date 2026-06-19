# npm Publishing Checklist

Use this only after the npm org, 2FA, and provenance settings are ready.

## Claim scoped package names

Prerequisite: create the npm organization named `beamcss`. npm only allows
publishing `@beamcss/*` packages after the matching organization scope exists
and the publishing user has write access to it.

```sh
pnpm --filter @beamcss/vite build
pnpm --filter @beamcss/postcss build
pnpm --filter @beamcss/mcp build

pnpm --filter @beamcss/vite publish --access public --provenance
pnpm --filter @beamcss/postcss publish --access public --provenance
pnpm --filter @beamcss/mcp publish --access public --provenance
```

The `Native Release` GitHub Actions workflow can also publish these scoped
stubs after `NPM_TOKEN` is configured. Bump versions before re-publishing any
package that already exists on npm.

## Current registry check

As of May 22, 2026:

- `beamcss` exists on npm at `0.1.0`.
- `@beamcss/vite` exists on npm at `0.0.0`.
- `@beamcss/postcss` exists on npm at `0.0.0`.
- `@beamcss/mcp` exists on npm at `0.0.0`.

The scoped package names are claimed. Future publishes need version bumps before
re-publishing any package that already exists on npm.
