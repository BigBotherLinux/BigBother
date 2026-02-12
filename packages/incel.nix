{
  lib,
  appimageTools,
  fetchurl,
}:
let
  pname = "incel";
  version = "1.1.0";

  src = fetchurl {
    url = "https://github.com/kluzzebass/incel/releases/download/v${version}/Incel-${version}.AppImage";
    hash = "sha256-nDCH1ZVOQBUo0FwWuBoRSzIrmTcJhCgt6/vCHIC/4KQ=";
  };

  appimageContents = appimageTools.extractType2 { inherit pname version src; };
in
appimageTools.wrapType2 {
  inherit pname version src;

  extraInstallCommands = ''
    install -m 444 -D ${appimageContents}/incel.desktop $out/share/applications/incel.desktop
    install -m 444 -D ${appimageContents}/incel.png $out/share/icons/hicolor/512x512/apps/incel.png
    substituteInPlace $out/share/applications/incel.desktop \
      --replace-fail 'Exec=AppRun' 'Exec=${pname}'
  '';

  meta = {
    description = "The Involuntarily Single-Celled Spreadsheet";
    homepage = "https://github.com/kluzzebass/incel";
    license = lib.licenses.mit;
    platforms = [ "x86_64-linux" ];
    mainProgram = "incel";
  };
}
