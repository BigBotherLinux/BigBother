{ config, pkgs, username, ... }:
{
  nixpkgs.config.allowUnfree = true;

  nix.settings = {
    experimental-features = [ "nix-command" "flakes"];
  };

  imports = [
    ./modules/version.nix
  ];
  bigbother.osInfo.enable = true;
  boot.plymouth.logo = ./logo.png;
  environment = { 
    systemPackages = with pkgs; [ 
      microsoft-edge
      lsb-release
    ];
    # just learn vim lol
    interactiveShellInit = ''
      alias nano='vim'
    '';
    etc = {
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

  # do not remember last logged in user
  services.xserver.displayManager.sddm.settings.Users = {
    RememberLastUser = false;
    RememberLastSession = false;
    MinimumUid = "10000";
  };

  # Accidental boot protection
  boot.loader.grub = {
    extraEntries = ''
    menuentry "Accidental boot protection" {

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
    splashImage = null;
    extraEntriesBeforeNixOS = true;
  };

}
