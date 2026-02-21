{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.bb-nag = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable bb-nag (notification annoyance generator)";
    };
  };

  config = mkIf config.bigbother.bb-nag.enable {
    environment.systemPackages = [ pkgs.bb-nag ];

    systemd.user.services.bb-nag = {
      description = "BigBother Nag - Notification annoyance generator";
      wantedBy = [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      serviceConfig = {
        ExecStart = "${pkgs.bb-nag}/bin/bb-nag";
        Restart = "on-failure";
        RestartSec = 5;
      };
    };
  };
}
