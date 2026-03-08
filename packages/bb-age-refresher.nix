{
  lib,
  rustPlatform,
  pkg-config,
  makeWrapper,
  dbus,
  fontconfig,
  freetype,
  libxkbcommon,
  libGL,
  wayland,
  xorg,
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = "bb-age-refresher";
  version = "0.1.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  cargoBuildFlags = [
    "--package"
    "bb-age-refresher"
  ];

  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];

  buildInputs = [
    dbus
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
    wrapProgram $out/bin/bb-age-refresher \
      --prefix LD_LIBRARY_PATH : ${
        lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libxcb
        ]
      }
  '';

  meta = with lib; {
    description = "BigBother Age Verification Refresher - Periodically re-confirms your age bracket";
    homepage = "https://github.com/BigBotherLinux/BigBother";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "bb-age-refresher";
  };
}
