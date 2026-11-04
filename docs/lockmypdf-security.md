# lockmypdf security notes

## Trust boundaries

- User-selected files and drag-and-drop bytes are untrusted document input.
- `?file=` is a user-controlled URL and is fetched only by the browser; same-origin/CORS rules
  remain enforced by the browser.
- Parent-frame messages are accepted only from the same origin and only for the existing allowlisted
  message types.
- PDF links can request navigation, but the existing URL policy decides what can open.

## Safe defaults

- No server-side file upload, account, telemetry, or secret storage is added.
- The page remains static and can be hosted behind HTTPS with the existing cache and compression
  rules.
- Documents stay in browser memory/local persistence as already supported by the app; the branding
  change does not introduce another storage path.
- Existing file size, parser, recursion, and panic guards in the engine are unchanged.

## Review checks

- Keep `?file=` behavior and CORS failures visible to the user.
- Keep the parent bridge origin check and message validation unchanged.
- Do not use `innerHTML`, `eval`, remote scripts, or user-provided HTML in the web shell.
- Keep security headers in the hosting guidance and serve WASM with the correct MIME type.
