{
  stdenv,
  python3,
  yq,
  python3Packages,
  fetchFromGitHub,
  lib,
}:

stdenv.mkDerivation {
  name = "underpass-font";

  src = fetchFromGitHub {
    owner = "BigBotherLinux";
    repo = "Underpass";
    rev = "a69d0dfdf0c912264faafc1d03053e91f3b861df";
    hash = "sha256-C3AN5QnQq1v5N5PZy0eftigdPI9fa+Gqha9hgtGF75I=";
  };
  buildInputs = [
    python3
    python3Packages.fonttools
    yq
  ];

  installPhase = ''
    mkdir -p $out/share/fonts/truetype
    find $src -type f -name "*.ttf" | while read file; do
      python3 $src/convert_font.py "$file" "$out/share/fonts/truetype"
    done
    mkdir -p $out/share/fonts/opentype
    find $src -type f -name "*.otf" | while read file; do
      python3 $src/convert_font.py "$file" "$out/share/fonts/opentype"
    done
    install -Dm644 */*/*.otf -t $out/share/fonts/opentype
    install -Dm644 */*/*.ttf -t $out/share/fonts/truetype
    runHook postInstall
  '';

  meta = {
    description = "A fork of the font Overpass, but with letters converted to lowercase versions";
    maintainers = with lib.maintainers; [ hausken ];
  };
}
