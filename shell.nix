{
  pkgs ? import <nixpkgs> { },
}:
let
  testScript = pkgs.writeShellScriptBin "testBB" ''
    if [ -f "/dev/shm/bigbother.qcow2" ]; then
      echo "Removing old image"
      rm -f /dev/shm/bigbother.qcow2
    fi
    nix build .\#nixosConfigurations.bb.config.formats.vm && ./result/run-bigbother-vm
  '';
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    testScript

    # Rust development
    cargo
    rustc
    rustfmt
    clippy

    # Build dependencies for bb-installer
    pkg-config
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

  LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";

  # Set library paths for GUI development
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.libGL
    pkgs.libxkbcommon
    pkgs.wayland
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi
    pkgs.xorg.libxcb
    pkgs.fontconfig
  ];

  # shellHook = ''
  #   # ...
  #   alias ,vb='nix build .\#nixosConfigurations.bb.config.formats.vm && ./result/run-bigbother-vm || rm /dev/shm/bigbother.qcow2'
  #
  # '';
}
