{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.bb-age-refresher = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable bb-age-refresher (periodic age re-attestation prompts)";
    };
  };

  config = mkIf config.bigbother.bb-age-refresher.enable {
    environment.systemPackages = [ pkgs.bb-age-refresher ];

    systemd.user.services.bb-age-refresher = {
      description = "BigBother Age Verification Refresher";
      wantedBy = [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${pkgs.bb-age-refresher}/bin/bb-age-refresher";
        Restart = "on-failure";
        RestartSec = "5s";
      };
    };

    systemd.user.timers.bb-age-refresher = {
      description = "Periodically prompt age re-attestation";
      wantedBy = [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      timerConfig = {
        OnStartupSec = "0";
        OnUnitActiveSec = "1h";
        RandomizedDelaySec = "30m";
      };
    };
  };
}
