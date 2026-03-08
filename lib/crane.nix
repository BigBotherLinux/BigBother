{
  inputs,
  system,
}:
let
  pkgs = import inputs.nixpkgs {
    inherit system;
    overlays = [ (import inputs.rust-overlay) ];
  };

  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };

  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

  buildInputs = with pkgs; [
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

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  commonArgs = {
    src = craneLib.cleanCargoSource ../.;
    strictDeps = true;
    inherit buildInputs nativeBuildInputs;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
{
  inherit
    pkgs
    rustToolchain
    craneLib
    buildInputs
    nativeBuildInputs
    commonArgs
    cargoArtifacts
    ;
}
