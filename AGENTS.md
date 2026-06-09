# AGENTS.md

Guidance for AI agents and contributors working in this repository.

## Project purpose

`ratatui-bubbletea` is a Rust workspace that brings Charm/Bubble Tea-inspired theme tokens, components, and optional Elm-style app-loop ergonomics to ratatui.

This project is a companion library. It must not replace ratatui's renderer or fork ratatui behavior.

## Workspace layout

- `crates/ratatui-bubbletea-theme`: semantic theme tokens and helpers for existing ratatui widgets.
- `crates/ratatui-bubbletea-components`: Bubbles-inspired ratatui widgets and widget state.
- `crates/ratatui-tea`: optional `Model` / `Msg` / `Cmd` / `Program` shell.
- `docs/`: usage guides, example mapping, implementation status, and milestone notes.
- `assets/`: README/showcase images.

## Hard invariants

1. Do not bypass ratatui with raw ANSI string rendering. Rendering should go through ratatui widgets, `Frame`, `Buffer`, `Line`, `Span`, `Style`, etc.
2. Keep the primary adoption path usable from existing ratatui apps. Theme and components must not require users to adopt `ratatui-tea`.
3. Keep dependency direction one-way:
   - `ratatui-bubbletea-theme` depends on ratatui only.
   - `ratatui-bubbletea-components` may depend on theme.
   - `ratatui-tea` may depend on theme.
   - Theme/components must not depend on `ratatui-tea`.
4. Target `ratatui 0.30.x`. Ratatui types are versioned Rust types, so avoid changes that mix or imply compatibility with `ratatui 0.29` in the same app.
5. Prefer `ratatui::crossterm` in examples and docs instead of adding a separate app-level `crossterm` dependency.
6. Public API changes must update docs/examples/status notes in the same PR when user-facing.
7. Keep release publishing order: theme → components → tea.
8. The crates.io release workflow should remain rerunnable/idempotent for already-published crates.
9. Do not modify ratatui source directly for this project.

## Validation gates

Run these before committing or recommending merge:

```bash
cargo fmt --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

If any gate fails, fix the failure before doing a full PR audit or merge recommendation.

## Examples policy

- Small component examples may use `TestBackend` when they are source-level compile/run showcases.
- Interactive examples should use ratatui's crossterm backend via `ratatui::crossterm` and must provide a clean quit path (`q`, Esc, and/or Ctrl+C where practical).
- Keep `docs/examples.md` and the README example commands in sync with new example files.
- The all-components showcase is the main visual demo and README screenshot source:

```bash
cargo run -p ratatui-bubbletea-components --example bubbletea-showcase
```

## API and design conventions

- Keep component state explicit and simple. Prefer small state structs like `SpinnerState`, `ListState`, `TextInputState`, and `ViewportState`.
- Prefer builder-style methods for widget configuration.
- Prefer stable ratatui APIs and avoid unnecessary feature coupling.
- Preserve incremental adoption: a user should be able to add only the theme crate, or only theme + components, without changing their app loop.
- When adding constants such as palettes, symbols, or spinner frame sets, use named constants rather than duplicating literals in examples.
- For fallible or lossy conversions, prefer `try_from`, safe iteration bounds, or explicit clamping over unchecked `as` casts.

## Documentation expectations

- README should stay short and direct: screenshot, crate summary, compatibility, examples, and quick usage snippets.
- `docs/usage-existing-ratatui.md` covers adoption from a plain ratatui app.
- `docs/usage-new-tea-app.md` covers the optional `ratatui-tea` path.
- `docs/examples.md` maps original Bubble Tea examples to Rust examples here and notes unsupported examples/future components.
- `docs/status.md` records implemented components, examples, release status, and follow-ups for future sessions.

## PR review expectations

- Do not merge during evaluation. Report findings and wait for explicit approval.
- If an author pushed less than four hours ago, treat the PR as potentially active unless the user explicitly asks to proceed.
- Check local CI before reviewing the diff deeply.
- Classify findings under: regressions, dead code, unnecessary duplication, magic values, clean code degradation, test coverage gaps, and doc staleness.
- This repo currently has no `CHANGELOG.md`; until one exists, require README/docs/status updates for user-facing API or behavior changes.

## Release notes

- `v0.1.0` has been published for:
  - `ratatui-bubbletea-theme`
  - `ratatui-bubbletea-components`
  - `ratatui-tea`
- Future releases should bump all relevant crate versions together unless there is a clear reason not to.
