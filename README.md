# Terminal Portfolio

A terminal-native portfolio built with **Rust** and **Ratatui**.

This project renders a full interactive portfolio **directly inside the terminal** with keyboard navigation, themes, and an SSH-ready interface.

Inspired by minimal shell aesthetics.

---


## Preview
<img width="1470" height="956" alt="Screenshot 2026-03-11 at 9 43 25 PM" src="https://github.com/user-attachments/assets/33ab6ccc-3b0d-4ffd-96ae-b2119d0f9f9e" />

<img width="1470" height="956" alt="Screenshot 2026-03-11 at 9 43 17 PM" src="https://github.com/user-attachments/assets/2d4b61b5-eeb9-4d82-a24d-0f5ea0a85b42" />




---

## Features

- Responsive terminal UI with dark and light themes
- Readable profile and selected link at 32 columns × 10 rows and above, with resize guidance below that size
- Portrait on wide, tall terminals; text-first layout in compact windows
- Scrollable links that keep the keyboard selection visible
- Terminal mode and cursor restoration on exit, setup errors, and panics

## Controls

| Key | Action |
|-----|--------|
| `h` / `←` | About tab |
| `l` / `→` | Links tab |
| `Tab` / `Shift+Tab` | Switch tabs |
| `j` / `↓` | Select the next link |
| `k` / `↑` | Select the previous link |
| `t` | Toggle dark/light theme |
| `q` / `Esc` / `Ctrl+C` | Quit |

Links are displayed for reference; selecting a link does not open a browser.
The resume is a placeholder until a URL is provided.

## Run locally

Install a stable Rust toolchain, then run in an interactive terminal:

```bash
git clone https://github.com/neohe-imer/terminal-portfolio
cd terminal-portfolio
cargo run --locked
```

## Development

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Tests cover keyboard input, selection visibility, both themes, and rendering at
zero-sized, compact, standard, and large terminal dimensions.

## SSH

The application can run inside an existing interactive SSH session. A public SSH
portfolio endpoint is not included; hosting one is still planned.

## License

MIT — see [LICENSE](LICENSE).
