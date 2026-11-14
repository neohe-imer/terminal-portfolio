use std::io::{self, stdout, Stdout};

use crossterm::{
    cursor::Show,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// Owns the terminal modes for the complete interactive session.
pub struct TerminalSession {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    restored: bool,
}

impl TerminalSession {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let result = (|| {
            execute!(stdout(), EnterAlternateScreen)?;
            Terminal::new(CrosstermBackend::new(stdout()))
        })();
        match result {
            Ok(terminal) => Ok(Self {
                terminal,
                restored: false,
            }),
            Err(error) => {
                // Setup can fail after raw mode or the alternate screen was enabled.
                let _ = restore_terminal();
                Err(error)
            }
        }
    }

    pub fn restore(&mut self) -> io::Result<()> {
        let result = restore_terminal();
        self.restored = result.is_ok();
        result
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        if !self.restored {
            let _ = restore_terminal();
        }
    }
}

fn restore_terminal() -> io::Result<()> {
    // Attempt every cleanup step even if an earlier operation fails.
    let raw_mode = disable_raw_mode();
    let alternate_screen = execute!(stdout(), LeaveAlternateScreen);
    let cursor = execute!(stdout(), Show);
    raw_mode.and(alternate_screen).and(cursor)
}

pub fn install_panic_hook() {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Restore before color-eyre prints its diagnostic, including aborting panics.
        let _ = restore_terminal();
        previous_hook(info);
    }));
}
