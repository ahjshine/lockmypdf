# lockmypdf performance notes

## Baseline

The existing release site is a Rust/WASM application whose primary payload is the PDF engine. The
baseline WASM check is the existing `trunk build --release` path; measure the generated `.wasm` and
`.js` artifacts after the first successful build rather than guessing from source size.

## Budgets

- Keep the HTML/CSS shell under 50 KiB before compression.
- Add no remote fonts, images, analytics, or JavaScript runtime.
- Keep the release build's existing `wasm-opt = "z"`, LTO, and content-hashed assets.
- Avoid new shadows, filters, or continuously running animation on the Home surface.
- Preserve relative URLs and immutable caching for hashed assets.

## Verification

The 2026-11-03 release build produced these content-hashed artifacts:

| Artifact | Bytes |
|---|---:|
| `index.html` | 3,712 |
| `lockmypdf-web-982583eeeb3821a4_bg.wasm` | 47,499,591 |
| `lockmypdf-web-6b6502855237be9.js` | 171,089 |
| packaged ZIP | 17,611,391 |

The HTML shell remains below the 50 KiB budget and uses relative URLs. A Chromium headless smoke
run loaded the real WASM app and captured screenshots at 320, 768, 1024, and 1440 px widths. Chrome
reported only the known Trunk preload-integrity warning and software-WebGL/ReadPixels diagnostics;
no page error, WASM exception, or HTTP failure was observed. A DevTools MCP performance trace was
unavailable in this environment, so LCP, INP, and CLS are not recorded here.
