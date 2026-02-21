{
  lib,
  rustPlatform,
  pkg-config,
  dbus,
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = "bb-nag";
  version = "0.1.0";

  src = ../bb-nag;

  cargoLock = {
    lockFile = ../bb-nag/Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    dbus
  ];

  meta = with lib; {
    description = "BigBother Nag - Humorously annoying desktop notifications";
    homepage = "https://github.com/BigBotherLinux/BigBother";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "bb-nag";
  };
}
