{ config, pkgs, ... }:
let
  # Script that moves the mouse cursor randomly in a direction
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
  environment.systemPackages = [ 
    pkgs.ydotool
    bb-mouse-drift
  ];

  # Daemon to run ydotool
  systemd.services.ydotoold = {
    enable = true;
    serviceConfig = {
      ExecStart = "${pkgs.ydotool}/bin/ydotoold";
      Restart = "always";
      User = "root";
    };
    wantedBy = [ "multi-user.target" ];
  };

  # Service that runs mousedrift script
  systemd.services.bb-mouse-drift-service = {
    enable = true;
    after = [ "ydotoold.service" ];
    requires = [ "ydotoold.service" ];
    wantedBy = [ "multi-user.target" ];
    serviceConfig = {
      ExecStart = "${bb-mouse-drift}/bin/bb-mouse-drift";
      Restart = "on-failure";
      User = "root";
    };
  };
}