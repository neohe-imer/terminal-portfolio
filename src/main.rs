mod terminal;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    terminal::install_panic_hook();
    let mut session = terminal::TerminalSession::new()?;
    let app_result = App::default().run(&mut session.terminal);
    let restore_result = session.restore();
    app_result?;
    restore_result?;
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
    link_state: ListState,
    running: bool,
    links: Vec<LinkItem>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tab: Tab::About,
            theme: ThemeMode::Dark,
            link_state: ListState::default().with_selected(Some(0)),
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
    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        // Releases must not trigger actions; navigation may repeat while held.
        if key.kind == KeyEventKind::Release {
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            if matches!(key.code, KeyCode::Char('c' | 'C')) {
                self.running = false;
            }
            return;
        }
        if key
            .modifiers
            .intersects(KeyModifiers::ALT | KeyModifiers::SUPER)
        {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Left | KeyCode::Char('h') => self.tab = Tab::About,
            KeyCode::Right | KeyCode::Char('l') => self.tab = Tab::Links,
            KeyCode::Tab | KeyCode::BackTab if key.kind == KeyEventKind::Press => {
                self.tab = match self.tab {
                    Tab::About => Tab::Links,
                    Tab::Links => Tab::About,
                };
            }
            KeyCode::Up | KeyCode::Char('k') if self.tab == Tab::Links => self.select_link(false),
            KeyCode::Down | KeyCode::Char('j') if self.tab == Tab::Links => self.select_link(true),
            KeyCode::Char('t') if key.kind == KeyEventKind::Press => {
                self.theme = match self.theme {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
            }
            _ => {}
        }
    }

    fn select_link(&mut self, next: bool) {
        if self.links.is_empty() {
            self.link_state.select(None);
            return;
        }
        let selected = self
            .link_state
            .selected()
            .unwrap_or(0)
            .min(self.links.len() - 1);
        self.link_state.select(Some(if next {
            selected.saturating_add(1).min(self.links.len() - 1)
        } else {
            selected.saturating_sub(1)
        }));
    }

    fn render(&mut self, frame: &mut Frame) {
        let palette = Palette::from(self.theme);
        let screen = frame.size();
        frame.render_widget(
            Block::default().style(Style::default().bg(palette.bg).fg(palette.fg)),
            screen,
        );
        if screen.width < 32 || screen.height < 10 {
            frame.render_widget(
                Paragraph::new("Resize to at least 32 × 10\nq / Esc / Ctrl+C to quit")
                    .style(Style::default().fg(palette.accent))
                    .wrap(Wrap { trim: true }),
                screen,
            );
            return;
        }

        // Fill small terminals; cap the panel on large displays without shrinking content.
        let outer = window_area(screen);
        let window = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.border))
            .style(Style::default().bg(palette.panel_bg).fg(palette.fg))
            .title(Line::styled(
                " neo / terminal portfolio ",
                Style::default().fg(palette.muted),
            ));
        let inner = window.inner(outer);
        frame.render_widget(window, outer);
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(3),
                Constraint::Length(2),
            ])
            .split(inner);
        self.render_header(frame, rows[0], palette);
        match self.tab {
            Tab::About => self.render_about(frame, rows[1], palette),
            Tab::Links => self.render_links(frame, rows[1], palette),
        }
        self.render_footer(frame, rows[2], palette);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let tabs = Line::from(vec![
            Span::styled(
                if self.tab == Tab::About {
                    " [About] "
                } else {
                    "  About  "
                },
                Style::default()
                    .fg(if self.tab == Tab::About {
                        palette.accent
                    } else {
                        palette.muted
                    })
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                if self.tab == Tab::Links {
                    " [Links] "
                } else {
                    "  Links  "
                },
                Style::default()
                    .fg(if self.tab == Tab::Links {
                        palette.accent
                    } else {
                        palette.muted
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        frame.render_widget(Paragraph::new(tabs), area);
        if area.width >= 48 {
            let theme = match self.theme {
                ThemeMode::Dark => "dark",
                ThemeMode::Light => "light",
            };
            frame.render_widget(
                Paragraph::new(format!("{theme} / v{} ", env!("CARGO_PKG_VERSION")))
                    .style(Style::default().fg(palette.muted))
                    .alignment(Alignment::Right),
                Rect {
                    x: area.x + 20,
                    width: area.width - 20,
                    ..area
                },
            );
        }
    }

    fn render_about(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let text_area = if area.width >= 76 && area.height >= 14 {
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(30), Constraint::Min(1)])
                .split(area);
            frame.render_widget(
                Paragraph::new(portrait_text())
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(palette.accent)),
                columns[0],
            );
            columns[1]
        } else {
            area
        };
        let mut bio = vec![
            Line::from(vec![
                Span::styled(
                    "Brayan Ruiz",
                    Style::default().fg(palette.fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled("  hola.", Style::default().fg(palette.accent)),
            ]),
            Line::from(""),
            Line::from("A personal terminal page built with Rust and Ratatui."),
        ];
        // Keep the identity and description visible even in a 32 × 10 terminal.
        if text_area.height >= 10 {
            bio.extend([
                Line::from(""),
                Line::styled(
                    "Explore the Links tab to find me online.",
                    Style::default().fg(palette.muted),
                ),
                Line::from(""),
                Line::styled(
                    "Use Tab or ←/→ to explore. Press t to change the theme.",
                    Style::default().fg(palette.muted),
                ),
            ]);
        }
        frame.render_widget(Paragraph::new(bio).wrap(Wrap { trim: true }), text_area);
    }

    fn render_links(&mut self, frame: &mut Frame, area: Rect, palette: Palette) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(2)])
            .split(area);
        if self.links.is_empty() {
            self.link_state.select(None);
            frame.render_widget(
                Paragraph::new("No links available.").style(Style::default().fg(palette.muted)),
                area,
            );
            return;
        }
        let selected = self
            .link_state
            .selected()
            .unwrap_or(0)
            .min(self.links.len() - 1);
        self.link_state.select(Some(selected));
        frame.render_widget(
            Paragraph::new(format!(" Links · {}/{}", selected + 1, self.links.len()))
                .style(Style::default().fg(palette.muted)),
            rows[0],
        );
        let items: Vec<_> = self
            .links
            .iter()
            .map(|link| {
                let lines = if rows[1].width >= 54 {
                    vec![
                        Line::from(format!("{:<12} {}", link.label, link.value)),
                        Line::from(""),
                    ]
                } else {
                    vec![Line::from(link.label), Line::from(link.value)]
                };
                ListItem::new(lines)
            })
            .collect();
        let list = List::new(items)
            .style(Style::default().fg(palette.fg))
            .highlight_style(
                Style::default()
                    .fg(palette.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("› ");
        frame.render_stateful_widget(list, rows[1], &mut self.link_state);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let navigation = match self.tab {
            Tab::About => "Tab / ←→ switch tabs",
            Tab::Links => "↑↓ / j k select · ←→ tabs",
        };
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(navigation),
                Line::from("t theme · q / Esc quit"),
            ])
            .style(Style::default().fg(palette.muted))
            .alignment(Alignment::Center),
            area,
        );
    }
}

fn window_area(area: Rect) -> Rect {
    let width = area.width.min(100);
    let height = area.height.min(30);
    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
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
                accent: Color::Rgb(160, 65, 8),
                border: Color::Rgb(175, 170, 160),
            },
        }
    }
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

#[cfg(test)]
mod tests;
