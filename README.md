# shhhed

Most dark themes fall into one of two traps: either everything is the same brightness and nothing guides your eye (flat), or every token is a different saturated color fighting for attention (noisy). After hours of staring at either, your eyes are cooked.

shhh is a dark theme for [Zed](https://zed.dev) that solves this. It uses five distinct visual depth planes — comments and punctuation genuinely recede, semantic colors float at equal perceptual brightness, and your code's structure emerges naturally without anything screaming at you. The result is a theme you can stare at for 8+ hours and still feel fine.

Built with AI. Fine-tuned for Dart, looks great everywhere (maybe).

![shhh-preview](preview.png)

## Palette

| Token | Color | Hex |
|-------|-------|-----|
| Functions & methods | ![#729fbc](https://placehold.co/16x16/729fbc/729fbc) | `#729fbc` |
| Function definitions | ![#72a47c](https://placehold.co/16x16/72a47c/72a47c) | `#72a47c` |
| Types & tags | ![#72a6a3](https://placehold.co/16x16/72a6a3/72a6a3) | `#72a6a3` |
| Keywords | ![#a089bc](https://placehold.co/16x16/a089bc/a089bc) | `#a089bc` |
| Strings | ![#b3946a](https://placehold.co/16x16/b3946a/b3946a) | `#b3946a` |
| Numbers & booleans | ![#c58186](https://placehold.co/16x16/c58186/c58186) | `#c58186` |
| Variables | ![#b3b3bc](https://placehold.co/16x16/b3b3bc/b3b3bc) | `#b3b3bc` |
| Params & props | ![#919199](https://placehold.co/16x16/919199/919199) | `#919199` |
| Background | ![#202024](https://placehold.co/16x16/202024/202024) | `#202024` |

## Design principles

**Five visual planes, not a flat surface.** Every token lives on a specific depth layer with clear brightness gaps between them:

| Plane | Color | L* | Role |
|-------|-------|----|------|
| Background | ![#202024](https://placehold.co/16x16/202024/202024) | 24 | Canvas |
| Comments | ![#5f5f69](https://placehold.co/16x16/5f5f69/5f5f69) | 49 | Recede |
| Operators | ![#778191](https://placehold.co/16x16/778191/778191) | 60 | Structural |
| Accents | ![#729fbc](https://placehold.co/16x16/729fbc/729fbc) ![#72a47c](https://placehold.co/16x16/72a47c/72a47c) ![#a089bc](https://placehold.co/16x16/a089bc/a089bc) ![#b3946a](https://placehold.co/16x16/b3946a/b3946a) ![#c58186](https://placehold.co/16x16/c58186/c58186) ![#72a6a3](https://placehold.co/16x16/72a6a3/72a6a3) | 67–69 | Semantic |
| Variables | ![#b3b3bc](https://placehold.co/16x16/b3b3bc/b3b3bc) | 77 | Reading plane |

**Perceptual equality across accents.** All six syntax colors sit within a 2-point OKLCH L* band (67–69%). No single token type visually dominates — differentiation comes from hue, not brightness. Your eye reads structure, not color noise.

**Night Shift resilient.** Colors are spaced to survive macOS Night Shift without collapsing into each other. Where hues converge under warm shift, chroma and lightness still differentiate.

**Retina P3 aware.** Minimum OKLCH chroma of 0.055 ensures colors register as chromatic on wide-gamut displays without looking washed out.

## Color science

The palette is informed by peer-reviewed research on visual fatigue, contrast, and readability:

- **APCA contrast** — Text targets Lc 75–90 for sustained reading on dark backgrounds, avoiding halation. WCAG 2.x overstates contrast near black; APCA (WCAG 3 draft) is perceptually accurate for dark themes.

- **Blue light and hue fatigue** — Blue wavelengths (415–455 nm) cause the most retinal scatter and accommodation fatigue. Red (hue ~0) is the most fatiguing text color on dark backgrounds. Both are used sparingly and desaturated.

- **Saturation control** — Accents stay at 15–30% HSL saturation. High saturation on dark backgrounds triggers the Helmholtz-Kohlrausch effect — perceived brightness exceeding actual luminance — causing visual strain.

- **Perceptual lightness equalization** — All accents balanced to OKLCH L* 67–69% using the OKLCH color space, which models human vision more accurately than HSL.

- **Limited palette** — 6 distinct hues with clear semantic purpose. Beyond 5–7 colors, the brain can't use color as a rapid lookup mechanism (Tonsky).

### Sources

- [APCA Contrast Algorithm](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html) — W3C WCAG 3 working draft
- [Display Color Mode and Visual Fatigue](https://ieeexplore.ieee.org/document/9363189/) — IEEE Access, 2021
- [Effect of Text Color on Visual Fatigue](https://pmc.ncbi.nlm.nih.gov/articles/PMC11175232/) — PMC, 2024
- [Blue Light and Ocular Hazards](https://pmc.ncbi.nlm.nih.gov/articles/PMC9938358/) — PMC, 2023
- [Solarized Color Scheme](https://en.wikipedia.org/wiki/Solarized_(color_scheme)) — CIELab-based perceptual uniformity
- [Syntax Highlighting Done Right](https://tonsky.me/blog/syntax-highlighting/) — Tonsky, cognitive load analysis

## Install

Install via Zed extensions.

## License

MIT
