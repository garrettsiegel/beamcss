# Beam CSS — Site Design Document

## Direction: "Bento Editorial"

Inspired by Awwwards 2024–2025 trends (bento grids, large kinetic typography, noise textures) combined with Swiss editorial grid principles. Goal: feel authored by a developer with taste, not assembled from a template.

---

## Visual Principles

| Principle | Decision | Rationale |
|---|---|---|
| Accent color | Warm amber `#e07b39` | Almost no dev tool uses orange. Distinctive on dark. |
| Hero alignment | Left-flush, NOT centered | Centering reads as AI default. Editorial sites go left. |
| Section rhythm | Monospace marks `01. /` above each heading | Gives the page a technical spec / documentation feel |
| Section dividers | Hard `border-t` only | No alternating `bg-surface` backgrounds — monotone except one intentional lift |
| Feature layout | Asymmetric CSS bento grid (2-col, card 1 spans 2 rows) | Awwwards trend; breaks the predictable equal-card grid |
| Code size | `text-base` in `<pre>` blocks | Code is the primary visual element, not a footnote |
| Card hover | Border accent only — no background fill | Hover fills feel SaaS-generic |
| Corner radius | `rounded-md` (8px) max | Sharp, intentional — no over-rounded "bubbly" cards |
| Texture | Subtle grain overlay via `body::before` SVG noise | Adds analog warmth without visible decoration |
| Nav | Three plain text links, no CTA button | Restraint signals confidence |
| Hero headline | `text-4xl` (3.75rem) — very large | Typographic confidence |

## What We Avoid (AI-Generated Red Flags)
- Centered hero
- Blue accent (the default)
- Soft card hover fills (`bg-panel` on hover)
- Gradient decorations
- Equal-size feature cards in a regular grid
- Background color alternation as the only section structure
- "Get started today" CTA centered at the bottom

---

## Stack

- **Vite + React 19 + TypeScript**
- `beamcss@^0.1.1` + `@beamcss/vite@^0.1.1` (from NPM — real published packages)
- `@vitejs/plugin-react` for JSX transform
- Beam CSS scanner picks up `className=` in `.tsx` files automatically

---

## Token Overrides (`site/beam.config.ts`)

```ts
accent: '#e07b39',           // amber — replaces blue
text['4xl']: '3.75rem',      // hero headline
radius.sm: '4px',            // sharp corners
```

Spacing scale uses named tokens so class strings are legible:
`xs=4px, sm=12px, md=20px, lg=32px, xl=56px, 2xl=96px`

---

## Component Map

```
App
├── Nav          — brand left, 3 text links right, border-b
├── Hero         — left-aligned, text-4xl headline, install command, 2 CTAs
├── Comparison   — 01. / mark, 2-col code split, no section bg
├── Features     — 02. / mark, CSS bento grid (card 1 spans 2 rows)
├── WhyBeam      — 03. / mark, numbered rows (large mono numbers left)
├── GetStarted   — 04. / mark, bg-surface lift, left-border accent on steps
└── Footer       — no border, 1 line, very understated
```

---

## Grain Texture (`src/global.css`)

```css
body::before {
  content: '';
  position: fixed;
  inset: 0;
  background-image: url("data:image/svg+xml,..."); /* SVG fractalNoise */
  opacity: 0.025;
  pointer-events: none;
  z-index: 9999;
}
```

---

## Known Beam CSS 0.1.1 Constraints

- `grid-cols-N` not supported → use `style={{ display: 'grid', gridTemplateColumns: '...' }}`
- `mx-auto` needs `mx-[auto]` (arbitrary value) or inline style
- Numeric step values are 1px/step (`gap-4` = 4px) → always use named spacing tokens
- Scanner picks up `className=` in `.tsx` files ✓
- Avoid literal `class="` in visible code examples → use `class=&quot;` HTML entity
