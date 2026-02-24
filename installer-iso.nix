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
  # Build bb-installer using the existing package definition
  bb-installer = pkgs.callPackage ./packages/bb-installer.nix { };
  bun2nix = inputs.bun2nix.packages.x86_64-linux.default;
  bbPackages = import ./packages { inherit pkgs bun2nix; };
in
{
  nixpkgs.config.allowUnfree = false;

  imports = [
    (modulesPath + "/installer/cd-dvd/installation-cd-minimal.nix")
    ./modules
  ];

  # ISO image configuration
  isoImage = {
    isoBaseName = lib.mkForce "bigbother";
    # volumeID = lib.mkForce "BB";
    splashImage = ./images/splashImage.png;
    squashfsCompression = lib.mkForce "xz -Xdict-size 100%"; # Better compatibility for Hyper-V

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

  # Load DRM kernel modules early
  boot.initrd.kernelModules = [
    "amdgpu"
    "radeon"
    "nouveau"
    "i915"
    "hv_balloon"
    "hv_netvsc"
    "hv_storvsc"
    "hv_utils"
    "hv_vmbus"
  ];

  boot.initrd.availableKernelModules = [
    "hyperv_keyboard"
  ];

  # Set default target to graphical for Wayland compositor
  systemd.defaultUnit = "graphical.target";

  # Disable getty on tty1 so cage can use it
  systemd.services."getty@tty1".enable = false;
  systemd.services."autovt@tty1".enable = false;

  # Use cage (kiosk Wayland compositor) to run bb-installer
  systemd.services.bb-installer-cage = {
    description = "BigBother Installer in Cage";
    wantedBy = [ "graphical.target" ];
    after = [
      "systemd-user-sessions.service"
      "multi-user.target"
    ];
    conflicts = [ "getty@tty1.service" ];

    serviceConfig = {
      Type = "simple";
      User = "root";
      # Ensure PATH includes system binaries (git, etc.)
      Environment = "PATH=/run/current-system/sw/bin:/run/wrappers/bin";
      ExecStart = pkgs.writeShellScript "bb-installer-cage" ''
        set -x  # Enable debug output

        # Create runtime directory
        export XDG_RUNTIME_DIR=/run/bb-installer
        mkdir -p $XDG_RUNTIME_DIR
        chmod 700 $XDG_RUNTIME_DIR

        # Use seatd for seat management
        export LIBSEAT_BACKEND=builtin

        # Set environment variables for bb-installer
        export BB_PROD=true
        export BB_FLAKE_PATH=/etc/bb-flake

        # Run bb-installer inside cage
        exec ${pkgs.cage}/bin/cage -s -- ${bb-installer}/bin/bb-installer
      '';
      StandardInput = "tty";
      StandardOutput = "tty";
      StandardError = "tty";
      TTYPath = "/dev/tty1";
      TTYReset = true;
      TTYVHangup = true;
      Restart = "on-failure";
    };
  };
  bigbother = {
    osInfo.enable = true; # version numbers in lsb-release
    bb-mouse-drift.enable = true;
  };

  # Copy the flake source into the ISO at /etc/bb-flake
  environment.etc."bb-flake".source = self;

  # System packages
  environment.systemPackages =
    with pkgs;
    [
      parted
      e2fsprogs
      dosfstools
      cage
      git
    ]
    ++ [
      bb-installer
    ];

  # Networking
  networking = {
    hostName = "bb-installer";
    # networkmanager.enable = true;
    # wireless.enable = false;
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
