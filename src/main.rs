use std::io::stdout;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = setup_terminal()?;
    let app_result = App::default().run(&mut terminal);
    restore_terminal(&mut terminal)?;
    app_result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(out);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    About,
    Links,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Clone)]
struct LinkItem {
    label: &'static str,
    value: &'static str,
}

#[derive(Debug)]
struct App {
    tab: Tab,
    theme: ThemeMode,
    selected_link: usize,
    running: bool,
    links: Vec<LinkItem>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tab: Tab::About,
            theme: ThemeMode::Dark,
            selected_link: 0,
            running: true,
            links: vec![
                LinkItem {
                    label: "GitHub",
                    value: "github.com/neohe-imer",
                },
                LinkItem {
                    label: "X / Twitter",
                    value: "x.com/brayanruizr",
                },
                LinkItem {
                    label: "Email",
                    value: "brayann.renee@gmail.com",
                },
                LinkItem {
                    label: "Resume",
                    value: "coming soon",
                },
            ],
        }
    }
}

impl App {
    fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key(key.code);
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,

            KeyCode::Left | KeyCode::Char('h') => match self.tab {
                Tab::About => {}
                Tab::Links => self.tab = Tab::About,
            },

            KeyCode::Right | KeyCode::Char('l') => match self.tab {
                Tab::About => self.tab = Tab::Links,
                Tab::Links => {}
            },

            KeyCode::Up | KeyCode::Char('k') => {
                if self.tab == Tab::Links && self.selected_link > 0 {
                    self.selected_link -= 1;
                }
            }

            KeyCode::Down | KeyCode::Char('j') => {
                if self.tab == Tab::Links && self.selected_link + 1 < self.links.len() {
                    self.selected_link += 1;
                }
            }

            KeyCode::Char('t') => {
                self.theme = match self.theme {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                }
            }

            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let palette = Palette::from(self.theme);

        frame.render_widget(Clear, frame.size());
        frame.render_widget(Block::default().style(Style::default().bg(palette.bg)), frame.size());

        let outer = centered_rect(80, 78, frame.size());

        let window = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.border))
            .style(Style::default().bg(palette.panel_bg))
            .title(Line::from(vec![
                Span::styled(" ● ", Style::default().fg(Color::Red)),
                Span::styled(" ● ", Style::default().fg(Color::Yellow)),
                Span::styled(" ● ", Style::default().fg(Color::Green)),
                Span::styled(" ssh hi.neo.dev ", Style::default().fg(palette.muted)),
            ]));

        frame.render_widget(window, outer);

        let inner = Rect {
            x: outer.x + 2,
            y: outer.y + 1,
            width: outer.width.saturating_sub(4),
            height: outer.height.saturating_sub(2),
        };

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(2),
            ])
            .split(inner);

        self.render_header(frame, rows[0], palette);
        self.render_body(frame, rows[1], palette);
        self.render_footer(frame, rows[2], palette);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(20),
                Constraint::Min(10),
                Constraint::Length(22),
            ])
            .split(area);

        let about = if self.tab == Tab::About {
            format!("[About]")
        } else {
            " About ".to_string()
        };

        let links = if self.tab == Tab::Links {
            format!("[Links]")
        } else {
            " Links ".to_string()
        };

        let left = Paragraph::new(Line::from(vec![
            Span::styled(
                about,
                Style::default().fg(if self.tab == Tab::About {
                    palette.accent
                } else {
                    palette.muted
                }),
            ),
            Span::raw("  "),
            Span::styled(
                links,
                Style::default().fg(if self.tab == Tab::Links {
                    palette.accent
                } else {
                    palette.muted
                }),
            ),
        ]));
        frame.render_widget(left, chunks[0]);

        let right = Paragraph::new(
            Line::from(vec![
                Span::styled("[", Style::default().fg(palette.accent)),
                Span::styled(" terminal portfolio ", Style::default().fg(palette.accent)),
                Span::styled("]", Style::default().fg(palette.accent)),
            ])
        )
        .alignment(Alignment::Right);
        frame.render_widget(right, chunks[2]);
    }

    fn render_body(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        match self.tab {
            Tab::About => self.render_about(frame, area, palette),
            Tab::Links => self.render_links(frame, area, palette),
        }
    }

    fn render_about(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(38),
                Constraint::Percentage(62),
            ])
            .split(area);

        let portrait = Paragraph::new(portrait_text())
            .style(Style::default().fg(palette.fg))
            .alignment(Alignment::Center);

        let portrait_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(horizontal[0]);

        frame.render_widget(portrait, portrait_box[1]);

        let bio = Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Brayan Ruiz", Style::default().fg(palette.fg).add_modifier(Modifier::BOLD)),
                Span::styled(" hola.", Style::default().fg(palette.fg)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "This demo is a Ratatui-powered personal terminal page inspired by shell aesthetics.",
                    Style::default().fg(palette.fg),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Use ",
                    Style::default().fg(palette.muted),
                ),
                Span::styled("h/l", Style::default().fg(palette.accent)),
                Span::styled(" or arrow keys to switch tabs, ", Style::default().fg(palette.muted)),
                Span::styled("t", Style::default().fg(palette.accent)),
                Span::styled(" to toggle theme, and ", Style::default().fg(palette.muted)),
                Span::styled("q", Style::default().fg(palette.accent)),
                Span::styled(" to quit.", Style::default().fg(palette.muted)),
            ]),
        ]);

        let text = Paragraph::new(bio).wrap(ratatui::widgets::Wrap { trim: true });
        let text_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(24),
                Constraint::Percentage(52),
                Constraint::Percentage(24),
            ])
            .split(horizontal[1]);

        frame.render_widget(text, text_box[1]);
    }

    fn render_links(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(10),
            ])
            .split(area);

        let intro = Paragraph::new("Selected links")
            .style(Style::default().fg(palette.accent));
        frame.render_widget(intro, rows[0]);

        let mut lines = Vec::new();

        for (idx, link) in self.links.iter().enumerate() {
            let selected = idx == self.selected_link;
            let prefix = if selected { "› " } else { "  " };

            lines.push(Line::from(vec![
                Span::styled(
                    prefix,
                    Style::default().fg(if selected { palette.accent } else { palette.muted }),
                ),
                Span::styled(
                    format!("{:<12}", link.label),
                    Style::default().fg(if selected { palette.fg } else { palette.muted }),
                ),
                Span::raw(" "),
                Span::styled(
                    link.value,
                    Style::default().fg(if selected { palette.accent } else { palette.fg }),
                ),
            ]));
            lines.push(Line::from(""));
        }

        let list = Paragraph::new(Text::from(lines)).wrap(ratatui::widgets::Wrap { trim: false });
        frame.render_widget(list, rows[1]);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let footer = Line::from(vec![
            Span::styled("v.0.1", Style::default().fg(palette.muted)),
            Span::raw("   "),
            Span::styled("←→", Style::default().fg(palette.accent)),
            Span::styled(" navigate ", Style::default().fg(palette.muted)),
            Span::styled("t", Style::default().fg(palette.accent)),
            Span::styled(" theme ", Style::default().fg(palette.muted)),
            Span::styled("q", Style::default().fg(palette.accent)),
            Span::styled(" quit", Style::default().fg(palette.muted)),
        ]);

        let paragraph = Paragraph::new(footer).alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

#[derive(Debug, Clone, Copy)]
struct Palette {
    bg: Color,
    panel_bg: Color,
    fg: Color,
    muted: Color,
    accent: Color,
    border: Color,
}

impl From<ThemeMode> for Palette {
    fn from(value: ThemeMode) -> Self {
        match value {
            ThemeMode::Dark => Self {
                bg: Color::Rgb(8, 8, 8),
                panel_bg: Color::Rgb(5, 5, 5),
                fg: Color::Rgb(230, 230, 230),
                muted: Color::Rgb(120, 120, 120),
                accent: Color::Rgb(255, 110, 20),
                border: Color::Rgb(55, 55, 55),
            },
            ThemeMode::Light => Self {
                bg: Color::Rgb(235, 232, 225),
                panel_bg: Color::Rgb(248, 245, 238),
                fg: Color::Rgb(28, 28, 28),
                muted: Color::Rgb(110, 110, 110),
                accent: Color::Rgb(210, 95, 10),
                border: Color::Rgb(175, 170, 160),
            },
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn portrait_text() -> &'static str {
    r#"
⠀⠀⠀⠀⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀
⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣦
⠀⠀⠀⣰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧
⠀⠀⠀⣿⣿⣿⣿⡿⠛⠛⠻⢿⣿⣿⣿⣿
⠀⠀⠀⣿⣿⣿⠋⠀⠀⠀⠀⠀⠙⣿⣿⣿
⠀⠀⠀⣿⣿⡇⠀⠀⢠⣶⣶⡄⠀⢸⣿⣿
⠀⠀⠀⣿⣿⣇⠀⠀⠀⠉⠉⠀⠀⣸⣿⣿
⠀⠀⠀⣿⣿⣿⣦⡀⠀⠀⠀⠀⣰⣿⣿⣿
⠀⠀⠀⠹⣿⣿⣿⣿⣶⣤⣴⣿⣿⣿⣿⠏
⠀⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⠟⠁
⠀⠀⠀⠀⠀⠀⠀⠉⠛⠛⠛⠉
"#
}
