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
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = "bb-bp";
  version = "0.1.0";

  src = ../bb-bp;

  cargoLock = {
    lockFile = ../bb-bp/Cargo.lock;
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
    wrapProgram $out/bin/bb-bp \
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
    description = "BigBother Pre-Login Splash Screen";
    homepage = "https://github.com/BigBotherLinux/BigBother";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "bb-bp";
  };
}
