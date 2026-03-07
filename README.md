# shhhed

A dark theme for [Zed](https://zed.dev) that doesn't fight for your attention.

Most dark themes go one of two ways: everything is colorful and loud, or everything is gray and flat. shhh does neither. Types and functions get real color. Keywords like `final` and `return` shut up - they're on every line anyway. You end up reading your code instead of its syntax.

Originally developed for Dart, tested across TypeScript, Python, and Rust.

![shhh-preview](preview.png?v=2)

## Why this one

- **Neutral gray background** - pure `#1e1e1e`, zero tint. No blue cast, no warm lean. Night Shift and f.lux can shift hues all they want - on a truly neutral canvas, your accent colors still separate cleanly.
- **Keywords stay quiet** - `if`, `return`, `final` are muted purple at low brightness. They're scaffolding, not content. You already know they're there.
- **Types and functions are easy to tell apart** - teal vs blue, different hues at the same perceptual brightness. Neither one pulls your eye first; you pick them apart by color, not by one being louder.
- **Brighter = more important** - five brightness planes from comments (recede) to variables (reading plane). Punctuation disappears.
- **Won't fry your eyes** - low saturation, APCA-targeted contrast, blue and red used sparingly. Built on actual research about dark-mode visual fatigue, not vibes.

## How it works

**Five brightness planes, not a flat surface.** Every token sits at a specific depth in OKLCH lightness. Comments genuinely recede, accent colors float in the middle, and variables live brightest because that's what you're actually reading.

| Plane | OKLCH L | Examples | Role |
|-------|---------|----------|------|
| Canvas | — | Background `#1e1e1e` | Disappears |
| Recede | 0.50–0.53 | Comments `#6b6b6b`, Punctuation `#646464` | You know it's there, you don't read it |
| Structural | 0.60–0.62 | Operators `#828282`, Keywords `#877b90`, Attributes `#7a8a7a` | Scaffolding |
| Semantic | 0.67–0.71 | Types `#60b1b1`, Functions `#729bcf`, Strings `#bc8f48`, Numbers `#d37b81`, Properties `#969696` | The meaning layer |
| Reading | 0.76–0.82 | Variables `#b5b5b5`, Definitions `#8bc37b` | What you're actually reading |

**Perceptual equality across accents.** Core accent colors (strings, numbers, functions) sit at OKLCH L ≈ 0.68; types at L ≈ 0.71. Differentiation comes from hue, not brightness. Your eye reads structure, not color noise.

**Low saturation, on purpose.** High saturation on dark backgrounds triggers the Helmholtz-Kohlrausch effect - colors look brighter than they actually are, and your eyes pay for it over an 8-hour session. Accents stay under 40% HSL saturation, with structural tokens (keywords, operators) under 10%.

**Blue and red used sparingly.** Blue wavelengths (415-455 nm) cause the most retinal scatter. Red on dark backgrounds is the most fatiguing text color. Numbers are rose, functions are blue - both desaturated so they won't burn you out.

**APCA contrast, not WCAG 2.x.** WCAG 2.x overstates contrast near black. APCA (the WCAG 3 draft algorithm) is perceptually accurate for dark themes - reading-plane text targets Lc 75-90 for sustained reading without halation. Semantic accents sit at Lc 55-70, high enough for comfortable identification.

## Palette

| Token | Color | Hex |
|-------|-------|-----|
| Types | ![#60b1b1](https://placehold.co/16x16/60b1b1/60b1b1) | `#60b1b1` |
| Definitions | ![#8bc37b](https://placehold.co/16x16/8bc37b/8bc37b) | `#8bc37b` |
| Functions | ![#729bcf](https://placehold.co/16x16/729bcf/729bcf) | `#729bcf` |
| Strings | ![#bc8f48](https://placehold.co/16x16/bc8f48/bc8f48) | `#bc8f48` |
| Numbers | ![#d37b81](https://placehold.co/16x16/d37b81/d37b81) | `#d37b81` |
| Keywords | ![#877b90](https://placehold.co/16x16/877b90/877b90) | `#877b90` |
| Background | ![#1e1e1e](https://placehold.co/16x16/1e1e1e/1e1e1e) | `#1e1e1e` |

## Sources

The palette is informed by research, not eyeballing.

- [APCA Contrast Algorithm](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html) - W3C WCAG 3 working draft
- [Display Color Mode and Visual Fatigue](https://ieeexplore.ieee.org/document/9363189/) - IEEE Access, 2021
- [Effect of Text Color on Visual Fatigue](https://pmc.ncbi.nlm.nih.gov/articles/PMC11175232/) - PMC, 2024
- [Blue Light and Ocular Hazards](https://pmc.ncbi.nlm.nih.gov/articles/PMC9938358/) - PMC, 2023
- [Solarized Color Scheme](https://en.wikipedia.org/wiki/Solarized_(color_scheme)) - CIELab-based perceptual uniformity
- [Syntax Highlighting Done Right](https://tonsky.me/blog/syntax-highlighting/) - Tonsky, cognitive load analysis

## Install

Search for "shhhed" in Zed extensions.

## License

MIT
