pkgs: {
  underpass = pkgs.callPackage ./underpass.nix { };
  gust-cursor-theme = pkgs.callPackage ./gust.nix { };
  bb-installer = pkgs.callPackage ./bb-installer.nix { };
}
