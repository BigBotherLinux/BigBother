{
  lib,
  rustPlatform,
  pkg-config,
  makeWrapper,
  fontconfig,
  freetype,
  libxkbcommon,
  libGL,
  wayland,
  xorg,
  parted,
  util-linux,
  e2fsprogs,
  dosfstools,
  nixos-install-tools,
  mkpasswd,
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = "bb-installer";
  version = "0.1.0";

  src = ../bb-installer;

  cargoLock = {
    lockFile = ../bb-installer/Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];

  buildInputs = [
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

  postInstall = ''
    # Copy all .nix files from the repo for installation
    wrapProgram $out/bin/bb-installer \
      --prefix PATH : ${lib.makeBinPath [
        parted
        util-linux
        e2fsprogs
        dosfstools
        nixos-install-tools
        mkpasswd
      ]} \
      --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath [
        libGL
        libxkbcommon
        wayland
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libxcb
      ]} \
  '';

  meta = with lib; {
    description = "BigBother NixOS Installer - Your Friendly Surveillance-Themed Setup Wizard";
    homepage = "https://github.com/BigBotherLinux/BigBother";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "bb-installer";
  };
}
