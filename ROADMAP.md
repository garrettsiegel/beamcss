# Beam CSS — Build Roadmap

> From placeholder package to adopted framework. Companion to `beam-css-spec.md` (the syntax spec is the source of truth; this is the build order).

**How to use this:** each phase has a goal, concrete tasks, and an exit criterion. Don't move to the next phase until the current one's exit criterion is met. Ship a thin vertical slice early, then widen — don't try to build the whole grammar before anything compiles.

---

## Guiding principles (carry these through every phase)

- **The grammar is the public API.** Once people ship Beam, the class-string grammar (spec §8) is a stability contract under semver. Treat breaking it like breaking a function signature.
- **Security is supply-chain first.** A popular npm package is a malware target; spend the security budget there (Phase 3).
- **AI-native, not docs-dependent.** Build the machine-readable surfaces (llms.txt, MCP) as first-class, because agents — not docs visitors — are how people will discover and write Beam.
- **Sustainability is decided early.** The core stays free/MIT; money comes from services around it (hosted tokens/design-system, team/MCP backend, sponsors), never from docs traffic. Don't repeat Tailwind's funnel.
- **Dogfood constantly.** Every feature gets used in a real example app before it's "done."

---

## Phase 0 — Repo foundations

**Goal:** turn the placeholder into a credible, secure-by-default monorepo.

- [ ] **Pick a license.** Recommend **MIT** for the core (maximizes adoption); your `package.json` currently says `UNLICENSED`. Add a `LICENSE` file and update `package.json`. The monetization layer lives in separate, separately-licensed services later.
- [ ] **Restructure to a monorepo** (pnpm workspaces). Reserve the package layout now:
  - `beamcss` — umbrella the user installs (engine + CLI)
  - `@beamcss/vite` — Vite plugin
  - `@beamcss/postcss` — PostCSS plugin (broad bundler compat)
  - `@beamcss/mcp` — agent server (Phase 4)
  - Publish empty `0.0.x` stubs of the scoped names to claim them.
- [ ] **Decide the Rust↔JS binding.** Recommend **napi-rs** (native addon) for the Node engine — same approach as Lightning CSS / Tailwind Oxide / SWC. Add a **WASM** build later for the web playground.
- [ ] **Add governance + safety files:** `SECURITY.md` (disclosure path), `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, issue/PR templates.
- [ ] **CI skeleton** (GitHub Actions): build, lint, test, plus `cargo audit` and `npm audit` on every PR. No `postinstall` scripts anywhere.
- [ ] Keep the `beamcss` placeholder published so the name stays held while you build.

**Exit criterion:** `pnpm i && pnpm build` works across the monorepo; CI is green; license and security files are in place.

---

## Phase 1 — Core engine (the heart)

**Goal:** given config + class strings, emit correct atomic CSS. Build it as vertical slices.

- [ ] **Slice 1 — walking skeleton.** Load `beam.config.ts` tokens → parse a flat list of basic utilities (`p-4`, `bg-surface`, `gap-2`) → emit atomic CSS under `@layer` with deduping. End to end, even if tiny.
- [ ] **Slice 2 — the grammar.** Implement the full parser from spec §8: variants, the `variant:(...)` **grouping** unfold, nesting, and stacking. This is the signature feature — get it right.
- [ ] **Slice 3 — layout primitives.** `stack`, `row`, `cluster`, `grid`, `place` with their modifier vocabulary and documented defaults (spec §6).
- [ ] **Slice 4 — values.** Token steps/names, static arbitrary `[…]`, and dynamic `(--var)` → `var()`.
- [ ] **Slice 5 — output pipeline.** Cascade-layer ordering, global dedup, tree-shaking to used atoms, modern-CSS transpile + minify for target browsers.
- [ ] **Fuzz the parser from day one** (cargo-fuzz). It must never panic or hang on malformed/pathological input. Cap input sizes. Target `#![forbid(unsafe_code)]`.

**Exit criterion:** a golden-file test passes — a representative source file of Beam markup compiles to expected atomic CSS, and the fuzzer runs clean over a large corpus.

---

## Phase 2 — Integration & DX

**Goal:** make it usable in a real project.

- [ ] **Node bindings** via napi-rs exposing the engine to JS.
- [ ] **Source scanner** that extracts class strings from `.tsx/.jsx/.html/.vue/.svelte/.astro`.
- [ ] **`@beamcss/vite` plugin** — scan → compile → inject CSS, with HMR.
- [ ] **`@beamcss/postcss` plugin** — for the bundlers Vite doesn't cover.
- [ ] **CLI:** `beam init` (scaffold + config), `beam dev` (watch), `beam build` (emit).
- [ ] **Dogfood app** — a small real UI (a landing page or dashboard) built entirely in Beam, kept in the repo as `examples/`.

**Exit criterion:** you can `beam init` a fresh Vite app, write Beam in components, and see correct styles with hot reload.

---

## Phase 3 — Hardening & review

**Goal:** very few defects, no known criticals, defensible performance claims.

- [ ] **Test suite:** output snapshots, grammar edge cases, per-framework integration tests.
- [ ] **Benchmark harness** vs Tailwind (build time, output size). Public, reproducible — this is also marketing and AIO citation bait.
- [ ] **Adversarial multi-agent review** (Claude Code subagents): security, performance, API/DX, and a red-teamer whose job is to break the others' findings. A finding isn't accepted until another agent fails to refute it. Referee with the fuzz corpus, snapshots, and benchmarks.
- [ ] **Supply-chain lockdown:** npm **provenance** (OIDC from Actions), 2FA on all maintainers + org, granular short-lived publish tokens, committed lockfiles, minimal deps.
- [ ] **Browser-support matrix** + transpile targets documented (cascade layers, `@scope`, container queries, `color-mix`).

**Exit criterion:** green test + fuzz + bench in CI; security review produces no open criticals; provenance is live on releases.

---

## Phase 4 — Docs & the AI surface

**Goal:** discoverable by humans and agents; trivial to adopt.

- [ ] **beamcss.dev:** one-screen hero (tagline + before/after snippet + email capture), getting-started, full syntax reference. Semantic HTML, structured data, fast and crawlable.
- [ ] **`llms.txt` + `llms-full.txt`** — the thing Tailwind declined. One-fetch, complete, machine-readable docs.
- [ ] **`@beamcss/mcp`** — agent server exposing tokens, primitives, and component scaffolding live.
- [ ] **Tailwind → Beam codemod** — maps atoms and folds repeated prefixes into groups. Single biggest adoption lever.
- [ ] **Editor extension / LSP** — hover-to-resolved-CSS, autocomplete from your tokens. Docs that live in the editor.

**Exit criterion:** a developer (or their agent) can go from zero to styled UI using only the site + autocomplete; an agent can write valid Beam from the MCP server alone.

---

## Phase 5 — Launch & growth

**Goal:** zero to scale. (Full marketing playbook to be built separately — flagged for later.)

- [ ] Stand up the **coming-soon / waitlist** page (the social teasers point here).
- [ ] **0.x public release** + Show HN / dev-community launch once the dogfood app and benchmarks are solid.
- [ ] Execute the **growth plan** (positioning, content/AIO engine, first true believers, growth loops).
- [ ] **Activate the sustainability model** — turn on the chosen revenue path before scale, not after.

**Exit criterion:** real external projects using Beam; a funding mechanism live.

---

## Start here (first week)

1. Phase 0: set the **MIT license**, restructure to the **pnpm monorepo**, add `SECURITY.md` + CI.
2. Commit `beam-css-spec.md` into `/docs` as the source of truth.
3. Phase 1, Slice 1: the **walking skeleton** — config → a few utilities → atomic CSS. Prove the pipeline before building the grammar.
4. Wire **cargo-fuzz** into the parser crate from the first commit, so hardening is a habit, not a phase.

---

## Open decisions to settle (from spec §10)

1. Spacing scale: index array vs named keys (leaning array).
2. Primitive base specificity: `beam.base` vs `beam.utilities` (recommend base so atoms win).
3. Group delimiter `()` escaping scheme — confirm it's bundler-safe.
4. Dynamic `(--var)` patterns under SSR/streaming — document per framework.

---

*Beam CSS — focused styles, zero scatter. · beamcss.dev*