{ config, lib, ... }:
let
  # Script that moves the mouse cursor randomly in a direction
  cfg = config.bigbother.accidental-boot-protection;
in
{

  options.bigbother.accidental-boot-protection = {
    enable = lib.mkEnableOption "accidental-boot-protection";
    description = "Enable Accidental boot protection boot menu";
  };

  config = lib.mkIf cfg.enable {
    # Accidental boot protection
    boot.loader.grub = {
      #splashImage = inputs.bigbother-theme + "/images/logo.png";
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
          sleep 1
          clear
          echo "Accidental boot avoided, shutting down......"
          sleep 2
          clear
          echo "Accidental boot avoided, shutting down......."
          sleep 1
          halt
        }
      '';
      extraEntriesBeforeNixOS = true;
    };
  };
}
