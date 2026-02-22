{
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

    # Remove symlinked desktop files and create modified versions
    rm -rf $out/share/applications
    mkdir -p $out/share/applications

    # Copy and fix desktop entries to use the wrapped binary
    for desktop in ${microsoft-edge}/share/applications/*.desktop; do
      if [ -f "$desktop" ]; then
        filename=$(basename "$desktop")
        sed "s|Exec=${microsoft-edge}/bin/microsoft-edge|Exec=$out/bin/microsoft-edge|g" \
          "$desktop" > "$out/share/applications/$filename"
      fi
    done
  '';

  meta = {
    description = "Microsoft Edge with AdBoost extension pre-loaded";
    mainProgram = "microsoft-edge";
  };
}
