use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui_bubbletea_theme::BubbleTheme;

/// A named spinner frame set.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct SpinnerFrames {
    name: &'static str,
    frames: &'static [&'static str],
}

impl SpinnerFrames {
    /// Braille dot spinner, close to the classic Charm default.
    pub const DOTS: Self = Self {
        name: "dots",
        frames: &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
    };

    /// Alias for the classic mini-dot spinner.
    pub const MINIDOT: Self = Self {
        name: "minidot",
        frames: Self::DOTS.frames,
    };

    /// Simple ASCII line spinner for conservative terminals.
    pub const LINE: Self = Self {
        name: "line",
        frames: &["-", "\\", "|", "/"],
    };

    /// Braille jump spinner.
    pub const JUMP: Self = Self {
        name: "jump",
        frames: &["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
    };

    /// Block pulse spinner.
    pub const PULSE: Self = Self {
        name: "pulse",
        frames: &["█", "▓", "▒", "░"],
    };

    /// Bouncing points spinner.
    pub const POINTS: Self = Self {
        name: "points",
        frames: &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
    };

    /// Progress meter spinner.
    pub const METER: Self = Self {
        name: "meter",
        frames: &["▱▱▱", "▰▱▱", "▰▰▱", "▰▰▰", "▰▰▱", "▰▱▱"],
    };

    /// Hamburger menu spinner.
    pub const HAMBURGER: Self = Self {
        name: "hamburger",
        frames: &["☱", "☲", "☴", "☲"],
    };

    /// Ellipsis spinner.
    pub const ELLIPSIS: Self = Self {
        name: "ellipsis",
        frames: &["", ".", "..", "..."],
    };

    /// Globe spinning.
    pub const GLOBE: Self = Self {
        name: "globe",
        frames: &["🌍", "🌎", "🌏"],
    };

    /// Moon phases.
    pub const MOON: Self = Self {
        name: "moon",
        frames: &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
    };

    /// See-no-evil monkey.
    pub const MONKEY: Self = Self {
        name: "monkey",
        frames: &["🙈", "🙉", "🙊"],
    };

    /// Arc spinner.
    pub const ARC: Self = Self {
        name: "arc",
        frames: &["◜", "◠", "◝", "◞", "◡", "◟"],
    };

    /// Thick braille dot spinner.
    pub const DOTS_THICK: Self = Self {
        name: "dotsThick",
        frames: &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
    };

    /// Asterisk spinner (previously STAR).
    pub const ASTERISK: Self = Self {
        name: "asterisk",
        frames: &["·", "✻", "✽", "✶", "✳", "✢"],
    };

    /// Wandering dot spinner (dotOrbit).
    pub const DOT_ORBIT: Self = Self {
        name: "dotOrbit",
        frames: &[
            "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁",
            "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘",
            "⢃⠨", "⡃⢐", "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩",
            "⠈⢙", "⠈⡙", "⠈⠩", "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀",
        ],
    };

    /// Clock spinner.
    pub const CLOCK: Self = Self {
        name: "clock",
        frames: &[
            "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ",
        ],
    };

    /// Box trace spinner (boxTrace).
    pub const BOX_TRACE: Self = Self {
        name: "boxTrace",
        frames: &[
            "⠉⠉", "⠈⠙", "⠀⠹", "⠀⢸", "⠀⣰", "⢀⣠", "⣀⣀", "⣄⡀", "⣆⠀", "⡇⠀", "⠏⠀", "⠋⠁",
        ],
    };

    /// Dots circle spinner.
    pub const DOTS_CIRCLE: Self = Self {
        name: "dotsCircle",
        frames: &["⢎ ", "⠎⠁", "⠊⠑", "⠈⠱", " ⡱", "⢀⡰", "⢄⡠", "⢆⡀"],
    };

    /// Sand spinner.
    pub const SAND: Self = Self {
        name: "sand",
        frames: &[
            "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦", "⣮",
            "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉", "⠑", "⠡",
            "⢁",
        ],
    };

    /// Star spinner.
    pub const STAR: Self = Self {
        name: "star",
        frames: &["✶", "✸", "✹", "✺", "✹", "✷"],
    };

    /// Circle spinner.
    pub const CIRCLE: Self = Self {
        name: "circle",
        frames: &["◡", "⊙", "◠"],
    };

    /// Square corners spinner.
    pub const SQUARE_CORNERS: Self = Self {
        name: "squareCorners",
        frames: &["◰", "◳", "◲", "◱"],
    };

    /// Creates a custom frame set.
    #[must_use]
    pub const fn new(name: &'static str, frames: &'static [&'static str]) -> Self {
        Self { name, frames }
    }

    /// Returns the frame set name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Returns all frames.
    #[must_use]
    pub const fn frames(self) -> &'static [&'static str] {
        self.frames
    }

    /// Returns whether the frame set has no frames.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.frames.is_empty()
    }

    fn frame(self, index: usize) -> &'static str {
        if self.frames.is_empty() {
            ""
        } else {
            self.frames[index % self.frames.len()]
        }
    }
}

impl Default for SpinnerFrames {
    fn default() -> Self {
        Self::DOTS
    }
}

/// Mutable state for a spinner.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct SpinnerState {
    frame_index: usize,
}

impl SpinnerState {
    /// Creates a spinner state at the first frame.
    #[must_use]
    pub const fn new() -> Self {
        Self { frame_index: 0 }
    }

    /// Returns the current frame index.
    #[must_use]
    pub const fn frame_index(self) -> usize {
        self.frame_index
    }

    /// Advances to the next frame.
    pub const fn tick(&mut self) {
        self.frame_index = self.frame_index.wrapping_add(1);
    }

    /// Resets to the first frame.
    pub const fn reset(&mut self) {
        self.frame_index = 0;
    }
}

/// A themed spinner widget.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Spinner {
    state: SpinnerState,
    frames: SpinnerFrames,
    label: String,
    theme: BubbleTheme,
}

impl Spinner {
    /// Creates a spinner with the default frame set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: SpinnerState::new(),
            frames: SpinnerFrames::default(),
            label: String::new(),
            theme: BubbleTheme::default(),
        }
    }

    /// Sets the frame set.
    #[must_use]
    pub const fn frames(mut self, frames: SpinnerFrames) -> Self {
        self.frames = frames;
        self
    }

    /// Sets the label rendered after the spinner frame.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets the theme.
    #[must_use]
    pub const fn theme(mut self, theme: BubbleTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Returns the current state.
    #[must_use]
    pub const fn state(&self) -> SpinnerState {
        self.state
    }

    /// Returns the configured frame set.
    #[must_use]
    pub const fn frame_set(&self) -> SpinnerFrames {
        self.frames
    }

    /// Returns the current frame symbol.
    #[must_use]
    pub fn current_frame(&self) -> &'static str {
        self.frames.frame(self.state.frame_index)
    }

    /// Advances to the next frame.
    pub const fn tick(&mut self) {
        self.state.tick();
    }

    /// Resets to the first frame.
    pub const fn reset(&mut self) {
        self.state.reset();
    }

    fn line(&self) -> Line<'_> {
        if self.label.is_empty() {
            Line::from(vec![self.theme.accent(self.current_frame())])
        } else {
            Line::from(vec![
                self.theme.accent(self.current_frame()),
                self.theme.span(" "),
                self.theme.span(self.label.as_str()),
            ])
        }
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &Spinner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        buf.set_line(area.x, area.y, &self.line(), area.width);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;

    use super::{Spinner, SpinnerFrames, SpinnerState};

    #[test]
    fn state_ticks_and_resets() {
        let mut state = SpinnerState::new();

        assert_eq!(state.frame_index(), 0);
        state.tick();
        assert_eq!(state.frame_index(), 1);
        state.reset();
        assert_eq!(state.frame_index(), 0);
    }

    #[test]
    fn built_in_frame_sets_keep_compatibility_names() {
        assert_eq!(SpinnerFrames::default().name(), "dots");
        assert_eq!(SpinnerFrames::DOTS.name(), "dots");
        assert_eq!(SpinnerFrames::DOTS.frames()[0], "⠋");
        assert_eq!(
            SpinnerFrames::MINIDOT.frames(),
            SpinnerFrames::DOTS.frames()
        );
        assert_eq!(SpinnerFrames::DOTS_THICK.name(), "dotsThick");
        assert_eq!(SpinnerFrames::DOTS_THICK.frames()[0], "⣾");
    }

    #[test]
    fn spinner_cycles_through_frames() {
        let mut spinner = Spinner::new().frames(SpinnerFrames::LINE);

        assert_eq!(spinner.current_frame(), "-");
        spinner.tick();
        assert_eq!(spinner.current_frame(), "\\");
        spinner.tick();
        assert_eq!(spinner.current_frame(), "|");
        spinner.tick();
        assert_eq!(spinner.current_frame(), "/");
        spinner.tick();
        assert_eq!(spinner.current_frame(), "-");
    }

    #[test]
    fn empty_custom_frame_set_renders_empty_frame() {
        let spinner = Spinner::new().frames(SpinnerFrames::new("empty", &[]));

        assert_eq!(spinner.current_frame(), "");
        assert!(spinner.frame_set().is_empty());
    }

    #[test]
    fn spinner_renders_frame_and_label() -> Result<(), Box<dyn std::error::Error>> {
        let spinner = Spinner::new().frames(SpinnerFrames::LINE).label("Loading");
        let backend = TestBackend::new(10, 1);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|frame| {
            frame.render_widget(&spinner, Rect::new(0, 0, 10, 1));
        })?;

        let buffer = terminal.backend().buffer();
        let rendered = (0..9).map(|x| buffer[(x, 0)].symbol()).collect::<String>();
        assert_eq!(rendered, "- Loading");
        assert_eq!(buffer[(0, 0)].fg, spinner.theme.palette.accent);
        assert_eq!(buffer[(2, 0)].fg, spinner.theme.palette.foreground);

        Ok(())
    }
}
