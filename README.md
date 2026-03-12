![shhhed dark](dark.png)

> Font: Iosevka SS14 (JetBrains Mono) 548 · Terminal: Iosevka Term SS10 · Icons: [FantastIcons](https://zed.dev/extensions/fantasticons-icons-theme) · Tip: `"colorize_brackets": false`

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
- Structural tokens clear 4.5:1 against the canvas (WCAG AA). Recede tokens (comments, punctuation, brackets) sit at 3.5–4.1:1 - legible but not competing for reading attention.

## Palette

| Token | Color | Hex |
|-------|-------|-----|
| Types | ![#60b1b1](https://placehold.co/16x16/60b1b1/60b1b1) | `#60b1b1` |
| Functions | ![#729bcf](https://placehold.co/16x16/729bcf/729bcf) | `#729bcf` |
| Strings | ![#bd9049](https://placehold.co/16x16/bd9049/bd9049) | `#bd9049` |
| Numbers | ![#ca8489](https://placehold.co/16x16/ca8489/ca8489) | `#ca8489` |
| Keywords | ![#918699](https://placehold.co/16x16/918699/918699) | `#918699` |
| Background | ![#1e1e22](https://placehold.co/16x16/1e1e22/1e1e22) | `#1e1e22` |

![shhhed dark ui](dark2.png)

## Light

Same five tiers, inverted. Cool canvas, gray scaffolding, vivid meaning.

The light variant inverts the lightness ladder: reading tokens are darkest, recede tokens lightest.

Clean tier separation is harder to achieve on a bright background. In the dark theme, making a token brighter and more colorful both push in the same direction for the token to stand out more. In the light theme these two work against each other: semantic tokens need to be dark to rank above structural scaffolding, but darker colors can carry less color saturation in sRGB. So the tiers sit in a tighter band with narrower gaps than the dark variant.

![shhhed light](light.png)

| Tier | OKLCH L | C | Examples | Role |
|------|---------|---|----------|------|
| Canvas | 0.97 | ~0 | Background ![#f1f6fb](https://placehold.co/10x10/f1f6fb/f1f6fb) `#f1f6fb` | Background |
| Recede | 0.58–0.60 | 0.01–0.02 | Comments ![#877f73](https://placehold.co/10x10/877f73/877f73) `#877f73`, Punctuation ![#7f7970](https://placehold.co/10x10/7f7970/7f7970) `#7f7970` | Present but not competing |
| Structural | 0.51–0.54 | 0.02–0.08 | Keywords ![#715a8b](https://placehold.co/10x10/715a8b/715a8b) `#715a8b`, Operators ![#726b5f](https://placehold.co/10x10/726b5f/726b5f) `#726b5f`, Properties ![#756d62](https://placehold.co/10x10/756d62/756d62) `#756d62` | Scaffolding |
| Semantic | 0.46–0.49 | 0.10–0.22 | Types ![#006a8a](https://placehold.co/10x10/006a8a/006a8a) `#006a8a`, Functions ![#0049d4](https://placehold.co/10x10/0049d4/0049d4) `#0049d4`, Strings ![#735b00](https://placehold.co/10x10/735b00/735b00) `#735b00`, Numbers ![#a3004e](https://placehold.co/10x10/a3004e/a3004e) `#a3004e` | Meaning |
| Reading | 0.28–0.41 | 0.01–0.20 | Variables ![#2d2821](https://placehold.co/10x10/2d2821/2d2821) `#2d2821`, Constructors ![#6a1098](https://placehold.co/10x10/6a1098/6a1098) `#6a1098` | What you're reading |

| Token | Color | Hex | OKLCH |
|-------|-------|-----|-------|
| Types | ![#006a8a](https://placehold.co/16x16/006a8a/006a8a) | `#006a8a` | L=0.49 C=0.10 H=227 |
| Functions | ![#0049d4](https://placehold.co/16x16/0049d4/0049d4) | `#0049d4` | L=0.47 C=0.22 H=262 |
| Strings | ![#735b00](https://placehold.co/16x16/735b00/735b00) | `#735b00` | L=0.48 C=0.10 H=91 |
| Numbers | ![#a3004e](https://placehold.co/16x16/a3004e/a3004e) | `#a3004e` | L=0.46 C=0.19 H=3 |
| Keywords | ![#715a8b](https://placehold.co/16x16/715a8b/715a8b) | `#715a8b` | L=0.51 C=0.08 H=305 |
| Background | ![#f1f6fb](https://placehold.co/16x16/f1f6fb/f1f6fb) | `#f1f6fb` | L=0.97 C=0.01 H=248 |

![shhhed light ui](light2.png)

## Install

[shhhed](https://zed.dev/extensions/shhhed-theme) in Zed extensions.

## Further reading

- [APCA Contrast Algorithm](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html)
- [OKLCH Color Space](https://oklch.com)
- [Helmholtz–Kohlrausch effect](https://en.wikipedia.org/wiki/Helmholtz%E2%80%93Kohlrausch_effect)
- [Syntax Highlighting Done Right](https://tonsky.me/blog/syntax-highlighting/) — Tonsky

## License

MIT
