{
  stdenv,
  pkgs,
  fetchFromGitHub,
  lib,
  ...
}:
stdenv.mkDerivation {
  pname = "gust-cursor-theme";
  version = "1.0.0";
  src = fetchFromGitHub {
    owner = "BigBotherLinux";
    repo = "bigbother-theme";
    rev = "598ebaa6b3f13c23b66aa829b70bdbe21b569921";
    hash = "sha256-72GZSSgpGI7WyLKktzSbEm1FK+BG2iCn6aVp3wJxvDI=";
  };
  nativeBuildInputs = with pkgs; [
    inkscape
    xorg.xcursorgen
    bash
  ];

  buildPhase = ''
    # Inkscape will fail writing to the home directory with a permission denied error.. This is just to suppress that error
    export HOME=/tmp
    cd cursors/Gust

    bash ./build.sh
    mkdir -p $out/share/icons/
    cp -r Gust $out/share/icons/
  '';
}
