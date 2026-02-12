{
  lib,
  symlinkJoin,
  makeWrapper,
  microsoft-edge,
  adboost,
}:

symlinkJoin {
  name = "microsoft-edge-adboost";
  paths = [ microsoft-edge ];
  nativeBuildInputs = [ makeWrapper ];
  postBuild = ''
    wrapProgram $out/bin/microsoft-edge \
      --add-flags "--load-extension=${adboost}/share/adboost"
  '';

  meta = {
    description = "Microsoft Edge with AdBoost extension pre-loaded";
    mainProgram = "microsoft-edge";
  };
}
