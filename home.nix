{ home-manager, config, pkgs, username , ... }:
{
  home-manager.useGlobalPkgs = true;
  home-manager.useUserPackages = true;
  home-manager.users."${username}" = {
    programs.home-manager.enable = true;
    programs.zsh.enable = true;
    home.stateVersion = "22.11";
  };
}
