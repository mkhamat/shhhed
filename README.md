![shhhed-hero](hero.png)

> Font: Iosevka SS14 (JetBrains Mono) 548 · Terminal: Iosevka Term SS10 · Tip: `"colorize_brackets": false`

# shhhed

A quiet theme for [Zed](https://zed.dev). Scaffolding recedes, meaning emerges.

## Design

Five brightness tiers, assigned by how much each token matters when scanning code:

| Tier | OKLCH L | Examples | Role |
|------|---------|----------|------|
| Canvas | — | Editor ![#1e1e22](https://placehold.co/10x10/1e1e22/1e1e22) `#1e1e22`, chrome ![#1a1a1e](https://placehold.co/10x10/1a1a1e/1a1a1e) `#1a1a1e` | Background |
| Recede | 0.56–0.60 | Comments ![#797981](https://placehold.co/10x10/797981/797981) `#797981`, Punctuation ![#74747a](https://placehold.co/10x10/74747a/74747a) `#74747a` | Present but not competing |
| Structural | 0.62–0.67 | Operators ![#929296](https://placehold.co/10x10/929296/929296) `#929296`, Keywords ![#918699](https://placehold.co/10x10/918699/918699) `#918699`, Attributes / properties / parameters / member vars ![#909094](https://placehold.co/10x10/909094/909094) `#909094` | Scaffolding |
| Semantic | 0.68–0.71 | Types ![#60b1b1](https://placehold.co/10x10/60b1b1/60b1b1) `#60b1b1`, Functions ![#729bcf](https://placehold.co/10x10/729bcf/729bcf) `#729bcf`, Strings ![#bd9049](https://placehold.co/10x10/bd9049/bd9049) `#bd9049`, Numbers ![#ca8489](https://placehold.co/10x10/ca8489/ca8489) `#ca8489` | Meaning |
| Reading | 0.76–0.79 | Variables ![#b8b8bc](https://placehold.co/10x10/b8b8bc/b8b8bc) `#b8b8bc`, Constructors ![#c6a6be](https://placehold.co/10x10/c6a6be/c6a6be) `#c6a6be`, Variants ![#c6a6be](https://placehold.co/10x10/c6a6be/c6a6be) `#c6a6be` | What you're reading |

- Near-neutral canvas with blue undertone. Compatible with Night Shift / f.lux.
- Palette computed in [OKLCH](https://oklch.com). Same-tier accents differ by hue, not brightness.
- Saturation under 50% HSL (most under 40%) to reduce strain on dark backgrounds.
- Structural tokens clear 4.5:1 against the canvas (WCAG AA). Recede tokens (comments, punctuation, brackets) sit at 3.5–4.1:1 — legible but not competing for reading attention.

## UI

Covers the full Zed surface: git gutter, diffs, search highlights, debugger, minimap, scrollbar, terminal (16-color ANSI + bright/dim), and status colors.

![shhhed-preview](preview.png)

## Palette

| Token | Color | Hex |
|-------|-------|-----|
| Types | ![#60b1b1](https://placehold.co/16x16/60b1b1/60b1b1) | `#60b1b1` |
| Functions | ![#729bcf](https://placehold.co/16x16/729bcf/729bcf) | `#729bcf` |
| Strings | ![#bd9049](https://placehold.co/16x16/bd9049/bd9049) | `#bd9049` |
| Numbers | ![#ca8489](https://placehold.co/16x16/ca8489/ca8489) | `#ca8489` |
| Keywords | ![#918699](https://placehold.co/16x16/918699/918699) | `#918699` |
| Background | ![#1e1e22](https://placehold.co/16x16/1e1e22/1e1e22) | `#1e1e22` |

## Light

Same five tiers, inverted. Cool canvas, gray scaffolding, vivid meaning.

The light variant inverts the lightness ladder — reading tokens are darkest, recede tokens lightest — and pushes maximum chroma into semantic tokens so they cut through the bright background.

| Tier | OKLCH L | C | Examples | Role |
|------|---------|---|----------|------|
| Canvas | 0.97 | ~0 | Background ![#f1f6fb](https://placehold.co/10x10/f1f6fb/f1f6fb) `#f1f6fb` | Background |
| Recede | 0.58–0.60 | 0.01–0.02 | Comments ![#877f73](https://placehold.co/10x10/877f73/877f73) `#877f73`, Punctuation ![#7f7970](https://placehold.co/10x10/7f7970/7f7970) `#7f7970` | Present but not competing |
| Structural | 0.44–0.48 | 0.02–0.10 | Keywords ![#5f447d](https://placehold.co/10x10/5f447d/5f447d) `#5f447d`, Operators ![#625a4e](https://placehold.co/10x10/625a4e/625a4e) `#625a4e`, Properties ![#645c50](https://placehold.co/10x10/645c50/645c50) `#645c50` | Scaffolding |
| Semantic | 0.49–0.60 | 0.12–0.22 | Types ![#008cb4](https://placehold.co/10x10/008cb4/008cb4) `#008cb4`, Functions ![#0050d8](https://placehold.co/10x10/0050d8/0050d8) `#0050d8`, Strings ![#9c7c00](https://placehold.co/10x10/9c7c00/9c7c00) `#9c7c00`, Numbers ![#c82868](https://placehold.co/10x10/c82868/c82868) `#c82868` | Meaning |
| Reading | 0.28–0.41 | 0.01–0.20 | Variables ![#2d2821](https://placehold.co/10x10/2d2821/2d2821) `#2d2821`, Constructors ![#6a1098](https://placehold.co/10x10/6a1098/6a1098) `#6a1098` | What you're reading |

| Token | Color | Hex | OKLCH |
|-------|-------|-----|-------|
| Types | ![#008cb4](https://placehold.co/16x16/008cb4/008cb4) | `#008cb4` | L=0.60 C=0.12 H=227 |
| Functions | ![#0050d8](https://placehold.co/16x16/0050d8/0050d8) | `#0050d8` | L=0.49 C=0.22 H=262 |
| Strings | ![#9c7c00](https://placehold.co/16x16/9c7c00/9c7c00) | `#9c7c00` | L=0.60 C=0.12 H=91 |
| Numbers | ![#c82868](https://placehold.co/16x16/c82868/c82868) | `#c82868` | L=0.55 C=0.20 H=3 |
| Keywords | ![#5f447d](https://placehold.co/16x16/5f447d/5f447d) | `#5f447d` | L=0.44 C=0.10 H=305 |
| Background | ![#f1f6fb](https://placehold.co/16x16/f1f6fb/f1f6fb) | `#f1f6fb` | L=0.97 C=0.01 H=248 |

## Install

[shhhed](https://zed.dev/extensions/shhhed-theme) in Zed extensions.

## Further reading

- [APCA Contrast Algorithm](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html)
- [OKLCH Color Space](https://oklch.com)
- [Helmholtz–Kohlrausch effect](https://en.wikipedia.org/wiki/Helmholtz%E2%80%93Kohlrausch_effect)
- [Syntax Highlighting Done Right](https://tonsky.me/blog/syntax-highlighting/) — Tonsky

## License

MIT
