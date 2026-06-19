# Beam CSS Benchmarks

These benchmarks are smoke-level build comparisons for local development. They
are meant to be reproducible and useful for trend tracking, not final marketing
claims.

Run:

```sh
pnpm benchmark
```

The harness generates temporary Beam and Tailwind fixtures, runs Beam through
the same native JS build path used by the Vite/PostCSS integrations, runs
Tailwind through its CLI, and reports median build time plus output size.
Tailwind follows the current v4 CLI setup from the official docs: install
`tailwindcss` and `@tailwindcss/cli`, then build from a CSS file containing
`@import "tailwindcss";`.

Options:

```sh
BEAM_BENCH_REPEATS=40 pnpm benchmark
BEAM_BENCH_ITERATIONS=9 pnpm benchmark
BEAM_BENCH_KEEP=1 pnpm benchmark
```

Notes:

- The Beam fixture uses Beam primitives and grouped variants.
- The Tailwind fixture uses analogous Tailwind utility classes, not a byte-for-byte
  semantic clone.
- Cold dependency install time is not measured.
- Results vary by machine, CPU state, and filesystem.
