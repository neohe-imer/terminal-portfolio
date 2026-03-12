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

- terminal-native UI
- keyboard navigation (`h/l` or arrow keys)
- theme switching (`t`)
- ASCII portrait rendering
- minimal shell-inspired design
- ready to run over SSH

---

## Controls

| Key | Action |
|----|----|
| `h` / `←` | switch tabs |
| `l` / `→` | switch tabs |
| `t` | toggle theme |
| `q` | quit |

---

## Run Locally

```bash
git clone https://github.com/neohe-imer/terminal-portfolio
cd terminal-portfolio
cargo run

SSH Version (coming soon)

The project is designed to run over SSH so visitors can open the portfolio directly in their terminal:
