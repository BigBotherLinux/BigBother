{
  inputs,
  system,
  treefmt,
  ...
}:
let
  # Rust overlay setup for crane
  pkgsWithRust = import inputs.nixpkgs {
    inherit system;
    overlays = [ (import inputs.rust-overlay) ];
  };

  # Crane setup
  rustToolchain = pkgsWithRust.rust-bin.stable.latest.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };
  craneLib = (inputs.crane.mkLib pkgsWithRust).overrideToolchain rustToolchain;

  # Build inputs for bb-installer (GUI app)
  buildInputs = with pkgsWithRust; [
    fontconfig
    freetype
    libxkbcommon
    libGL
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libxcb
  ];

  nativeBuildInputs = with pkgsWithRust; [
    pkg-config
  ];

  # Common args for crane builds
  commonArgs = {
    src = craneLib.cleanCargoSource ./bb-installer;
    strictDeps = true;
    inherit buildInputs nativeBuildInputs;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
{
  # Crane-based checks for Rust packages
  bb-installer = craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;
    }
  );

  bb-installer-clippy = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets -- --deny warnings";
    }
  );

  bb-installer-fmt = craneLib.cargoFmt {
    src = craneLib.cleanCargoSource ./bb-installer;
  };

  # Pre-commit hooks check
  pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
    src = ./.;
    default_stages = [ "pre-commit" ];
    hooks = {
      # ========== General ==========
      check-added-large-files = {
        enable = true;
        excludes = [
          "\\.png"
          "\\.jpg"
        ];
      };
      check-case-conflicts.enable = true;
      check-executables-have-shebangs.enable = true;
      check-shebang-scripts-are-executable.enable = false; # many of the scripts in the config aren't executable because they don't need to be.
      check-merge-conflicts.enable = true;
      detect-private-keys.enable = true;
      fix-byte-order-marker.enable = true;
      mixed-line-endings.enable = true;
      trim-trailing-whitespace.enable = true;

      forbid-submodules = {
        enable = true;
        name = "forbid submodules";
        description = "forbids any submodules in the repository";
        language = "fail";
        entry = "submodules are not allowed in this repository:";
        types = [ "directory" ];
      };

      deadnix = {
        enable = true;
        excludes = [ "packages/.*-bun\\.nix$" ];
        settings = {
          noLambdaArg = true;
          exclude = [
            "packages/incel-bun.nix"
            "packages/werd-bun.nix"
          ];
        };
      };

      # ========== formatting ==========
      treefmt = {
        enable = true;
        package = treefmt;
      };

    };
  };
}
