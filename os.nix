{ config, pkgs, ... }:
# To build this, run:
# nix-build '<nixpkgs/nixos>' -A config.system.build.isoImage -I nixos-config=os.nix -I nixpkgs=channel:nixos-23.05
#let
#  localCalamares = import ./bigbother-calamares.nix { inherit (pkgs) stdenv lib; };
#  nixos-version = "23.05";
#in
{

#  nixpkgs.config.packageOverrides = pkgs: {
#    calamares-nixos-extensions = localCalamares;
#  };
#  imports = [
    #<nixpkgs/nixos/modules/installer/cd-dvd/installation-cd-graphical-calamares-plasma5.nix>
    # Provide an initial copy of the NixOS channel so that the user
    # doesn't need to run "nix-channel --update" first.
    #<nixpkgs/nixos/modules/installer/cd-dvd/channel.nix>

    # Import the main config for the installer
#    (import ./calamares/modules/nixos/bigbother-config.nix { username = "test1"; fullname = "test1full"; nixversion = nixos-version; inherit pkgs; })
#  ];

  services.xserver.layout = "no";
  isoImage.squashfsCompression = "gzip -Xcompression-level 1";
  nixpkgs.config.allowUnfree = true;
#  system.stateVersion = nixos-version;
}
