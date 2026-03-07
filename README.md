<p align="center">
  <img src="theguy.png" alt="shhhed" width="400">
</p>

# shhhed

A dark theme for [Zed](https://zed.dev) that doesn't fight for your attention.

Most dark themes are either colorful and loud, or gray and flat. shhhed gives real color to types and functions while keywords stay quiet — they're on every line anyway. You end up reading your code instead of its syntax.

Originally developed for Dart, tested across TypeScript, Python, and Rust.

![shhhed-preview](preview.png)

## Design

Colors are assigned to five brightness levels based on how much they matter to reading:

| Plane | OKLCH L | Examples | Role |
|-------|---------|----------|------|
| Canvas | — | Editor `#1e1e1e`, chrome `#1a1a1a` | Disappears |
| Recede | 0.50–0.56 | Comments `#6b6b6b`, Punctuation `#646464` | You know it's there, you don't read it |
| Structural | 0.60–0.62 | Operators `#828282`, Keywords `#877b90`, Attributes `#7a8a7a` | Scaffolding |
| Semantic | 0.67–0.71 | Types `#60b1b1`, Functions `#729bcf`, Strings `#bc8f48`, Numbers `#ca8489` | The meaning layer |
| Reading | 0.76–0.82 | Variables `#b5b5b5`, Definitions `#8bc37b` | What you're actually reading |

Properties and parameters (`property`, `variable.parameter`, `variable.special`) form a gradient between Structural and Semantic (L 0.64–0.66) — they carry some meaning but aren't the primary reading target.

- **Neutral gray canvas** — editor `#1e1e1e`, chrome `#1a1a1a`, zero tint. No blue cast, no warm lean. Works cleanly with Night Shift and f.lux.
- **OKLCH-computed palette** — every accent color was computed in [OKLCH](https://oklch.com), a perceptually uniform color space. Accents at the same brightness level are told apart by hue, not by one being louder.
- **Moderate saturation** — accents stay under 50% HSL saturation (four of six under 40%). High saturation on dark backgrounds makes colors look brighter than they are and adds strain over long sessions.
- **APCA contrast targets** — contrast is based on the [APCA algorithm](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html) (WCAG 3 draft), which is perceptually accurate for dark themes. Reading-plane text targets Lc 75–90, semantic accents sit at Lc 55–70.

## UI coverage

Covers the full Zed UI surface, not just syntax highlighting.

![shhhed-terminal](preview-terminal.png)

- **Git gutter & diffs** — added/modified/deleted indicators, word-level diff highlighting, merge conflict markers (ours vs theirs)
- **Search** — passive matches are subtle, the active match stands out
- **Debugger** — active line highlight and accent color
- **Minimap & scrollbar** — three-state thumb (idle, hover, active), all neutral
- **Terminal** — full 16-color ANSI palette with bright and dim variants
- **Status colors** — green for success, amber for warnings, orange for conflicts, rose for errors

![shhhed-sidebar](preview-sidebar.png)

![shhhed-diff](preview-diff.png)

## Palette

| Token | Color | Hex |
|-------|-------|-----|
| Types | ![#60b1b1](https://placehold.co/16x16/60b1b1/60b1b1) | `#60b1b1` |
| Definitions | ![#8bc37b](https://placehold.co/16x16/8bc37b/8bc37b) | `#8bc37b` |
| Functions | ![#729bcf](https://placehold.co/16x16/729bcf/729bcf) | `#729bcf` |
| Strings | ![#bc8f48](https://placehold.co/16x16/bc8f48/bc8f48) | `#bc8f48` |
| Numbers | ![#ca8489](https://placehold.co/16x16/ca8489/ca8489) | `#ca8489` |
| Keywords | ![#877b90](https://placehold.co/16x16/877b90/877b90) | `#877b90` |
| Background | ![#1e1e1e](https://placehold.co/16x16/1e1e1e/1e1e1e) | `#1e1e1e` |

## Further reading

- [APCA Contrast Algorithm](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html) — W3C WCAG 3 working draft
- [OKLCH Color Space](https://oklch.com) — perceptually uniform lightness
- [Helmholtz–Kohlrausch effect](https://en.wikipedia.org/wiki/Helmholtz%E2%80%93Kohlrausch_effect) — why high saturation on dark backgrounds causes perceived brightness spikes
- [Display Color Mode and Visual Fatigue](https://ieeexplore.ieee.org/document/9363189/) — IEEE Access, 2021
- [Effect of Text Color on Visual Fatigue](https://pmc.ncbi.nlm.nih.gov/articles/PMC11175232/) — PMC, 2024
- [Blue Light and Ocular Hazards](https://pmc.ncbi.nlm.nih.gov/articles/PMC9938358/) — PMC, 2023
- [Solarized](https://en.wikipedia.org/wiki/Solarized_(color_scheme)) — prior art for computed palettes using CIELab
- [Syntax Highlighting Done Right](https://tonsky.me/blog/syntax-highlighting/) — Tonsky, cognitive load analysis for syntax coloring

## Install

Search for "shhhed" in Zed extensions.

## License

MIT
