{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.bb-age-attestation = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable bb-age-attestation (D-Bus age bracket service)";
    };
  };

  config = mkIf config.bigbother.bb-age-attestation.enable {
    environment.systemPackages = [ pkgs.bb-age-attestation ];

    systemd.services.bb-age-attestation = {
      description = "BigBother Age Attestation D-Bus Service";
      wantedBy = [ "multi-user.target" ];
      after = [ "dbus.service" ];
      serviceConfig = {
        ExecStart = "${pkgs.bb-age-attestation}/bin/bb-age-attestation";
        Restart = "on-failure";
        RestartSec = 5;
      };
    };

    services.dbus.packages = [
      (pkgs.writeTextDir "share/dbus-1/system.d/org.bigbother.AgeAttestation1.conf" ''
        <!DOCTYPE busconfig PUBLIC
          "-//freedesktop//DTD D-BUS Bus Configuration 1.0//EN"
          "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
        <busconfig>
          <policy context="default">
            <allow send_destination="org.bigbother.AgeAttestation1"/>
            <allow receive_sender="org.bigbother.AgeAttestation1"/>
          </policy>
          <policy user="root">
            <allow own="org.bigbother.AgeAttestation1"/>
          </policy>
        </busconfig>
      '')
    ];
  };
}
