{
  lib,
  stdenvNoCC,
  kdePackages,
}:
stdenvNoCC.mkDerivation {
  pname = "bb-sddm-theme";
  version = "1.0.0";

  src = ./bb-sddm-theme;

  dontBuild = true;
  dontWrapQtApps = true;

  propagatedBuildInputs = with kdePackages; [
    qtsvg
  ];

  installPhase = ''
    mkdir -p $out/share/sddm/themes/bb-theme
    cp -r . $out/share/sddm/themes/bb-theme/
  '';

  meta = {
    description = "BigBother SDDM theme";
    license = lib.licenses.mit;
  };
}
