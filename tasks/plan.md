# Implementation Plan: lockmypdf web refresh

## Overview

Rebrand the existing Rust/WASM web shell, establish a Möbius-informed design system, redesign the
Home surface, add responsive editor chrome adjustments, and verify the real browser artifact without
changing the PDF engine.

## Architecture decisions

- Keep Rust/WASM and egui; no JavaScript rewrite or server is needed for this request.
- Keep internal `lockmypdf_*` crate names and bridge message names for compatibility; change only
  user-facing web branding.
- Put visual tokens in the existing theme module and keep Home layout in the existing Home module.
- Prefer CSS for the static host page and egui layout branches for canvas UI; avoid a second UI stack.

## Task list

### Phase 1: Foundation

- [x] Rebrand the web entry point and metadata.
- [x] Establish lockmypdf design/security/performance contracts.

### Phase 2: Vertical UI slices

- [x] Update shared dark tokens and default web appearance.
- [x] Redesign the Home surface with a responsive primary open flow.
- [x] Make document chrome controls compact at narrow widths.

### Checkpoint: app surface

- [x] Focused UI tests pass.
- [x] WASM check passes.
- [x] Trunk release artifact is generated and inspected.

### Phase 3: recursive verification

- [x] Run the workspace/automation test coverage that can execute locally.
- [x] Run format, lint, layer, asset, and parity checks where available.
- [x] Perform browser smoke checks or record the browser-runtime gap.

## Verification record

- `cargo fmt --all -- --check` passed.
- `cargo check --locked -p lockmypdf-web --target wasm32-unknown-unknown` passed.
- Home unit tests passed: 5/5; Home integration filter passed: 13/13.
- `cargo test --locked -p lockmypdf-automation` passed: 316/316.
- The complete UI integration binary passed 1,331 tests; 39 renderer/font failures remain
  environment-only in this container (36 missing WGPU adapters and 3 missing Arabic/replacement
  glyphs), with no lockmypdf assertion failure.
- `cargo xtask layers` and `cargo xtask parity` passed. `cargo xtask assets` and
  `cargo xtask license-check` could not run because the supplied archive has no `.git` metadata and
  both gates require `git ls-files`; no source or asset files were changed to bypass that check.
- `trunk build --release` and `packaging/web/package.sh --skip-build` passed.
- The static shell now includes a custom `404.html`, favicon/touch-icon outputs, crawl-safe 404
  metadata, Open Graph metadata, and hosting CSP/referrer headers. The CSP was exercised through a
  local header-injecting server in Chromium; the WASM app reached ready state with no console errors,
  exceptions, or HTTP failures. Chromium emitted only the known generated-preload SRI and
  software-renderer warnings.
- Chromium headless smoke checks and screenshots passed at 320, 768, 1024, and 1440 px, and the
  static 404 page loaded without missing resources. Chrome DevTools MCP was not available, so a
  DevTools performance trace remains open.

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| egui UI is canvas-based | HTML CSS cannot style editor controls | Use existing egui tokens/layout and limit host CSS to the page shell |
| WASM build is large | Slow first visit | No new runtime assets; retain Trunk hashing, compression, and release flags |
| Rebrand breaks persistence/embeds | Existing users lose settings or host integrations | Keep storage keys and `lockmypdf:*` bridge protocol stable |
| Small viewport clips controls | Mobile unusable | Add width-aware layout branches and test 320 px explicitly |

## Task sizing

All implementation slices are intended to touch no more than five files and leave the project
compilable after each slice.
