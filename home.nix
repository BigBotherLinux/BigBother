{ config, pkgs, inputs, ... }:
{
  programs = { 
    home-manager.enable = true;
    #nano.enable = false; # just learn vim lol
    vim.enable = true;
    bigbother-theme = {
      enable = true;  
    };
    plasma = {
      enable = true;
      # plasma-apply-desktoptheme --list-themes
    };
  };
  home = {
      packages = with pkgs; [
        neofetch
      ];
      stateVersion = "23.05";
    };
}
