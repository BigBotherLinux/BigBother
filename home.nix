{ config, pkgs, inputs, ... }:
{
  programs = { 
    home-manager.enable = true;
    #nano.enable = false; # just learn vim lol
    vim.enable = true;
    
    plasma = {
      enable = true;
      configFile."kwinrc"."ElectricBorders"."BottomLeft" = "LockScreen";
      configFile."kwinrc"."ElectricBorders"."Left" = "LockScreen";
      configFile."kwinrc"."ElectricBorders"."BottomRight" = "LockScreen";
      configFile."kwinrc"."ElectricBorders"."Right" = "LockScreen";
      configFile."kwinrc"."ElectricBorders"."Top" = "LockScreen";
      configFile."kwinrc"."ElectricBorders"."TopLeft" = "LockScreen";
      configFile."kwinrc"."ElectricBorders"."TopRight" = "LockScreen";
      configFile."kwinrc"."Windows"."ElectricBorderDelay" = 0;
      configFile."ksplashrc"."KSplash"."Engine" = "none";
      workspace.wallpaper = "${inputs.bigbother-theme.packages.x86_64-linux.bb-kde-theme}/share/wallpapers/Crowded";
    };
  };
  home.file.".local/share/icons/Gust".source = inputs.bigbother-theme.packages.x86_64-linux.gust-cursor-theme;
  xdg.configFile."kcminputrc".text = ''
      [Mouse]
      cursorTheme=Gust
    '';
  home = {
    packages = with pkgs; [
      neofetch
    ];
    stateVersion = "23.05";
  };
}
