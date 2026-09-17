{
  lib,
  rustPlatform,
  pkg-config,
  makeWrapper,
  alsa-lib,
  udev,
  vulkan-loader,
  libxkbcommon,
  libGL,
  wayland,
  xorg,
  ...
}:

let
  runtimeLibs = [
    vulkan-loader
    libxkbcommon
    libGL
    wayland
    alsa-lib
    udev
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
  ];
in
rustPlatform.buildRustPackage {
  pname = "bb-installer-bevy";
  version = "0.1.0";

  # Standalone crate with its own lockfile (bevy's dep tree is kept out of the
  # root workspace).
  src = ../bb-installer-bevy;

  cargoLock = {
    lockFile = ../bb-installer-bevy/Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];

  buildInputs = runtimeLibs;

  # Bevy dlopens libvulkan/libGL at runtime, so they must be on the library path.
  # Sprites and other assets are looked up under $BEVY_ASSET_ROOT/assets.
  postInstall = ''
    mkdir -p $out/share/bb-installer-bevy
    cp -r assets $out/share/bb-installer-bevy/
    wrapProgram $out/bin/bb-installer-bevy \
      --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath runtimeLibs} \
      --set BEVY_ASSET_ROOT $out/share/bb-installer-bevy
  '';

  meta = with lib; {
    description = "BigBother NixOS Installer - Bevy edition";
    homepage = "https://github.com/BigBotherLinux/BigBother";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "bb-installer-bevy";
  };
}
