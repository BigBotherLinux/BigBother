{
  inputs,
  system,
  self,
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

  # Ensure the ISO closure contains no unfree packages (x86_64-linux only).
  # Evaluating toplevel.drvPath forces Nix to instantiate every derivation in the
  # ISO closure.  If any has an unfree license the eval fails because the ISO sets
  # allowUnfree = false.  The check itself is just `touch $out` — no ISO build.
  isoUnfreeCheck =
    if system == "x86_64-linux" then
      {
        iso-no-unfree = builtins.seq self.nixosConfigurations.bb-iso.config.system.build.toplevel.drvPath (
          pkgsWithRust.runCommand "iso-no-unfree" { } "touch $out"
        );
      }
    else
      { };
in
isoUnfreeCheck
// {
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
