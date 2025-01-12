{
  lib,
  pkgs,
  config,
  inputs,
  ...
}:
let
  inherit (lib) mkEnableOption mkIf optionalAttrs;
  cfg = config.bigbother.theme;
in
{
  options.bigbother.theme = {
    enable = mkEnableOption "Enable bigBother desktop theme";
    cursor = mkEnableOption "Enable 'Gust' cursor theme";
    font = mkEnableOption "Enable 'Underpass' font";
  };

  config = mkIf cfg.enable {
    home.file.".local/share/icons/Gust" = {
      source = pkgs.gust-cursor-theme;
    };
    xdg.configFile."kcminputrc".text = lib.strings.optionalString cfg.cursor ''
      [Mouse]
      cursorTheme=Gust
    '';

    programs.plasma = {
      configFile = optionalAttrs cfg.font {
        kdeglobals = {
          General = {
            "font" = "Underpass,10,-1,5,50,0,0,0,0,0";
            # "fixed" = "Underpass,10,-1,5,50,0,0,0,0,0";
            "smallestReadableFont" = "Underpass,8,-1,5,50,0,0,0,0,0";
            "toolBarFont" = "Underpass,12,-1,5,50,0,0,0,0,0";
            "menuFont" = "Underpass,10,-1,5,50,0,0,0,0,0";
          };
        };
      };
      workspace.wallpaper = "${inputs.bigbother-theme.packages.x86_64-linux.bb-kde-theme}/share/wallpapers/Crowded";
    };

  };
}
