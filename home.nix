{ config, pkgs, ... }:
{
  programs = { 
    home-manager.enable = true;
    #nano.enable = false; # just learn vim lol
    vim.enable = true;
    gust-cursor-theme = {
      enable = true;  
      #package = bigbother-theme.packages.x86_64-linux.gust-cursor-theme;
    };
  };
  home = {
      packages = with pkgs; [
        neofetch
      ];
      stateVersion = "23.05";
    };
}
