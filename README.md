# LockMyPDF (Web)

Browser-based PDF editor, form filler, signer, and document toolkit compiled to WebAssembly (`wasm32-unknown-unknown`).

## Project Structure

- `apps/lockmypdf-web` — WebAssembly frontend built with Trunk (`index.html`, `landing.css`, `src/main.rs`).
- `crates/` — Core Rust PDF engine, parser (`cos`), cryptography (`crypt`), rendering, editing (`edit`), forms (`forms`), annotations (`annot`), accessibility (`a11y`), and export crates.
- `assets/` — Embedded fonts (`assets/fonts`) and SVG UI icons (`assets/icons`).

## Development

### Prerequisites

```powershell
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Run Locally

```powershell
cd apps/lockmypdf-web
trunk serve --open
```

### Build for Production

```powershell
cd apps/lockmypdf-web
trunk build --release
```
