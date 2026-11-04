# lockmypdf design contract

## Direction

lockmypdf is a quiet, local-first PDF workspace. The Home surface should feel editorial rather than
like a generic utility dashboard: a strong title, visible construction lines, deliberate empty
space, and a small number of high-signal actions.

## Tokens

- Background: near-black green charcoal for the default web theme
- Text: warm off-white, with muted sage for supporting copy
- Accent: electric lime for primary actions and active states
- Supporting marks: restrained cyan/orange only for diagrams or status, never as decoration on every
  control
- Radius: small and consistent; avoid pill-shaped controls except for compact status badges
- Motion: short hover/press easing; honor reduced-motion preferences through the existing app setting

## Layout rules

- Mobile first. Use 20 px outer inset below 720 px, 32 px at tablet widths, and 48 px on desktop.
- Home content stays inside a readable max width and uses normal flow so long translations wrap.
- The primary action is visible without scrolling on a typical phone viewport.
- Tool cards remain large enough for touch and keyboard use; no action is conveyed by color alone.
- The grid is decorative and must never reduce contrast or block hit testing.
- The editor remains a full-viewport canvas; responsive work on the editor is limited to its chrome,
  panels, and wrapping controls.

## Accessibility rules

- Preserve the app's AccessKit labels and widget roles.
- Every icon-only action keeps a tooltip/accessible name.
- Maintain visible keyboard focus and logical tab order.
- Keep text contrast at WCAG AA or better against its surface.
- Do not use the grid, color, or motion as the only state signal.
