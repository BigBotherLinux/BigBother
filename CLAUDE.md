# CLAUDE.md

## Project Overview

BigBother is an intentionally annoying (but functional) NixOS-based Linux distribution. This repo is the main flake containing the OS configuration, custom packages, and NixOS modules.

## Cargo Workspace

All Rust crates live in a single Cargo workspace at the repo root. Members: `bb-installer`, `bb-bp`, `bb-age-verify`, `bb-age-attestation`, `bb-age-refresher`.

- One shared `Cargo.lock` at the repo root
- Release profile in root `Cargo.toml`: `opt-level = "z"`, LTO, strip
- Build a single crate: `cargo build -p <crate-name>`
- Check: `cargo check -p <crate-name>`
- Format: `cargo fmt`
- Lint: `cargo clippy -- --deny warnings`

## Dev Shell

Enter with `direnv allow` or `nix develop` from the repo root. Provides Rust toolchain, system libs (libGL, wayland, X11, fontconfig, dbus), and `LD_LIBRARY_PATH`.

## Nix Package Definitions (`packages/`)

Each package has its own `.nix` file in `packages/` and is registered in `packages/default.nix` via `pkgs.callPackage`.

### Workspace Rust crate pattern (used by bb-bp, bb-installer, bb-age-refresher)

```nix
rustPlatform.buildRustPackage {
  pname = "crate-name";
  src = ../.;                              # workspace root
  cargoLock.lockFile = ../Cargo.lock;      # shared workspace lock
  cargoBuildFlags = [ "--package" "crate-name" ];
  # ...
}
```

### GUI crates (eframe/egui)

Crates using eframe need these `buildInputs`: fontconfig, freetype, libxkbcommon, libGL, wayland, xorg.{libX11, libXcursor, libXrandr, libXi, libxcb}. They also need a `postInstall` wrapper setting `LD_LIBRARY_PATH` and `makeWrapper` in `nativeBuildInputs`.

### Non-workspace crates (e.g. bb-nag)

Standalone crates with their own `Cargo.lock` use `src = ../crate-dir` and `cargoLock.lockFile = ../crate-dir/Cargo.lock`.

### Adding a new package

1. Create `packages/<name>.nix`
1. Add `<name> = pkgs.callPackage ./<name>.nix { };` to `packages/default.nix`
1. `git add` the new files before running `nix build` (Nix won't see untracked files)

## NixOS Modules (`modules/`)

Each module lives in `modules/` and is imported via `modules/default.nix`. All modules follow this pattern:

```nix
{
  options.bigbother.<name> = {
    enable = mkOption { type = types.bool; default = false; description = "..."; };
  };
  config = mkIf config.bigbother.<name>.enable {
    environment.systemPackages = [ pkgs.<name> ];
    # optional: systemd.user.services, systemd.user.timers, systemd.services, etc.
  };
}
```

Enable modules in `configuration.nix` under the `bigbother` block.

### Common module patterns

- **Simple package**: just adds to `environment.systemPackages` (see `incel.nix`)
- **User service**: `systemd.user.services` with `wantedBy = [ "graphical-session.target" ]` (see `bb-nag.nix`)
- **Timer + oneshot**: `systemd.user.timers` + `systemd.user.services` with `Type = "oneshot"` (see `bb-age-refresher.nix`)
- **System service**: `systemd.services` for root-level services (see `mouse-drift.nix`, `bb-bp.nix`)

### Adding a new module

1. Create `modules/<name>.nix`
1. Add `./name.nix` to the imports list in `modules/default.nix`
1. Add `<name>.enable = true;` to the `bigbother` block in `configuration.nix`

## Build & Test

```bash
nix build .#<package-name>       # build a single package
nix flake check                  # run flake checks
nix build .#nixosConfigurations.bigbother.config.system.build.vm  # build VM
```

## Important

- When `nix build` says a file doesn't exist, `git add` it first — Nix only sees tracked files.
- `smithay-clipboard` and `home` crates are pinned in GUI crate Cargo.toml files for stable Rust compat.
