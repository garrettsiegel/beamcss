# Supply Chain Checklist

Beam CSS treats package publishing as a security boundary. Use this checklist
before any public release.

## npm Accounts And Org

- Require 2FA for every npm maintainer.
- Keep the `beamcss` npm org owner list small.
- Prefer maintainer roles with the least access needed.
- Remove inactive maintainers before each minor release.

## Publish Tokens

- Use granular npm tokens.
- Prefer short-lived tokens.
- Store publish tokens only as GitHub Actions secrets.
- Rotate `NPM_TOKEN` after any maintainer or workflow access change.
- Never commit `.npmrc` files containing credentials.

## Provenance

- Publish from GitHub Actions with `--provenance`.
- Keep workflow `id-token: write` permissions limited to release workflows.
- Release from tags or explicit manual workflow dispatches.
- Verify published packages show npm provenance before announcement.

## Package Scripts

- Do not add `preinstall`, `install`, `postinstall`, `prepublish`, or `prepare`
  scripts to any package.
- CI runs `pnpm security:package-scripts` to enforce this.
- If native artifacts are needed, build them in CI and include them in the
  package tarball instead of compiling during user install.

## Lockfiles And Dependencies

- Commit `pnpm-lock.yaml`, `Cargo.lock`, and `fuzz/Cargo.lock`.
- Run `pnpm audit --audit-level moderate` and `cargo audit` before release.
- Keep runtime dependencies small and boring.
- Prefer dev-only benchmark/test dependencies over production dependencies.

## Release Preflight

```sh
pnpm security:package-scripts
pnpm test
pnpm benchmark
pnpm audit --audit-level moderate
cargo audit
```
