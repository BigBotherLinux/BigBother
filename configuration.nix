{
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
    accidental-boot-protection.enable = true;
    sudo.enable = true;
    sddm.enable = true;
    theme.enable = true;
  };

  formatConfigs.vm =
    { config, ... }:
    {
      virtualisation = {
        cores = 4;
        memorySize = 6000;
        diskImage = "/dev/shm/${config.system.name}.qcow2";
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
    sharedModules = [
      inputs.plasma-manager.homeManagerModules.plasma-manager
    ];
    extraSpecialArgs = { inherit inputs; };
  };

  users.groups.nixos = { };

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
      microsoft-edge
      git
      inputs.bigbother-theme.packages.${pkgs.system}.bb-kde-theme
    ];

    # Set shell settings, such as alias to vim
    interactiveShellInit = ''
      alias nano='vim'
    '';
  };

  networking.hostName = lib.mkDefault "bigbother";

  services.xserver.xkb.layout = lib.mkDefault "no";
  system.stateVersion = "23.05";
}
