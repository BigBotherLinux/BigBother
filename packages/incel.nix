{
  lib,
  stdenv,
  bun2nix,
  fetchFromGitHub,
  electron,
  makeWrapper,
  makeDesktopItem,
  copyDesktopItems,
}:
stdenv.mkDerivation rec {
  pname = "incel";
  version = "1.1.0";

  src = fetchFromGitHub {
    owner = "kluzzebass";
    repo = "incel";
    rev = "v${version}";
    hash = "sha256-8rSN5XNydSm7Tf5vB8vz9nfmgtcF+sM0qOvFxuwPsJU=";
  };

  nativeBuildInputs = [
    bun2nix.hook
    makeWrapper
    copyDesktopItems
  ];

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./incel-bun.nix;
  };

  # electron-vite build compiles main/preload/renderer
  buildPhase = ''
    runHook preBuild
    bun run build
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/lib/incel
    cp -r dist $out/lib/incel/
    cp -r node_modules $out/lib/incel/
    cp package.json $out/lib/incel/

    mkdir -p $out/bin
    makeWrapper ${electron}/bin/electron $out/bin/incel \
      --add-flags "$out/lib/incel"

    install -Dm644 resources/icon.png $out/share/icons/hicolor/512x512/apps/incel.png

    runHook postInstall
  '';

  desktopItems = [
    (makeDesktopItem {
      name = "incel";
      exec = "incel";
      icon = "incel";
      desktopName = "Incel";
      comment = "The Involuntarily Single-Celled Spreadsheet";
      categories = [ "Office" ];
    })
  ];

  meta = {
    description = "The Involuntarily Single-Celled Spreadsheet";
    homepage = "https://github.com/kluzzebass/incel";
    license = lib.licenses.mit;
    platforms = [ "x86_64-linux" ];
    mainProgram = "incel";
  };
}
