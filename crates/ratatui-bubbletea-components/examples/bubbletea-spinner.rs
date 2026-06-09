//! Port of Bubble Tea's `examples/spinner` idea.

use std::io;
use std::time::Duration;

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::Rect;
use ratatui_bubbletea_components::{Spinner, SpinnerFrames};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;

    let mut spinners = vec![
        Spinner::new()
            .frames(SpinnerFrames::MINIDOT)
            .label("MiniDot"),
        Spinner::new().frames(SpinnerFrames::DOTS).label("Dots"),
        Spinner::new()
            .frames(SpinnerFrames::DOTS_THICK)
            .label("Thick Dots"),
        Spinner::new().frames(SpinnerFrames::LINE).label("Line"),
        Spinner::new().frames(SpinnerFrames::JUMP).label("Jump"),
        Spinner::new().frames(SpinnerFrames::PULSE).label("Pulse"),
        Spinner::new().frames(SpinnerFrames::POINTS).label("Points"),
        Spinner::new().frames(SpinnerFrames::METER).label("Meter"),
        Spinner::new()
            .frames(SpinnerFrames::HAMBURGER)
            .label("Hamburger"),
        Spinner::new()
            .frames(SpinnerFrames::ELLIPSIS)
            .label("Ellipsis"),
        Spinner::new().frames(SpinnerFrames::GLOBE).label("Globe"),
        Spinner::new().frames(SpinnerFrames::MOON).label("Moon"),
        Spinner::new().frames(SpinnerFrames::MONKEY).label("Monkey"),
        Spinner::new().frames(SpinnerFrames::ARC).label("Arc"),
        Spinner::new()
            .frames(SpinnerFrames::ASTERISK)
            .label("Asterisk"),
        Spinner::new().frames(SpinnerFrames::CLOCK).label("Clock"),
        Spinner::new()
            .frames(SpinnerFrames::DOT_ORBIT)
            .label("Dot Orbit"),
        Spinner::new()
            .frames(SpinnerFrames::BOX_TRACE)
            .label("Box Trace"),
        Spinner::new()
            .frames(SpinnerFrames::DOTS_CIRCLE)
            .label("Dots Circle"),
        Spinner::new().frames(SpinnerFrames::SAND).label("Sand"),
        Spinner::new().frames(SpinnerFrames::STAR).label("Star"),
        Spinner::new().frames(SpinnerFrames::CIRCLE).label("Circle"),
        Spinner::new()
            .frames(SpinnerFrames::SQUARE_CORNERS)
            .label("Square Corners"),
    ];

    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|frame| {
            for (y, spinner) in (0..frame.area().height).zip(spinners.iter()) {
                frame.render_widget(spinner, Rect::new(0, y, 48, 1));
            }
        })?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()?
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                break;
            }
        } else {
            for spinner in &mut spinners {
                spinner.tick();
            }
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}
