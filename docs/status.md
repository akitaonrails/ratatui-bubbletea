# Implementation status

This file records the current state so future sessions can resume after context compaction.

## Completed milestones

- Milestone 0: workspace skeleton.
- Milestone 1: Charm-inspired theme primitives and ratatui widget helpers.
- Milestone 2: key bindings, key map, and help footer.
- Milestone 3: spinner component.
- Milestone 4: progress component.
- Milestone 5: selectable list component.
- Milestone 6: themed table wrapper/helpers.
- Milestone 7: text input component.
- Milestone 8: viewport/pager component.
- Milestone 9: optional `ratatui-tea` MVP with `Model`, `Cmd`, `Program`, `ProgramHandle` channel, and ratatui draw integration.
- Milestone 10: `ratatui-tea` command helpers: `once`, `batch`, `sequence`, and one-shot `tick`.

## Release status

Latest planned release: `v0.2.0`.

Previously released: `v0.1.0`.

Published crates:

- `ratatui-bubbletea-theme = "0.2.0"`
- `ratatui-bubbletea-components = "0.2.0"`
- `ratatui-tea = "0.2.0"`

GitHub release:

- <https://github.com/akitaonrails/ratatui-bubbletea/releases/tag/v0.2.0>

The release workflow is intentionally rerunnable: already-published crates are skipped based on crates.io's `already exists` response.

## Current crate layout

```text
crates/
  ratatui-bubbletea-theme/
  ratatui-bubbletea-components/
  ratatui-tea/
```

## Example ports

Current Bubble Tea-inspired example ports are documented in [`docs/examples.md`](examples.md).

Implemented ports:

- components: `bubbletea-help`, `bubbletea-list-simple`, `bubbletea-result`, `bubbletea-progress-static`, `bubbletea-spinner`, `bubbletea-table`, `bubbletea-textinput`, `bubbletea-pager`, `bubbletea-tabs`, `bubbletea-showcase`.
- tea: `bubbletea-simple`, `bubbletea-sequence`.

These examples use ratatui rendering. Small component examples may target `TestBackend` for CI-friendly compilation. `bubbletea-spinner`, `bubbletea-showcase`, and the `ratatui-tea` examples are interactive crossterm apps that quit on `q`/Esc.

## Implemented components

`ratatui-bubbletea-theme` currently exports:

- `BubbleTheme` / `Theme`,
- `Palette::CHARM` as a const default palette,
- semantic styles for text, muted text, accent, success, warning, error, borders, focused borders, title, selected rows/items, and help text,
- block helpers: `block`, `block_with_focus`, `titled_block`, `modal_block`, `titled_modal_block`,
- text helpers: `span`, `muted`, `accent`, `success`, `warning`, `error`, `title`, `bullet`, `checked`, `crossed`, `help_line`,
- paragraph helpers: `paragraph`, `paragraph_in_block`.

`ratatui-bubbletea-components` currently exports:

- `KeyBinding`, `KeyMap`,
- `Help`, `HelpMode`,
- `Spinner`, `SpinnerFrames`, `SpinnerState`, including built-in frame sets such as `DOTS`, `DOTS_THICK`, `LINE`, `JUMP`, `PULSE`, `POINTS`, `METER`, `HAMBURGER`, `ELLIPSIS`, `GLOBE`, `MOON`, `MONKEY`, `ARC`, `ASTERISK`, `CLOCK`, `DOT_ORBIT`, `BOX_TRACE`, `DOTS_CIRCLE`, `SAND`, `STAR`, `CIRCLE`, and `SQUARE_CORNERS`,
- `Progress`, `ProgressSymbols`,
- `ListItem`, `ListState`, `SelectList`,
- `ThemedTable`,
- `TextInput`, `TextInputState`,
- `Viewport`, `ViewportState`.

## Validation command

Run this before committing changes:

```bash
cargo fmt --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## Important design constraints

- Requires `ratatui 0.30.x`; do not mix with app-level `ratatui 0.29` types.
- Prefer `ratatui::crossterm` over a separate `crossterm` dependency when using ratatui's crossterm backend/events.
- Do not make `ratatui-bubbletea-theme` or `ratatui-bubbletea-components` depend on `ratatui-tea`.
- Keep components usable from plain ratatui event loops.
- Keep `ratatui-tea` optional and adapter-like.
- Do not bypass ratatui with ANSI string rendering.
- Prefer stable ratatui APIs.

## Known follow-ups

- Decide whether `ratatui-tea::Cmd::tick` should move from blocking synchronous sleep to an async/threaded executor.
- Add real terminal lifecycle/event-loop helpers behind an optional crossterm feature if desired.
- Add more screenshots/golden render docs once visual defaults settle.
- For future releases, bump all three crate versions and keep publish order: theme → components → tea.
