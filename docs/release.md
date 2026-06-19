# Release Checklist

Use this checklist for a real `0.x` release.

## Preflight

```sh
pnpm install --frozen-lockfile
pnpm security:package-scripts
pnpm test
pnpm benchmark
pnpm --filter beamcss pack:check
npm pack --dry-run --cache /tmp/beamcss-npm-cache # from packages/vite
npm pack --dry-run --cache /tmp/beamcss-npm-cache # from packages/postcss
npm pack --dry-run --cache /tmp/beamcss-npm-cache # from packages/mcp
cargo test --workspace
cargo audit
```

## Versioning

- Bump every package that will publish.
- Keep `beamcss`, Rust crates, and native binding versions aligned for public
  releases.
- Do not republish `0.0.0` scoped stubs; npm requires a version bump.
- Update `CHANGELOG.md` before tagging.

## Native Artifacts

- Run the `Native Release` workflow from a tag.
- Confirm artifacts exist for:
  - `darwin-arm64`
  - `darwin-x64`
  - `linux-x64-gnu`
  - `win32-x64-msvc`
- Confirm `beamcss` package tarball includes `native/beam_node.*.node`.

## npm

- Confirm `beamcss` org maintainer 2FA policy is enabled.
- Confirm `NPM_TOKEN` is granular, short-lived, and stored only in GitHub
  Actions secrets.
- Publish with provenance from GitHub Actions.
- Verify npm package pages show provenance before announcement.

## Post-release

- Install into a fresh Vite app.
- Run `beam init --template vite`.
- Run `beam build`, `beam check`, and `beam codemod`.
- Confirm `https://beamcss.dev/llms.txt` and `/llms-full.txt` are live.
- Create a GitHub release with the changelog notes.
