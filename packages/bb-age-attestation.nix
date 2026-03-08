{
  lib,
  rustPlatform,
  pkg-config,
  dbus,
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = "bb-age-attestation";
  version = "0.1.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  cargoBuildFlags = [
    "--package"
    "bb-age-attestation"
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    dbus
  ];

  meta = with lib; {
    description = "BigBother Age Attestation Service - OS-level age bracket signaling via D-Bus";
    homepage = "https://github.com/BigBotherLinux/BigBother";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
    mainProgram = "bb-age-attestation";
  };
}
