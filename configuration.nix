{
  config,
  lib,
  pkgs,
  inputs,
  outputs,
  ...
}:
{

  imports = [
    ./modules
    inputs.home-manager.nixosModules.home-manager
  ] ++ lib.optionals (builtins.pathExists ./hardware-configuration.nix) [
    ./hardware-configuration.nix
  ] ++ lib.optionals (builtins.pathExists ./installer.nix) [
    ./installer.nix
  ];

  bigbother = {
    osInfo.enable = true; # version numbers in lsb-release
    bb-mouse-drift.enable = true;
    # accidental-boot-protection.enable = true;
    sudo.enable = true;
    sddm.enable = true;
    theme.enable = true;
    bb-bp.enable = false;
    incel.enable = true;
  };

  formatConfigs.vm =
    { config, ... }:
    {
      virtualisation = {
        cores = 4;
        memorySize = 8000;
        diskImage = "/dev/shm/${config.system.name}.qcow2";
        qemu.options = [
          "-vga none"
          "-device virtio-vga-gl"
          "-display gtk,gl=on"
        ];
      };
    };

  nix.settings = {
    experimental-features = [
      "nix-command"
      "flakes"
    ];
  };

  nixpkgs = {
    config = {
      allowUnfree = true;
    };
    overlays = [
      outputs.overlays.additions
      outputs.overlays.modifications
      outputs.overlays.stable-packages
      outputs.overlays.flake-inputs
    ];
    # config.packageOverrides = localPkgs: {
    #   calamares-nixos-extensions = inputs.calamares-bb.packages.${pkgs.system}.calamares-nixos-extensions;
    # };
  };

  fonts.packages = with pkgs; [
    underpass
  ];

  documentation = {
    doc.enable = false;
    man.enable = false;
  };

  home-manager = {
    useGlobalPkgs = true;
    useUserPackages = true;
    sharedModules = [
      inputs.plasma-manager.homeManagerModules.plasma-manager
    ];
    extraSpecialArgs = { inherit inputs; };
  };

  users.users.${config.bigbother.primaryUser} = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" "video" "audio" config.programs.ydotool.group ];
    initialPassword = lib.mkDefault "bothered";
  };

  home-manager.users.${config.bigbother.primaryUser} = import ./home.nix;

  services = {
    desktopManager.plasma6.enable = true;
    displayManager = {
      sddm.enable = true;
      # autoLogin.enable = lib.mkDefault true;
      # autoLogin.user = lib.mkDefault "test";
    };
  };

  environment = {
    systemPackages = with pkgs; [
      git
      inputs.bigbother-theme.packages.${pkgs.system}.bb-kde-theme
    ];

    # Set shell settings, such as alias to vim
    interactiveShellInit = ''
      alias nano='vim'
    '';
  };

  networking.hostName = lib.mkDefault "bigbother";

  # Default filesystems (overridden by hardware-configuration.nix during install)
  fileSystems."/" = lib.mkDefault {
    device = "/dev/disk/by-label/nixos";
    fsType = "ext4";
  };
  fileSystems."/boot" = lib.mkDefault {
    device = "/dev/disk/by-label/boot";
    fsType = "vfat";
  };

  # Default boot loader (overridden by hardware-configuration.nix during install)
  boot.loader.systemd-boot.enable = lib.mkDefault true;
  boot.loader.efi.canTouchEfiVariables = lib.mkDefault true;

  services.xserver.xkb.layout = lib.mkDefault "no";
  system.stateVersion = "23.05";
}
