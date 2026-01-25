{ config, lib, pkgs, inputs, ... }:

let
  bb-installer = pkgs.callPackage ../packages/bb-installer.nix { };
in
{
  imports = [
    "${toString pkgs.path}/nixos/modules/installer/cd-dvd/installation-cd-graphical-base.nix"
  ];

  # ISO image configuration
  isoImage = {
    isoBaseName = "bigbother";
    volumeID = "BIGBOTHER";
    squashfsCompression = "zstd -Xcompression-level 6";

    # Custom splash images from bigbother-theme
    splashImage = inputs.bigbother-theme + "/images/splashImage.png";
    efiSplashImage = inputs.bigbother-theme + "/images/splashImage.png";
    grubTheme = inputs.bigbother-theme + "/grub2-theme";
  };

  # Desktop environment for the live session
  services.xserver = {
    enable = true;
    desktopManager.plasma5.enable = true;
    displayManager.sddm.enable = true;
  };

  # Auto-login as nixos user (standard for NixOS ISOs)
  services.displayManager = {
    autoLogin = {
      enable = true;
      user = "nixos";
    };
  };

  # Disable screen lock in live session
  programs.xfconf.enable = true;

  # System packages for the live ISO
  environment.systemPackages = with pkgs; [
    bb-installer

    # Utilities for the installer
    gparted
    parted
    e2fsprogs
    dosfstools
    ntfs3g

    # Network tools
    networkmanagerapplet

    # General utilities
    firefox
    konsole
    dolphin

    # Text editor
    kwrite
  ];

  # Copy the BigBother flake to /etc/bb-flake for the installer
  environment.etc."bb-flake" = {
    source = ../.;
    mode = "0755";
  };

  # Create desktop entry for BigBother Installer
  environment.etc."xdg/autostart/bb-installer.desktop" = {
    text = ''
      [Desktop Entry]
      Type=Application
      Name=BigBother Installer
      Comment=Install BigBother NixOS
      Exec=${bb-installer}/bin/bb-installer
      Icon=system-software-install
      Terminal=false
      Categories=System;
      X-KDE-autostart-phase=2
    '';
    mode = "0644";
  };

  # Also add a visible desktop icon
  system.activationScripts.installerDesktopIcon = ''
    mkdir -p /home/nixos/Desktop
    cat > /home/nixos/Desktop/bb-installer.desktop << EOF
    [Desktop Entry]
    Type=Application
    Name=Install BigBother
    Comment=Install BigBother NixOS to your system
    Exec=${bb-installer}/bin/bb-installer
    Icon=system-software-install
    Terminal=false
    Categories=System;
    EOF
    chmod +x /home/nixos/Desktop/bb-installer.desktop
    chown nixos:users /home/nixos/Desktop/bb-installer.desktop 2>/dev/null || true
  '';

  # Networking
  networking = {
    hostName = "bigbother-live";
    networkmanager.enable = true;
    wireless.enable = false;
  };

  # Enable firmware for better hardware support
  hardware.enableRedistributableFirmware = true;

  # User configuration for live session
  users.users.nixos = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" "video" "audio" ];
    initialPassword = "";
    description = "Live Session User";
  };

  # Allow passwordless sudo for nixos user
  security.sudo.wheelNeedsPassword = false;

  # Disable some unnecessary services for live session
  documentation.enable = false;
  documentation.nixos.enable = false;

  # Set timezone
  time.timeZone = "UTC";

  # Internationalization
  i18n.defaultLocale = "en_US.UTF-8";

  system.stateVersion = "24.05";
}
