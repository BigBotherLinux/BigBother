{ pkgs, ... }: {
  environment.systemPackages = [ calamares-bb.packages.x86_64-linux.calamares-nixos-extensions ];
    nixpkgs.config.packageOverrides = localPkgs: {
    calamares-nixos-extensions = calamares-bb.packages.x86_64-linux.calamares-nixos-extensions;
  };
  environment.etc.test2.source = "${self}/os.nix";
}
