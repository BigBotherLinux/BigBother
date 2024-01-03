{ lib, config, options, pkgs, ... }:
{
  imports = [
    ./mouse-drift.nix
    ./version.nix
    ./accidental-boot-protection.nix
    ./sudo.nix
    ./sddm.nix
    ./theme.nix
  ];
}