# Bubble Tea example ports

This repository includes Rust examples that recreate the parts of Charm's Bubble Tea example catalog that map to the current API surface.

The examples intentionally render through ratatui widgets. Small component examples may use `TestBackend` so they are cheap to compile and run in CI; they are source-level showcases rather than full interactive terminal demos. `bubbletea-spinner`, `bubbletea-showcase`, and the `ratatui-tea` examples use ratatui's crossterm backend and stay open until you quit.

## All-components TUI showcase

Run the interactive showcase:

```bash
cargo run -p ratatui-bubbletea-components --example bubbletea-showcase
```

Controls:

- `j`/`k` or arrows: move the selected list item.
- PageDown/Space and PageUp: scroll the viewport.
- `+`/`-`: adjust progress.
- `s`: advance spinner once.
- `t`: toggle spinner frame set.
- Type regular characters: edit the text input.
- Backspace/Delete/Left/Right: edit or move inside the input.
- `h`: toggle compact/expanded help.
- `q`, Esc, or Ctrl+C: quit.

## Ported examples

| Original Bubble Tea example | ratatui-bubbletea example | Crate | Notes |
| --- | --- | --- | --- |
| `examples/help` | `bubbletea-help` | `ratatui-bubbletea-components` | Compact and expanded key help. |
| `examples/list-simple` | `bubbletea-list-simple` | `ratatui-bubbletea-components` | Simple selected list with descriptions. |
| `examples/result` | `bubbletea-result` | `ratatui-bubbletea-components` | Choice list plus selected result panel. |
| `examples/progress-static` | `bubbletea-progress-static` | `ratatui-bubbletea-components` | Multiple static progress bars. |
| `examples/spinner` | `bubbletea-spinner` | `ratatui-bubbletea-components` | Interactive spinner gallery with all built-in frame sets. |
| `examples/table` | `bubbletea-table` | `ratatui-bubbletea-components` | Themed table with header and selected row. |
| `examples/textinput` | `bubbletea-textinput` | `ratatui-bubbletea-components` | Focused single-line input and cursor rendering. |
| `examples/pager` | `bubbletea-pager` | `ratatui-bubbletea-components` | Scrollable viewport with position indicator. |
| `examples/tabs` | `bubbletea-tabs` | `ratatui-bubbletea-components` | Themed tabs built from ratatui `Line`/`Paragraph`. |
| combined showcase | `bubbletea-showcase` | `ratatui-bubbletea-components` | Interactive TUI showing theme, key map, help, spinner, progress, list, table, text input, and viewport. |
| `examples/simple` | `bubbletea-simple` | `ratatui-tea` | Interactive `Model` / `Msg` / `Cmd` counter; `↑`/`+`, `↓`/`-`, `q`. |
| `examples/sequence` | `bubbletea-sequence` | `ratatui-tea` | Interactive ordered command demo; `r` replays, `q` quits. |

## Run examples

```bash
cargo run -p ratatui-bubbletea-components --example bubbletea-help
cargo run -p ratatui-bubbletea-components --example bubbletea-list-simple
cargo run -p ratatui-bubbletea-components --example bubbletea-result
cargo run -p ratatui-bubbletea-components --example bubbletea-progress-static
cargo run -p ratatui-bubbletea-components --example bubbletea-spinner
cargo run -p ratatui-bubbletea-components --example bubbletea-table
cargo run -p ratatui-bubbletea-components --example bubbletea-textinput
cargo run -p ratatui-bubbletea-components --example bubbletea-pager
cargo run -p ratatui-bubbletea-components --example bubbletea-tabs
cargo run -p ratatui-bubbletea-components --example bubbletea-showcase
cargo run -p ratatui-tea --example bubbletea-simple
cargo run -p ratatui-tea --example bubbletea-sequence
```

`bubbletea-spinner`, `bubbletea-showcase`, `bubbletea-simple`, and `bubbletea-sequence` enter the alternate screen and wait for keyboard input. Press `q` or Esc to quit.

## Not ported yet

Some original examples need APIs that this project does not provide yet:

- `textarea`, `chat`, `split-editors`, `dynamic-textarea`: multiline textarea component.
- `file-picker`: file picker component.
- `mouse`, `clickable`: mouse event abstractions.
- `capability`, `query-term`, `colorprofile`, `keyboard-enhancements`: terminal query/protocol commands.
- `exec`, `suspend`, `pipe`: external process/stdin lifecycle helpers.
- `glamour`: markdown renderer integration.
- `timer`, `stopwatch`, `progress-animated`: non-blocking timer/async command executor for smooth interactive updates.
