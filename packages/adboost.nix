{
  lib,
  stdenv,
  fetchFromGitHub,
}:

stdenv.mkDerivation {
  pname = "adboost";
  version = "1.0";

  src = fetchFromGitHub {
    owner = "surprisetalk";
    repo = "AdBoost";
    rev = "master";
    hash = "sha256-Z8afMuOlMgLS7SJ8JAfdNcLOKXfcE9Bv730esoOzYCw=";
  };

  installPhase = ''
    runHook preInstall
    mkdir -p $out/share/adboost
    cp -r manifest.json content.js icons $out/share/adboost/
    runHook postInstall
  '';

  meta = {
    description = "The only browser extension that adds ads to web pages";
    homepage = "https://github.com/surprisetalk/AdBoost";
    license = lib.licenses.unlicense;
    platforms = lib.platforms.all;
  };
}
