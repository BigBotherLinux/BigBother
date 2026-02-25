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
  pname = "werd";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "kluzzebass";
    repo = "werd";
    rev = "v${version}";
    hash = "sha256-unHfyLs8Vl3siQAe4IGGV8in7btPjHCDo0Kxu4tLFho=";
  };

  nativeBuildInputs = [
    bun2nix.hook
    makeWrapper
    copyDesktopItems
  ];

  bunDeps = bun2nix.fetchBunDeps {
    bunNix = ./werd-bun.nix;
  };

  buildPhase = ''
    runHook preBuild
    bun run build
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/lib/werd
    cp -r dist $out/lib/werd/
    cp package.json $out/lib/werd/

    mkdir -p $out/bin
    makeWrapper ${electron}/bin/electron $out/bin/werd \
      --add-flags "$out/lib/werd"

    install -Dm644 resources/icon.png $out/share/icons/hicolor/512x512/apps/werd.png

    runHook postInstall
  '';

  desktopItems = [
    (makeDesktopItem {
      name = "werd";
      exec = "werd";
      icon = "werd";
      desktopName = "Werd";
      comment = "The One-Word Word Processor";
      categories = [ "Office" ];
    })
  ];

  meta = {
    description = "The One-Word Word Processor";
    homepage = "https://github.com/kluzzebass/werd";
    license = lib.licenses.mit;
    platforms = [ "x86_64-linux" ];
    mainProgram = "werd";
  };
}
