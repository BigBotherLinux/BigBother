# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is bb-bp?

bb-bp (BigBother Boot Pre-login) is a fullscreen splash/tutorial screen shown before the display manager starts. It's part of the BigBother project — an intentionally annoying (but functional) NixOS-based Linux distribution. The app uses eframe/egui for its GUI and displays paginated content with navigation buttons. It runs fullscreen with no decorations and has a green-on-black terminal aesthetic.

## Build & Development

This is a Rust project using Cargo. The broader BigBother project uses Nix flakes with crane for Rust builds.

```bash
# Build (debug)
cargo build

# Build (release, optimized for size with LTO)
cargo build --release

# Run locally (needs display server — Wayland or X11)
cargo run

# Format code
cargo fmt

# Lint
cargo clippy -- --deny warnings
```

The nix dev shell (from the parent flake) provides all required system dependencies (libGL, wayland, X11, fontconfig, etc.) and sets `LD_LIBRARY_PATH`. Enter it with `direnv allow` or `nix develop` from the parent directory.

## Architecture

Single-file application (`src/main.rs`). The structure is straightforward:

- `SplashApp` — main app state, holds `current_page` index and a `Vec<Page>`
- `Page` — static title/content pairs
- `eframe::App::update()` — renders the current page with BACK/CONTINUE/FINISH navigation
- ESC key closes the app (debug escape hatch)

## Key Dependencies

- **eframe 0.33** with glow renderer, Wayland and X11 support
- `smithay-clipboard` and `home` are pinned to specific versions for stable-Rust compatibility

## Notes

- When nix build says files don't exist, it's likely because newly added files need `git add` first.
- The release profile is tuned for minimal binary size (`opt-level = "z"`, LTO, strip).
