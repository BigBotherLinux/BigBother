{
  config,
  pkgs,
  lib,
  ...
}:
let
  # Script that moves the mouse cursor randomly in a direction
  cfg = config.bigbother.bb-mouse-drift;

  bb-mouse-drift = pkgs.writeScriptBin "bb-mouse-drift" ''
    #!${pkgs.stdenv.shell}
    set -e
    sleep_interval=0.05

    while true; do
      random_number=$((RANDOM % 2))

      if [ $random_number -eq 0 ]; then
        ${pkgs.ydotool}/bin/ydotool mousemove -x 1 -y 0
      else
        ${pkgs.ydotool}/bin/ydotool mousemove -x 0 -y 1
      fi

      sleep $sleep_interval
    done
  '';
in
{

  options.bigbother.bb-mouse-drift = {
    enable = lib.mkEnableOption "bb-mouse-drift";
    enableSafeSpace = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Enable safe space for mouse drift";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [
      pkgs.ydotool
      bb-mouse-drift
    ];

    programs.ydotool.enable = true;

    # Service that runs mousedrift script
    systemd.services.bb-mouse-drift-service = {
      enable = true;
      after = [ "ydotoold.service" ];
      requires = [ "ydotoold.service" ];
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        Environment = [ "YDOTOOL_SOCKET=/run/ydotoold/socket" ];
        ExecStart = "${bb-mouse-drift}/bin/bb-mouse-drift";
        Restart = "on-failure";
        User = "root";
      };
    };

  };
}
