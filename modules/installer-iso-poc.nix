# Minimal POC installer ISO that auto-starts bb-installer
{ config, lib, pkgs, inputs, self, outputs, modulesPath, ... }:

let
  # Build bb-installer using the existing package definition
  bb-installer = pkgs.callPackage ../packages/bb-installer.nix { };
in
{
  imports = [
    (modulesPath + "/installer/cd-dvd/installation-cd-minimal.nix")
  ];

  # ISO image configuration
  isoImage = {
    isoBaseName = lib.mkForce "bigbother-poc";
    volumeID = lib.mkForce "BB_POC";
    squashfsCompression = lib.mkForce "zstd -Xcompression-level 3"; # Faster compression for POC
  };

  # Use openbox as minimal window manager with auto-login
  services.xserver = {
    enable = true;
    windowManager.openbox.enable = true;
    displayManager = {
      lightdm = {
        enable = true;
        greeter.enable = false;
      };
      sessionCommands = ''
        # Set keyboard layout
        ${pkgs.xorg.setxkbmap}/bin/setxkbmap us &

        # Launch bb-installer as root (passwordless sudo already configured)
        sudo BB_PROD=true BB_FLAKE_PATH=/etc/bb-flake ${bb-installer}/bin/bb-installer &
      '';
    };
  };

  services.displayManager.autoLogin = {
    enable = true;
    user = "nixos";
  };

  # Enable libinput for keyboard/mouse input
  services.libinput.enable = true;

  # XKB configuration for keyboard layout
  services.xserver.xkb = {
    layout = "us";
    options = "";
  };

  # Auto-login on tty1
  services.getty.autologinUser = "nixos";

  # Copy the flake source into the ISO at /etc/bb-flake
  environment.etc."bb-flake".source = self;

  # System packages
  environment.systemPackages = [
    bb-installer
    pkgs.parted
    pkgs.e2fsprogs
    pkgs.dosfstools
    pkgs.openbox
  ];

  # Networking
  networking = {
    hostName = "bb-installer-poc";
    # networkmanager.enable = true;
    # wireless.enable = false;
  };

  # User configuration
  users.users.nixos = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" "video" ];
    initialPassword = "";
  };

  security.sudo.wheelNeedsPassword = false;

  # Minimal docs
  documentation.enable = false;
  documentation.nixos.enable = false;

  system.stateVersion = "24.05";
}
