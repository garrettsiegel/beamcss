# Contributing

Beam CSS uses Conventional Commits and keeps the class-string grammar as a
public API contract.

## Local Checks

```sh
pnpm install
pnpm build
pnpm test
```

Rust checks:

```sh
cargo test --workspace
cargo audit
```

Parser changes should add golden output tests and keep the fuzz target running
without panics or hangs.

