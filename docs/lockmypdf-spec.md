# lockmypdf web specification

## Objective

Rebrand the existing lockmypdf browser shell as **lockmypdf** and give its Home surface a focused,
responsive web experience for people who need to open, edit, organize, sign, or protect a PDF
without uploading it. The Rust PDF engine and WASM delivery model remain the functional foundation.

The visual direction is informed by the supplied Möbius reference: editorial hierarchy, a visible
grid, confident typography, high contrast, compact controls, and small moments of motion. No
Möbius copy, assets, or exact layout are reused.

## Tech stack

- Rust 1.95+ with the existing Cargo workspace
- `lockmypdf-ui-egui` for the shared application UI
- `eframe`/egui compiled to `wasm32-unknown-unknown`
- Trunk for the static web build
- Existing open-source fonts and Lucide icon assets only
- No handwritten JavaScript or remote runtime dependency

## User flow

1. A visitor opens the static site and sees the lockmypdf identity, a short privacy promise, and a
   clear primary action to open a PDF.
2. The visitor can reach recommended tools without hunting through menus.
3. The visitor can use the existing browser file picker, drag-and-drop, URL loading, PDF editing,
   saving, and export flows.
4. On narrow screens, the Home surface stacks content without horizontal scrolling or clipped
   actions. The editor canvas continues to fill the viewport.
5. A visitor can understand that processing is local to the browser and that no account is needed.

## Commands

```text
# From apps/lockmypdf-web
trunk serve
trunk build --release

# From the repository root
cargo check --locked -p lockmypdf-web --target wasm32-unknown-unknown
cargo test --locked -p lockmypdf-ui-egui
cargo test --locked -p lockmypdf-automation
```

## Project structure

- `apps/lockmypdf-web/index.html` — web metadata, canvas host, and responsive page shell styles
- `apps/lockmypdf-web/src/main.rs` — WASM bootstrap and browser integration
- `crates/ui-egui/src/home.rs` — lockmypdf Home experience and responsive tool surface
- `crates/ui-egui/src/theme.rs` — shared light/dark design tokens
- `crates/ui-egui/src/chrome.rs` — responsive editor chrome
- `docs/lockmypdf-spec.md` — this contract
- `docs/lockmypdf-design.md` — visual and responsive design contract
- `docs/lockmypdf-security.md` — trust boundaries and safe defaults
- `docs/lockmypdf-performance.md` — measurement baseline and budgets

## Code style

Use the existing Rust/egui patterns: keep UI state in `LockMyPdfApp`, use the shared `Tokens` and
`theme` helpers, use existing icon assets, keep interactive elements keyboard reachable, and make
the smallest responsive branch needed for a given viewport.

```rust
let compact = ui.available_width() < 720.0;
let inset = if compact { 20.0 } else { 48.0 };
egui::Frame::NONE
    .inner_margin(egui::Margin::symmetric(inset, 28.0))
    .show(ui, |ui| {
        // Content remains in normal egui layout so it can wrap and scroll.
    });
```

## Testing strategy

- Unit tests for responsive layout helpers and existing Home tool behavior.
- Focused UI crate tests after each UI slice.
- WASM compile check for the real web target.
- Release Trunk build and static artifact inspection.
- Browser smoke checks at 320, 768, 1024, and 1440 CSS pixels when a browser runtime is available.
- Workspace automation tests for the existing headless tool catalog; no user-facing tool is marked
  complete based on a button existing alone.

## Boundaries

- **Always:** preserve local-first PDF handling; keep URLs relative; use existing assets and
  licenses; keep the canvas accessible; test invalid, empty, and narrow states; keep host bridge
  compatibility.
- **Ask first:** adding a server, analytics, authentication, cloud PDF processing, new fonts,
  remote assets, or a new runtime dependency.
- **Never:** upload documents silently, add secrets, use untrusted HTML, copy the reference site's
  assets or text, weaken the existing panic/error rules, or turn off tests to make a check green.

## Success criteria

- The visible web product is named lockmypdf in its page title, Home surface, and default document
  title.
- The web shell builds as WASM with relative asset URLs and no handwritten JavaScript.
- Home has one clear primary open action, recommended tools, privacy copy, and an empty/recent state.
- Layout remains usable at 320, 768, 1024, and 1440 pixels without horizontal overflow.
- Existing PDF open/save flows and parent-frame bridge remain intact.
- Focused tests and the real WASM check pass; any unavailable browser or full-workspace check is
  reported as a gap rather than claimed as passed.
