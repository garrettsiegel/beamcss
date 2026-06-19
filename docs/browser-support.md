# Browser Support

This page defines Beam CSS browser-support policy as of May 22, 2026.

## Target

Beam targets modern evergreen browsers by default:

- Current and previous two stable versions of Chrome, Edge, Firefox, and Safari.
- Current iOS Safari.
- Current Chromium-based Android browsers.

Beam does not target Internet Explorer.

## CSS Feature Policy

| Feature | Beam usage | Support posture |
| --- | --- | --- |
| Cascade layers, `@layer` | Core output ordering: `beam.reset`, `beam.tokens`, `beam.base`, `beam.utilities` | Required. MDN marks `@layer` as Baseline Widely available. |
| Custom properties | Token output and dynamic `(--var)` values | Required. No fallback planned. |
| Media queries | Responsive variants such as `tablet:` and `desktop:` | Required. |
| Container queries, `@container` | Future container-aware variants | Allowed. MDN marks `@container` as Baseline Widely available. |
| `color-mix()` | Arbitrary values and examples | Allowed. MDN marks `color-mix()` as Baseline Widely available since May 2023. |
| `@scope` | Future scoped component output | Experimental. MDN marks `@scope` as Baseline 2025 and newly available since December 2025, so Beam should not emit it by default until a transpile/fallback path exists. |

## Transpile Policy

Beam's current compiler emits modern CSS directly. Before a public release that
claims legacy browser support, add a dedicated CSS transform stage for:

- `@scope` lowering or opt-in gating.
- Future CSS nesting, if Beam begins emitting nested CSS.
- Optional `color-mix()` lowering for projects with older Safari/iOS support
  requirements.

Beam should not silently polyfill layout behavior that changes semantics. If a
feature cannot be lowered faithfully, keep it opt-in and document the browser
requirement.

## Release Checklist

Before each minor release:

1. Re-check MDN compatibility for `@layer`, `@container`, `color-mix()`, and
   `@scope`.
2. Update this document if a feature changes Baseline status.
3. Run the dogfood examples in Chrome, Firefox, and Safari.
4. If Beam emits a new CSS feature, add it to the feature policy table.

## Sources

- MDN `@layer`: https://developer.mozilla.org/en-US/docs/Web/CSS/%40layer
- MDN `@container`: https://developer.mozilla.org/en-US/docs/Web/CSS/%40container
- MDN `color-mix()`: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value/color-mix
- MDN `@scope`: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40scope
