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
    mkdir -p $out/share/bb-flake
    cd ${../.}
    find . -name "*.nix" -type f ! -path "*/target/*" ! -path "*/.git/*" ! -path "*/result/*" -exec sh -c '
      rel_path="$1"
      dest_dir="$out/share/bb-flake/$(dirname "$rel_path")"
      mkdir -p "$dest_dir"
      cp "$rel_path" "$dest_dir/"
    ' _ {} \;

    # Copy flake.lock if it exists
    if [ -f flake.lock ]; then
      cp flake.lock $out/share/bb-flake/
    fi

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
      --set BB_FLAKE_PATH $out/share/bb-flake
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
