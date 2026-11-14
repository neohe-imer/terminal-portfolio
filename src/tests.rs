use super::*;
use ratatui::{backend::TestBackend, buffer::Buffer};

fn press(app: &mut App, code: KeyCode) {
    app.handle_key(KeyEvent::new(code, KeyModifiers::NONE));
}

fn draw(app: &mut App, width: u16, height: u16) -> Buffer {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|frame| app.render(frame)).unwrap();
    terminal.backend().buffer().clone()
}

fn text(buffer: &Buffer) -> String {
    (0..buffer.area.height)
        .map(|y| {
            (0..buffer.area.width)
                .map(|x| buffer.get(x, y).symbol())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn navigation_preserves_selection_and_stays_in_bounds() {
    let mut app = App::default();
    press(&mut app, KeyCode::Down);
    assert_eq!(app.link_state.selected(), Some(0));
    press(&mut app, KeyCode::Tab);
    assert_eq!(app.tab, Tab::Links);
    for _ in 0..10 {
        press(&mut app, KeyCode::Char('j'));
    }
    assert_eq!(app.link_state.selected(), Some(3));
    press(&mut app, KeyCode::BackTab);
    assert_eq!(app.tab, Tab::About);
    press(&mut app, KeyCode::Right);
    assert_eq!(app.link_state.selected(), Some(3));
    for _ in 0..10 {
        press(&mut app, KeyCode::Up);
    }
    assert_eq!(app.link_state.selected(), Some(0));
    press(&mut app, KeyCode::Left);
    assert_eq!(app.tab, Tab::About);
}

#[test]
fn ctrl_c_and_existing_exit_keys_quit() {
    for key in [
        KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
    ] {
        let mut app = App::default();
        app.handle_key(key);
        assert!(!app.running);
    }
}

#[test]
fn releases_and_modified_shortcuts_do_not_trigger_actions() {
    let mut app = App::default();
    app.handle_key(KeyEvent::new_with_kind(
        KeyCode::Char('q'),
        KeyModifiers::NONE,
        KeyEventKind::Release,
    ));
    app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::ALT));
    app.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL));
    app.handle_key(KeyEvent::new_with_kind(
        KeyCode::Char('t'),
        KeyModifiers::NONE,
        KeyEventKind::Repeat,
    ));
    assert!(app.running);
    assert_eq!(app.theme, ThemeMode::Dark);
    press(&mut app, KeyCode::Char('t'));
    assert_eq!(app.theme, ThemeMode::Light);
    press(&mut app, KeyCode::Char('t'));
    assert_eq!(app.theme, ThemeMode::Dark);
}

#[test]
fn held_navigation_keys_move_selection() {
    let mut app = App {
        tab: Tab::Links,
        ..App::default()
    };
    app.handle_key(KeyEvent::new_with_kind(
        KeyCode::Down,
        KeyModifiers::NONE,
        KeyEventKind::Repeat,
    ));
    assert_eq!(app.link_state.selected(), Some(1));
}

#[test]
fn compact_about_keeps_identity_description_and_controls_visible() {
    for theme in [ThemeMode::Dark, ThemeMode::Light] {
        for (width, height) in [(32, 10), (40, 12), (80, 24), (120, 40)] {
            let mut app = App {
                theme,
                ..App::default()
            };
            let screen = text(&draw(&mut app, width, height));
            for expected in [
                "Brayan Ruiz",
                "Ratatui.",
                "[About]",
                "Links",
                "t theme",
                "quit",
            ] {
                assert!(
                    screen.contains(expected),
                    "missing {expected} at {width}x{height}:\n{screen}"
                );
            }
        }
    }
}

#[test]
fn selected_link_remains_visible_in_both_directions_and_after_resize() {
    for theme in [ThemeMode::Dark, ThemeMode::Light] {
        let mut app = App {
            tab: Tab::Links,
            theme,
            ..App::default()
        };
        for (width, height) in [(80, 24), (32, 10), (40, 12), (120, 40), (32, 10)] {
            for index in (0..app.links.len()).chain((0..app.links.len()).rev()) {
                app.link_state.select(Some(index));
                let buffer = draw(&mut app, width, height);
                let screen = text(&buffer);
                assert!(screen.contains(app.links[index].label), "{screen}");
                assert!(screen.contains(app.links[index].value), "{screen}");
                assert!(screen.contains(&format!("{}/4", index + 1)), "{screen}");
                assert!(screen.contains("↑↓ / j k select"), "{screen}");
                let marker = buffer
                    .content
                    .iter()
                    .find(|cell| cell.symbol() == "›")
                    .unwrap();
                assert_eq!(marker.fg, Palette::from(theme).accent);
                assert!(marker.modifier.contains(Modifier::BOLD));
            }
        }
    }
}

#[test]
fn empty_and_stale_selection_are_safe() {
    let mut app = App {
        tab: Tab::Links,
        ..App::default()
    };
    app.link_state.select(Some(usize::MAX));
    assert!(text(&draw(&mut app, 32, 10)).contains("coming soon"));
    assert_eq!(app.link_state.selected(), Some(3));
    app.links.clear();
    press(&mut app, KeyCode::Down);
    press(&mut app, KeyCode::Up);
    assert_eq!(app.link_state.selected(), None);
    assert!(text(&draw(&mut app, 32, 10)).contains("No links available."));
}

#[test]
fn undersized_terminal_has_resize_guidance_and_recovers() {
    let mut app = App::default();
    let screen = text(&draw(&mut app, 31, 9));
    assert!(screen.contains("Resize to at least 32 × 10"));
    assert!(screen.contains("Ctrl+C to quit"));
    assert!(text(&draw(&mut app, 80, 24)).contains("Brayan Ruiz"));
}

#[test]
fn rendering_handles_zero_tiny_and_breakpoint_sizes() {
    let mut app = App::default();
    for theme in [ThemeMode::Dark, ThemeMode::Light] {
        app.theme = theme;
        for tab in [Tab::About, Tab::Links] {
            app.tab = tab;
            for width in [0, 1, 2, 20, 31, 32, 49, 50, 55, 56, 77, 78, 80, 100, 160] {
                for height in [0, 1, 2, 9, 10, 14, 15, 16, 19, 20, 24, 30, 60] {
                    draw(&mut app, width, height);
                }
            }
        }
    }
}

#[test]
fn panel_respects_nonzero_origin_and_large_window_bounds() {
    assert_eq!(
        window_area(Rect::new(10, 20, 80, 24)),
        Rect::new(10, 20, 80, 24)
    );
    assert_eq!(
        window_area(Rect::new(10, 20, 160, 60)),
        Rect::new(40, 35, 100, 30)
    );
}
