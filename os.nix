{ config, pkgs, username, ... }:
{

  imports = [
    ./modules/version.nix
  ];
  bigbother.osInfo.enable = true; # version numbers in lsb-release


  # we need it specifically for hyper-v, 
  # im not sure if nixos-generate-config would have picked this up, so maybe it is only needed in the iso
  virtualisation.hypervGuest.enable = true; 

  nixpkgs.config.allowUnfree = true;

  nix.settings = {
    experimental-features = [ "nix-command" "flakes"];
  };

  boot.plymouth.logo = ./logo.png;

  # Enable home manager for the user
  # FYI: calamares will go in to this file and do a string replace on the username. 
  # It searches for 'users.nixos' and replaces it with 'users.<username>'
  home-manager.useGlobalPkgs = true;
  home-manager.useUserPackages = true;
  home-manager.users.nixos = import ./home.nix;

  environment = { 
    systemPackages = with pkgs; [ 
      microsoft-edge
      lsb-release
    ];

    # Set shell settings, such as alias to vim
    interactiveShellInit = ''
      alias nano='vim'
    '';

    # Write files to /etc, this is the last resort if nixos modules can't do it
    etc = {
    # Lecture file for sudo
    "/bb-sudoers.lecture".text = ''
      You are trying to run a command with root privileges, hopefully you know what you're about to do.
    '';
    };
  };

  # change badpass message and add lecture on sudo
  security.sudo.extraConfig = ''
    Defaults  badpass_message = "Wrong password, maybe try to type it correctly?"
    Defaults  lecture = always
    Defaults  lecture_file = /etc/bb-sudoers.lecture
  '';

  # Login screen: do not remember last logged in user
  services.xserver.displayManager.sddm.settings.Users = {
    RememberLastUser = false;
    RememberLastSession = false;
    MinimumUid = "10000";
  };

  # Accidental boot protection
  boot.loader.grub = {
    extraEntries = ''
    menuentry "Accidental boot protection" {
      
      clear
      echo "Accidental boot avoided, shutting down."
      sleep 3
      clear
      echo "Accidental boot avoided, shutting down.."
      sleep 1
      clear
      echo "Accidental boot avoided, shutting down..."
      sleep 3
      clear
      echo "Accidental boot avoided, shutting down...."
      sleep 2
      clear
      echo "Accidental boot avoided, shutting down....."
      sleep 5
      clear
      echo "Accidental boot avoided, shutting down......"
      sleep 2
      clear
      echo "Accidental boot avoided, shutting down......."
      sleep 1
      halt
    }
    '';
    extraConfig = "set theme=($drive2)${pkgs.breeze-grub}/grub/themes/breeze/theme.txt";
    splashImage = null; # TODO: Add some branding here
    extraEntriesBeforeNixOS = true;
  };
}
