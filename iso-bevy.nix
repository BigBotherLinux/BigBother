# Variant of the installer ISO that boots the bevy prototype instead of the
# eframe installer. Layered on top of installer-iso.nix so the two stay in sync.
{ lib, ... }:
{
  imports = [
    ./installer-iso.nix
    ./modules/bb-installer-bevy.nix
  ];

  bigbother = {
    # Only one kiosk can own tty1.
    bb-installer.enable = lib.mkForce false;

    bb-installer-bevy = {
      enable = true;
      # The ISO is expected to run in QEMU without a GPU.
      softwareRendering = true;
    };
  };

  # installer-iso.nix already sets this with mkForce, so this needs to outrank it.
  image.baseName = lib.mkOverride 40 "bigbother-bevy";

  networking.hostName = lib.mkForce "bb-bevy";
}
