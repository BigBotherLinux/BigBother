{
  config,
  pkgs,
  lib,
  inputs,
  ...
}:
let
  cfg = config.bigbother.theme;
  bb-kde-theme-service = pkgs.writeScriptBin "bb-kde-theme-service" ''
    #!${pkgs.stdenv.shell}
    set -e
    ${pkgs.plasma-workspace}/bin/plasma-apply-wallpaperimage ${inputs.bigbother-theme.packages.x86_64-linux.bb-kde-theme}/share/wallpapers/Crowded     
  '';
in
{
  options.bigbother.theme = {
    enable = lib.mkEnableOption "Big Bother enforce theme";
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [
      bb-kde-theme-service
    ];
    boot.plymouth.logo = inputs.bigbother-theme + "/images/logo.png";
    systemd.user.services.bb-theme-enforcement-service = {
      enable = true;
      after = [ "xdg-desktop-autostart.target" ];
      wantedBy = [ "xdg-desktop-autostart.target" ];
      serviceConfig = {
        ExecStart = "${bb-kde-theme-service}/bin/bb-kde-theme-service";
        Type = "oneshot";
        RemainAfterExit = true;
      };
    };
  };
}
