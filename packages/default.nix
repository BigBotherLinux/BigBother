{
  pkgs,
  bun2nix,
  craneLib ? null,
  commonArgs ? null,
  cargoArtifacts ? null,
  cranePkgs ? null,
}:
let
  adboost = pkgs.callPackage ./adboost.nix { };

  bb-installer-crane =
    let
      cp = cranePkgs;
    in
    craneLib.buildPackage (
      commonArgs
      // {
        inherit cargoArtifacts;
        cargoExtraArgs = "--package bb-installer";

        postInstall = ''
          # Copy all .nix files from the repo for installation
          mkdir -p $out/share/bb-flake
          cd ${./..}
          find . -name "*.nix" -type f ! -path "*/target/*" ! -path "*/.git/*" -exec sh -c '
            mkdir -p "$out/share/bb-flake/$(dirname "$1")"
            cp "$1" "$out/share/bb-flake/$1"
          ' _ {} \;

          # Copy flake.lock if it exists
          if [ -f flake.lock ]; then
            cp flake.lock $out/share/bb-flake/
          fi

          wrapProgram $out/bin/bb-installer \
            --prefix PATH : ${
              cp.lib.makeBinPath (
                with cp;
                [
                  parted
                  util-linux
                  e2fsprogs
                  dosfstools
                  nixos-install-tools
                  mkpasswd
                ]
              )
            } \
            --prefix LD_LIBRARY_PATH : ${cp.lib.makeLibraryPath commonArgs.buildInputs} \
            --set BB_FLAKE_PATH $out/share/bb-flake
        '';
        nativeBuildInputs = commonArgs.nativeBuildInputs ++ [ cp.makeWrapper ];
      }
    );
in
{
  underpass = pkgs.callPackage ./underpass.nix { };
  gust-cursor-theme = pkgs.callPackage ./gust.nix { };
  bb-installer =
    if craneLib != null then bb-installer-crane else pkgs.callPackage ./bb-installer.nix { };
  incel = pkgs.callPackage ./incel.nix { inherit bun2nix; };
  werd = pkgs.callPackage ./werd.nix { inherit bun2nix; };
  inherit adboost;
  edge-adboost = pkgs.callPackage ./edge-adboost.nix { inherit adboost; };
  bb-bp = pkgs.callPackage ./bb-bp.nix { };
  bb-nag = pkgs.callPackage ./bb-nag.nix { };
  bb-sddm-theme = pkgs.callPackage ./bb-sddm-theme.nix { };
  bb-fastfetch = pkgs.callPackage ./bb-fastfetch.nix { };
  bb-age-refresher = pkgs.callPackage ./bb-age-refresher.nix { };
  bb-age-attestation = pkgs.callPackage ./bb-age-attestation.nix { };
}
