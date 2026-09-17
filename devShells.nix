{
  inputs,
  system,
  self,
}:
let
  crane = import ./lib/crane.nix { inherit inputs system; };
  inherit (crane)
    pkgs
    rustToolchain
    craneLib
    buildInputs
    ;

  scripts = import ./scripts.nix { inherit pkgs; };

  # Runtime + build libs needed by bevy/wgpu.
  bevyLibs = with pkgs; [
    alsa-lib
    udev
    vulkan-loader
    libxkbcommon
    wayland
    libGL
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
  ];
in
{
  default = craneLib.devShell {
    checks = self.checks.${system};

    packages =
      (builtins.attrValues scripts)
      ++ (with pkgs; [
        rustToolchain
        pkg-config
        cargo-watch
        cargo-edit
        qemu
        OVMF
        just
        age
      ])
      # bevy's build scripts (alsa-sys, libudev-sys) need these via pkg-config.
      # They live here as well as in the bevy shell so rust-analyzer, which runs
      # in whatever shell direnv loaded, can check bb-installer-bevy.
      ++ bevyLibs;

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ bevyLibs);
    LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";
    RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
    BB_BEVY_SHELL = "1";
  };

  # Dev shell for the bevy-based installer prototype (bb-installer-bevy).
  # Enter with: nix develop .#bevy
  bevy = pkgs.mkShell {
    packages =
      (with pkgs; [
        rustToolchain
        pkg-config
        clang
        cargo-watch
        vulkan-tools
        mesa-demos
      ])
      ++ bevyLibs;

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath bevyLibs;
    LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";
    RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

    # Marks a shell that can build bevy, so `just dev-bevy` knows it does not
    # need to re-enter one.
    BB_BEVY_SHELL = "1";
  };
}
