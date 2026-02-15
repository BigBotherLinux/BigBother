{pkgs, bun2nix}:
let
  adboost = pkgs.callPackage ./adboost.nix { };
in
{
  underpass = pkgs.callPackage ./underpass.nix { };
  gust-cursor-theme = pkgs.callPackage ./gust.nix { };
  bb-installer = pkgs.callPackage ./bb-installer.nix { };
  incel = pkgs.callPackage ./incel.nix { inherit bun2nix; };
  werd = pkgs.callPackage ./werd.nix { inherit bun2nix; };
  inherit adboost;
  edge-adboost = pkgs.callPackage ./edge-adboost.nix { inherit adboost; };
  bb-bp = pkgs.callPackage ./bb-bp.nix { };
}
