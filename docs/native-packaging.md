# Native Packaging Notes

Beam's Node API is designed to use the napi-rs binding from `crates/beam_node`.

Current local behavior:

- `crates/beam_node` exposes `compile(configJson, classStrings)` through napi.
- `beamcss/native` looks for packaged `.node` files in `packages/beamcss/native/`.
- `beamcss/cli-runner` prefers the native binding for config loading, scanning, and
  CSS compilation, then falls back to the repo-local Rust CLI bridge if no native
  binding is available.
- `beam build`, `beam check`, Vite, and PostCSS use the native JS build path when
  a binding is available.
- `beam init` and `beam dev` still use the repo-local Rust CLI bridge until those
  workflows are ported or packaged as native CLI behavior.
- `pnpm --filter beamcss build:native` builds `beam_node` and copies the local artifact into `packages/beamcss/native/`.

Release path:

1. `Native Release` builds `beam_node` on macOS arm64, macOS x64, Linux x64
   GNU, and Windows x64 MSVC.
2. Each build uploads a platform-tagged `.node` artifact named
   `beam_node.<platform>.node`.
3. The publish job stages those artifacts into `packages/beamcss/native/`, runs
   a dry-run pack check, then publishes `beamcss` and the scoped integration
   packages with npm provenance.

Decision: the first native release bundles platform artifacts in `beamcss`
itself. Split platform packages can come later if package size becomes painful.

Release work still needed:

1. Add the `NPM_TOKEN` secret with a short-lived/granular npm token.
2. Bump package versions before running the release workflow.
3. Port `beam init` and `beam dev` away from the repo-local `cargo run` bridge
   once release artifacts are dependable.
