# Terminal Portfolio

A terminal-native portfolio built with **Rust** and **Ratatui**.

This project renders a full interactive portfolio **directly inside the terminal** with keyboard navigation, themes, and an SSH-ready interface.

Inspired by minimal shell aesthetics.

---

## Preview

## Preview

<img src="assets/about.png" width="900"/>

<img src="assets/links.png" width="900"/>

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
