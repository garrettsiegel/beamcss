# Beam CSS — Syntax Specification (v0)

> Rust-fast, utility-first CSS that compiles to atomic CSS. Tailwind's authoring speed, without the wall of classes.

This document specifies the **author-time syntax** and how it compiles. Two ideas carry the whole DX:

1. **Variant grouping** — factor repeated prefixes out of the class string (`hover:(...)`, `tablet:(...)`), so markup reads as grouped intent instead of soup.
2. **CSS-first utilities and config composition** — readable utility names (`flex`, `justify-center`, `rounded-md`) plus shortcuts, recipes, presets, and utility modules for design-system scale.

Everything you write is author-time sugar. The compiler unfolds it to plain atomic classes, dedupes them globally, tree-shakes, and emits native CSS under cascade layers. Nothing in this syntax survives to runtime as a runtime cost.

---

## 1. Design principles

- **Familiar first.** Reuse CSS vocabulary (`p-4`, `gap-2`, `rounded-md`, `justify-center`) so switching cost is low without cryptic aliases.
- **The string should read.** A long Beam class list reads as a handful of grouped clauses, not 18 space-separated atoms.
- **Tokens are numbers or names.** A spacing step is a number (`gap-4`), a semantic value is a name (`bg-surface`). Nothing else.
- **CSS-first over shorthand.** Prefer `text-muted`, `border-line`, `rounded-md`, and `align-center` over abbreviated prefixes.
- **Atomic underneath.** Author sugar; ship atoms.

---

## 2. Tokens (`beam.config.ts`)

The single source of truth. Emits a typed object (for tooling) and CSS custom properties (for the cascade).

```ts
import { defineConfig } from 'beamcss'

export default defineConfig({
  tokens: {
    // optional named spacing tokens; numeric spacing like `gap-4` is 4px
    spacing: { card: '1rem', section: '2rem' },
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
  shortcuts: {
    card: 'flex direction-column gap-4 p-card bg-surface rounded-md',
  },
  recipes: {
    button: {
      base: 'px-4 py-2 rounded-md',
      variants: {
        primary: 'bg-accent text-on-accent',
      },
    },
  },
  utilities: {
    layout: true,
    spacing: true,
    colors: true,
    typography: true,
    effects: true,
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
2. **Unfold** groups, utility groups, shortcuts, and recipes into a flat list of `(variant-chain, atom)` pairs.
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
| Text color | `text` | `text-muted` | `color:var(--color-muted)` |
| Background | `bg` | `bg-surface` | `background:var(--color-surface)` |
| Border color | `border` | `border-accent` | `border-color:var(--color-accent)` |
| Font size | `text` | `text-lg` | `font-size:var(--text-lg)` |
| Font weight | `font` | `font-medium` | `font-weight:500` |
| Radius | `rounded` | `rounded-md` | `border-radius:var(--radius-md)` |
| Border width | `border` | `border border-2` | `border-width:1px` |

> **Disambiguation rule:** `text-*` checks text-size tokens and numeric values first, then color tokens. `border-*` checks numeric widths first, then color tokens. This keeps authoring CSS-first while preserving deterministic output.

Display, flex, grid, position, overflow, border, and alignment atoms are plain utilities (`flex`, `grid`, `direction-column`, `align-center`, `justify-between`, `place-center`).

---

## 5. Variant grouping — the signature feature

A **variant** is a condition prefix: state (`hover focus active disabled`), group/peer (`group-hover peer-checked`), structural pseudo (`first last odd even`), media (`dark motion-safe print`), responsive (`sm md lg xl`), or an arbitrary selector (`[&>svg]`).

**Standard form** (Tailwind-compatible, one atom per prefix):

```
hover:bg-accent  hover:text-on-accent  hover:scale-105
```

**Grouped form** (Beam) — factor the prefix out once:

```
hover:(bg-accent text-on-accent scale-105)
```

Both compile to identical atoms. The group is read-time sugar that unfolds to `hover:bg-accent`, `hover:text-on-accent`, `hover:scale-105`.

**Stacking** variants — all conditions must hold; read outer→inner:

```
tablet:dark:(bg-base text-muted)    ->  at >=tablet AND dark: those atoms
```

**Nesting** groups — a group may contain further variants/groups:

```
tablet:(p-6 rounded-lg hover:(bg-surface scale-[1.02]))
```

Unfolds to: `tablet:p-6`, `tablet:rounded-lg`, `tablet:hover:bg-surface`, `tablet:hover:scale-[1.02]`.

This removes the single biggest source of class soup: the repeated `hover:…hover:…hover:` / `tablet:…tablet:…` prefix sprawl.

---

## 6. Layout utilities, utility grouping, and config composition

Layout is expressed with plain CSS-first utilities:

```html
<main class="grid place-center h-screen bg-base text-fg">
<section class="flex direction-column align-center gap-4 p-4 bg-surface rounded-md">
<header class="flex direction-column gap-4 tablet:(direction-row justify-between align-center)">
```

Utility grouping reduces visual noise for related declarations:

```html
<article class="padding:(16 top:24) text:(16 bold center) border:(1 solid accent)">
```

Expands as if written:

```txt
p-16 pt-24 text-16 font-bold text-center border-1 border-solid border-accent
```

Shortcuts are named class-string aliases:

```ts
shortcuts: {
  card: 'flex direction-column gap-4 p-card bg-surface rounded-md',
}
```

Usage:

```html
<article class="card hover:(bg-surface+8 scale-105)">
```

Recipes are first-class component variants in config. Beam class strings can reference the recipe base (`button`) or a recipe variant (`button:primary`):

```ts
recipes: {
  button: {
    base: 'px-4 py-2 rounded-md',
    variants: {
      primary: 'bg-accent text-on-accent',
      secondary: 'bg-surface border-line',
    },
  },
}
```

Tree-shakeable utility modules let projects disable families they do not use. Modules are enabled by default when omitted:

```ts
utilities: {
  layout: true,
  spacing: true,
  colors: true,
  typography: true,
  effects: true,
}
```

Presets are plain config fragments merged before local config, so local tokens, shortcuts, recipes, and utility flags win:

```ts
presets: [
  {
    tokens: { spacing: { section: '2rem' } },
    shortcuts: { center: 'grid place-center' },
  },
]
```

---

## 7. Values: static, arbitrary, and dynamic

**Literal / token name** (the common case): `gap-4` is `4px`; `gap-section` and `bg-surface` read tokens.

**Static arbitrary** (escape hatch, compiled once): square brackets.

```
w-[347px]   rounded-[10px]   bg-[oklch(72%_0.14_240)]   cols-[200px_1fr]
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
token       = utility-group | group | utility ;

utility-group = group-name ":(" group-items ")" ; (* padding:(16 top:24)      *)
group       = variant-chain "(" classlist ")" ;  (* tablet:hover:(bg-accent text-base)*)
utility     = [ variant-chain ] base ;           (* tablet:p-6, p-6            *)

variant-chain = variant { ":" variant } ":" ;    (* tablet:dark:hover:         *)
variant     = ident | arbitrary-selector ;       (* hover, tablet, [&>svg]     *)
group-name  = "padding" | "margin" | "text" | "border" ;
group-items = group-item { WS group-item } ;
group-item  = ident | ident ":" value ;

base        = ident [ "-" value ]                 (* p-4, bg-surface, w-full    *)
            | ident "-(" cssvar ")" ;             (* w-(--w)  -> dynamic        *)
value       = step | name | "[" raw "]" ;         (* 4 | surface | [347px]      *)
cssvar      = "--" ident ;
```

**Unfold rule:** groups distribute their `variant-chain` across every contained utility, recursively. Utility groups expand to ordinary utilities first. Shortcuts and recipes expand from config into class strings, then follow the same parse/unfold path. Each unique atom emits one CSS rule.

**Worked unfold:**

```
tablet:(p-6 hover:(bg-surface rounded-lg))
```
→
```
tablet:p-6
tablet:hover:bg-surface
tablet:hover:rounded-lg
```

```
padding:(16 top:24)
```
→
```
p-16
pt-24
```

---

## 9. Out of scope for v0 (but designed-for)

- **Tailwind codemod.** A migration that maps Tailwind atoms → Beam atoms and folds repeated prefixes into groups. Critical adoption lever; ships v0.2.
- **Full LSP / editor extension.** Hover a class to see resolved CSS; autocomplete from your tokens. This is the "docs live in the editor" play — no docs-site dependency.
- **Framework recipe helpers** for component props such as `<Button variant="primary" />`.
- **`@beam` directives** for composing named component classes from atoms when you genuinely want a semantic class.

---

## 10. Open decisions to settle before building

1. **Group delimiter** — parens `()` vs. brackets. Parens read best but must be escaped in generated class names; confirm the escaping scheme is bundler-safe.
2. **Arbitrary value separator inside grid** — underscore (`cols-[200px_1fr]`) follows Tailwind; keep for familiarity.
3. **How dynamic `(--w)` interacts with SSR/streaming** — inline `style` is fine everywhere; document the pattern per framework.

---

*Beam CSS — focused styles, zero scatter.*
