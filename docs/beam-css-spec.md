# Beam CSS — Syntax Specification (v0)

> Rust-fast, utility-first CSS that compiles to atomic CSS. Tailwind's authoring speed, without the wall of classes.

This document specifies the **author-time syntax** and how it compiles. Two ideas carry the whole DX:

1. **Variant grouping** — factor repeated prefixes out of the class string (`hover:(...)`, `tablet:(...)`), so markup reads as grouped intent instead of soup.
2. **Layout primitives** — a tiny vocabulary (`stack`, `row`, `grid`, `cluster`, `place`) that collapses the layout incantations you write a hundred times a day.

Everything you write is author-time sugar. The compiler unfolds it to plain atomic classes, dedupes them globally, tree-shakes, and emits native CSS under cascade layers. Nothing in this syntax survives to runtime as a runtime cost.

---

## 1. Design principles

- **Familiar first.** Reuse Tailwind's atomic vocabulary (`p-4`, `gap-2`, `round-md`) so switching cost is near zero. The new surface is the *grouping* and the *primitives*, not a relearned dictionary.
- **The string should read.** A long Beam class list reads as a handful of grouped clauses, not 18 space-separated atoms.
- **Tokens are numbers or names.** A spacing step is a number (`gap-4`), a semantic value is a name (`bg-surface`). Nothing else.
- **No overloaded prefixes.** Color and size never share a prefix (see §4).
- **Atomic underneath.** Author sugar; ship atoms.

---

## 2. Tokens (`beam.config.ts`)

The single source of truth. Emits a typed object (for tooling) and CSS custom properties (for the cascade).

```ts
import { defineConfig } from 'beamcss'

export default defineConfig({
  tokens: {
    // optional named spacing tokens; numeric spacing like `gap-4` is 4px
    space: { card: '1rem', section: '2rem' },
    color: {
      base: '#0b0b0c', surface: '#16161a', fg: '#e8e8ea',
      muted: '#6b7280', accent: '#3b82f6', 'on-accent': '#ffffff',
    },
    radius: { sm: '4px', md: '8px', lg: '16px', full: '9999px' },
    text:   { sm: '14px', base: '16px', lg: '20px', xl: '28px' },
    font:   { ui: 'Inter, system-ui, sans-serif', mono: 'ui-monospace, monospace' },
    screens:{
      tablet: '48rem',
      desktop: '64rem',
      wide: '80rem',
      'mobile-landscape': '(max-width:47.999rem) and (orientation:landscape)',
    },
  },
})
```

Compiles to:

```css
@layer beam.tokens {
  :root {
    --space-card: 1rem; --space-section: 2rem;
    --color-surface: #16161a; --color-accent: #3b82f6;
    --radius-md: 8px; --text-lg: 20px;
  }
}
```

Theming = override variables under a selector. No per-utility `dark:` smear required:

```ts
themes: { dark: { color: { base: '#0b0b0c', fg: '#e8e8ea' } } }
// -> @layer beam.tokens { [data-theme="dark"] { --color-base: #0b0b0c } }
```

---

## 3. Compilation model

The Rust engine scans source for class strings and runs four passes:

1. **Parse** the class string against the grammar in §8.
2. **Unfold** groups and primitives into a flat list of `(variant-chain, atom)` pairs.
3. **Emit** one atomic CSS rule per unique pair, under cascade layers, deduped globally and tree-shaken to only what's used.
4. **Transpile + minify** for the target browsers (nesting, `color-mix`, logical props, prefixing).

Output layer order is fixed for deterministic specificity — you never fight `!important`:

```css
@layer beam.reset, beam.tokens, beam.base, beam.utilities;
```

Every atom is one class / one declaration, reused everywhere. `gap-4` emits `.gap-4{gap:4px}` exactly once no matter how many elements or groups reference it.

---

## 4. Atomic utilities

Form: `base` or `base-value`, where numeric spacing values are pixels, named values resolve through tokens, and arbitrary values use brackets (§7).

| Concern | Prefix | Example | Compiles to |
|---|---|---|---|
| Padding / margin | `p m` (+ `t r b l x y`) | `px-4 mt-2` | `padding-inline:4px` |
| Gap | `gap` (+ `gap-x gap-y`) | `gap-4` | `gap:4px` |
| Size | `w h` (+ `min- max-`) | `w-full h-screen` | `width:100%` |
| **Text color** | `fg` | `fg-muted` | `color:var(--color-muted)` |
| Background | `bg` | `bg-surface` | `background:var(--color-surface)` |
| Border color | `bd` | `bd-accent` | `border-color:var(--color-accent)` |
| **Font size** | `text` | `text-lg` | `font-size:var(--text-lg)` |
| Font weight | `font` | `font-medium` | `font-weight:500` |
| Radius | `round` | `round-md` | `border-radius:var(--radius-md)` |
| Border width | `border` | `border border-2` | `border-width:1px` |

> **Deliberate Beam fix:** color and size never share a prefix. `fg-*` is text *color*, `text-*` is font *size*, `bd-*` is border *color*, `border-*` is border *width*. Tailwind overloads `text-` for both color and size; Beam doesn't, so the prefix tells you the property unambiguously.

Display/position atoms exist (`block hidden absolute relative fixed sticky`), but layout is usually expressed with primitives (§6) instead.

---

## 5. Variant grouping — the signature feature

A **variant** is a condition prefix: state (`hover focus active disabled`), group/peer (`group-hover peer-checked`), structural pseudo (`first last odd even`), media (`dark motion-safe print`), responsive (`sm md lg xl`), or an arbitrary selector (`[&>svg]`).

**Standard form** (Tailwind-compatible, one atom per prefix):

```
hover:bg-accent  hover:fg-on-accent  hover:scale-105
```

**Grouped form** (Beam) — factor the prefix out once:

```
hover:(bg-accent fg-on-accent scale-105)
```

Both compile to identical atoms. The group is read-time sugar that unfolds to `hover:bg-accent`, `hover:fg-on-accent`, `hover:scale-105`.

**Stacking** variants — all conditions must hold; read outer→inner:

```
tablet:dark:(bg-base fg-muted)    ->  at >=tablet AND dark: those atoms
```

**Nesting** groups — a group may contain further variants/groups:

```
tablet:(p-6 round-lg hover:(bg-surface scale-[1.02]))
```

Unfolds to: `tablet:p-6`, `tablet:round-lg`, `tablet:hover:bg-surface`, `tablet:hover:scale-[1.02]`.

This removes the single biggest source of class soup: the repeated `hover:…hover:…hover:` / `tablet:…tablet:…` prefix sprawl.

---

## 6. Layout primitives

A primitive is a class that sets a layout *intent*. It takes modifiers in parens (same grouping syntax as §5) for its common axes. Primitives are classes, not components — they sit in `class=""` alongside everything else.

| Primitive | Sets | Default modifiers |
|---|---|---|
| `stack` | `display:flex; flex-direction:column` | `gap-0`, items stretch |
| `row` | `display:flex; flex-direction:row` | `gap-0`, items stretch |
| `cluster` | `display:flex; flex-wrap:wrap; align-items:center` | `gap-0` |
| `grid` | `display:grid` | one column |
| `place` | `display:grid; place-items:center` | centers its child on both axes |

### Modifier vocabulary (inside a primitive's parens)

- **Gap:** `gap-N` (also `gap-x-N`, `gap-y-N`)
- **Cross-axis align** (`align-items`): bare `center`, or `align-start | align-end | align-stretch | align-baseline`
- **Main-axis distribute** (`justify-content`): `between | around | evenly`, or `justify-start | justify-center | justify-end`
- **Grid:** `cols-N` → `repeat(N, 1fr)`; `rows-N`; arbitrary `cols-[200px_1fr]`
- **Wrap:** `wrap | nowrap`

> **Disambiguation rule:** bare `center` *inside* `stack()/row()/cluster()` means `align-items:center` (the cross axis — the thing you reach for constantly). To center a single child on *both* axes, use the `place` primitive. `center` therefore never means two things on the same element.

### Examples

```html
<!-- vertical list, 1rem gaps -->
<div class="stack(gap-4)">…</div>

<!-- icon + label, vertically centered, small gap -->
<div class="row(center gap-2)">…</div>

<!-- nav bar: pushed apart, vertically centered -->
<header class="row(between center) p-4">…</header>

<!-- wrapping chip group -->
<div class="cluster(gap-2)">…</div>

<!-- responsive grid: 1 col on mobile, 3 from tablet -->
<div class="grid(cols-1 tablet:cols-3 gap-6)">…</div>

<!-- perfectly centered hero -->
<section class="place h-screen">…</section>
```

Primitives compose freely with atoms, grouping, and responsive variants on the same element:

```html
<article class="stack(gap-4) p-6 bg-surface round-lg tablet:row(between center)">
```

This is the everyday shape of Beam markup: one or two primitives carrying the layout, a few atoms for spacing/color, the occasional grouped variant. Compare:

```
Tailwind:
<div class="flex flex-col items-center gap-4 p-6 rounded-lg bg-zinc-900 hover:bg-zinc-800 md:flex-row md:items-center md:justify-between">

Beam:
<div class="stack(center gap-4) p-6 round-lg bg-surface hover:bg-base tablet:row(between center)">
```

---

## 7. Values: static, arbitrary, and dynamic

**Literal / token name** (the common case): `gap-4` is `4px`; `gap-section` and `bg-surface` read tokens.

**Static arbitrary** (escape hatch, compiled once): square brackets.

```
w-[347px]   round-[10px]   bg-[oklch(72%_0.14_240)]   grid(cols-[200px_1fr])
```

**Dynamic / runtime** — the thing Tailwind fumbles. A utility can read a CSS custom property via `(--name)`; you set the property at runtime. The class is stable and atomic regardless of the value, so there's no safelist and no class-string concatenation.

```jsx
// width follows a runtime percentage
<div className="w-(--w) bg-accent" style={{ '--w': `${pct}%` }} />
// compiles to: .w-\(--w\) { width: var(--w) }
```

Ergonomic helper for setting several at once:

```jsx
import { vars } from 'beamcss'

<div className="w-(--w) h-(--h)" style={vars({ w: `${pct}%`, h: rowHeight })} />
// vars({w, h}) -> { '--w': '...', '--h': '...' }
```

Truly dynamic, zero runtime framework cost, one atomic class. This is the headline answer to "Tailwind can't do runtime values cleanly."

---

## 8. The class-string grammar

The mini-language the compiler parses (EBNF-ish). This is the contract to implement against.

```ebnf
classlist   = token { WS token } ;
token       = primitive | group | utility ;

primitive   = name "(" classlist ")" ;          (* stack(center gap-4)        *)
group       = variant-chain "(" classlist ")" ;  (* tablet:hover:(bg-accent fg-base)*)
utility     = [ variant-chain ] base ;           (* tablet:p-6, p-6            *)

variant-chain = variant { ":" variant } ":" ;    (* tablet:dark:hover:         *)
variant     = ident | arbitrary-selector ;       (* hover, tablet, [&>svg]     *)

base        = ident [ "-" value ]                 (* p-4, bg-surface, w-full    *)
            | ident "-(" cssvar ")" ;             (* w-(--w)  -> dynamic        *)
value       = step | name | "[" raw "]" ;         (* 4 | surface | [347px]      *)
cssvar      = "--" ident ;
```

**Unfold rule:** `group` and `primitive` distribute their `variant-chain` (and, for primitives, their `display`/`direction` base) across every contained `utility`, recursively, until the result is a flat list of `(variant-chain, base)` atoms. Each unique atom emits one CSS rule.

**Worked unfold:**

```
tablet:(p-6 hover:(bg-surface round-lg))
```
→
```
tablet:p-6
tablet:hover:bg-surface
tablet:hover:round-lg
```

```
row(between center gap-2)
```
→ (row sets display:flex;flex-direction:row on the element, then:)
```
justify-between   ->  justify-content:space-between
items-center      ->  align-items:center
gap-2             ->  gap:2px
```

---

## 9. Out of scope for v0 (but designed-for)

- **Tailwind codemod.** A migration that maps Tailwind atoms → Beam atoms and folds repeated prefixes into groups. Critical adoption lever; ships v0.2.
- **LSP / editor extension.** Hover a class to see resolved CSS; autocomplete from your tokens. This is the "docs live in the editor" play — no docs-site dependency.
- **Recipe / variants API** for component-level variant sets (button tones/sizes).
- **`@beam` directives** for composing named component classes from atoms when you genuinely want a semantic class.

---

## 10. Open decisions to settle before building

1. **Primitive base specificity** — do primitives live in `beam.base` or `beam.utilities`? Affects whether an atom can override a primitive default on the same element. Recommend `beam.base` so atoms always win.
2. **Group delimiter** — parens `()` (proposed) vs. brackets. Parens read best but must be escaped in generated class names; confirm the escaping scheme is bundler-safe.
3. **Arbitrary value separator inside grid** — underscore (`cols-[200px_1fr]`) follows Tailwind; keep for familiarity.
4. **How dynamic `(--w)` interacts with SSR/streaming** — inline `style` is fine everywhere; document the pattern per framework.

---

*Beam CSS — focused styles, zero scatter.*
