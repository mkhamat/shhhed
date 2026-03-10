![shhhed-hero](hero.png)

# shhhed

A quiet dark theme for [Zed](https://zed.dev). Scaffolding recedes, meaning emerges.

## Design

Five brightness tiers, assigned by how much each token matters when scanning code:

| Tier | OKLCH L | Examples | Role |
|------|---------|----------|------|
| Canvas | — | Editor ![#1e1e22](https://placehold.co/10x10/1e1e22/1e1e22) `#1e1e22`, chrome ![#1a1a1e](https://placehold.co/10x10/1a1a1e/1a1a1e) `#1a1a1e` | Background |
| Recede | 0.53–0.58 | Comments ![#6a6a6e](https://placehold.co/10x10/6a6a6e/6a6a6e) `#6a6a6e`, Punctuation ![#6c6c70](https://placehold.co/10x10/6c6c70/6c6c70) `#6c6c70` | Present but not read |
| Structural | 0.61–0.63 | Operators ![#878787](https://placehold.co/10x10/878787/878787) `#878787`, Keywords ![#8d8196](https://placehold.co/10x10/8d8196/8d8196) `#8d8196`, Attributes ![#7a8a7a](https://placehold.co/10x10/7a8a7a/7a8a7a) `#7a8a7a` | Scaffolding |
| Semantic | 0.68–0.71 | Types ![#60b1b1](https://placehold.co/10x10/60b1b1/60b1b1) `#60b1b1`, Functions ![#729bcf](https://placehold.co/10x10/729bcf/729bcf) `#729bcf`, Strings ![#bc8f48](https://placehold.co/10x10/bc8f48/bc8f48) `#bc8f48`, Numbers ![#ca8489](https://placehold.co/10x10/ca8489/ca8489) `#ca8489` | Meaning |
| Reading | 0.76–0.79 | Variables ![#b8b8bc](https://placehold.co/10x10/b8b8bc/b8b8bc) `#b8b8bc`, Definitions ![#8bc37b](https://placehold.co/10x10/8bc37b/8bc37b) `#8bc37b`, Constructors ![#97be84](https://placehold.co/10x10/97be84/97be84) `#97be84` | What you're reading |

Properties and parameters sit between Structural and Semantic (L 0.64–0.67).

- Near-neutral canvas with blue undertone. Compatible with Night Shift / f.lux.
- Palette computed in [OKLCH](https://oklch.com). Same-tier accents differ by hue, not brightness.
- Saturation under 50% HSL (most under 40%) to reduce strain on dark backgrounds.
- Structural tokens clear 4.5:1 against the canvas (WCAG AA). Recede tokens sit below 4:1 by design.

## UI

Covers the full Zed surface: git gutter, diffs, search highlights, debugger, minimap, scrollbar, terminal (16-color ANSI + bright/dim), and status colors.

![shhhed-preview](preview.png)

## Palette

| Token | Color | Hex |
|-------|-------|-----|
| Types | ![#60b1b1](https://placehold.co/16x16/60b1b1/60b1b1) | `#60b1b1` |
| Definitions | ![#8bc37b](https://placehold.co/16x16/8bc37b/8bc37b) | `#8bc37b` |
| Functions | ![#729bcf](https://placehold.co/16x16/729bcf/729bcf) | `#729bcf` |
| Strings | ![#bc8f48](https://placehold.co/16x16/bc8f48/bc8f48) | `#bc8f48` |
| Numbers | ![#ca8489](https://placehold.co/16x16/ca8489/ca8489) | `#ca8489` |
| Keywords | ![#8d8196](https://placehold.co/16x16/8d8196/8d8196) | `#8d8196` |
| Background | ![#1e1e22](https://placehold.co/16x16/1e1e22/1e1e22) | `#1e1e22` |

## Install

[shhhed](https://zed.dev/extensions/shhhed-theme) in Zed extensions.

## Further reading

- [APCA Contrast Algorithm](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html)
- [OKLCH Color Space](https://oklch.com)
- [Helmholtz–Kohlrausch effect](https://en.wikipedia.org/wiki/Helmholtz%E2%80%93Kohlrausch_effect)
- [Syntax Highlighting Done Right](https://tonsky.me/blog/syntax-highlighting/) — Tonsky

## License

MIT
