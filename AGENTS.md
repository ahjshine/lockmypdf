# Agent Instructions — LockMyPDF Web

This workspace contains only the WebAssembly version of LockMyPDF (`apps/lockmypdf-web`) and its supporting Rust crates under `crates/`.

## Key Commands

- Check WASM compilation: `cargo check -p lockmypdf-web --target wasm32-unknown-unknown`
- Lint WASM target: `cargo clippy -p lockmypdf-web --target wasm32-unknown-unknown -- -D warnings`
- Run unit tests (core crates): `cargo test --workspace --exclude lockmypdf-web`
- Build web bundle: `cd apps/lockmypdf-web && trunk build --release`
