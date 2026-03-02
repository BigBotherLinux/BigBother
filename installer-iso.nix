# Minimal POC installer ISO that auto-starts bb-installer
{
  lib,
  pkgs,
  inputs,
  self,
  modulesPath,
  ...
}:

let
  bun2nix = inputs.bun2nix.packages.x86_64-linux.default;
  bbPackages = import ./packages { inherit pkgs bun2nix; };
in
{
  nixpkgs.config.allowUnfree = false;

  imports = [
    (modulesPath + "/installer/cd-dvd/installation-cd-minimal.nix")
    ./modules
    ./modules/bb-installer.nix
  ];

  bigbother = {
    bb-installer = {
      enable = true;
      prod = true;
    };
    osInfo.enable = true; # version numbers in lsb-release
    bb-mouse-drift.enable = true;
  };

  # ISO image configuration
  isoImage = {
    isoBaseName = lib.mkForce "bigbother";
    splashImage = ./images/splashImage.png;

    # Pre-build bun-based packages on the host and include them in the ISO's
    # nix store. Bun requires AVX2 which QEMU's default CPU doesn't support,
    # so these can't be built inside the VM.
    storeContents = with bbPackages; [
      incel
      werd
      bb-installer
      bb-bp
      bb-nag
    ];
  };

  # Enable graphics support for Wayland compositor
  hardware.graphics.enable = true;

  boot.initrd.kernelModules = [
    "hv_balloon"
    "hv_netvsc"
    "hv_storvsc"
    "hv_utils"
    "hv_vmbus"
  ];

  boot.initrd.availableKernelModules = [
    "hyperv_keyboard"
  ];

  # Copy the flake source into the ISO at /etc/bb-flake
  environment.etc."bb-flake".source = self;

  # Networking
  networking = {
    hostName = "bb-installer";
  };

  # User configuration
  users.users.nixos = {
    isNormalUser = true;
    extraGroups = [
      "wheel"
      "networkmanager"
      "video"
    ];
    initialPassword = "";
  };

  security.sudo.wheelNeedsPassword = false;

  # Minimal docs
  documentation.enable = false;
  documentation.nixos.enable = false;

  system.stateVersion = "24.05";
}
