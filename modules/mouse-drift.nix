{ config, pkgs, lib, inputs, ... }:
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
  ## REQUIRES inputs.xremap.nixosModules.default to be imported.
  ## TODO: Optionally import it here.
  # imports = [
  #   inputs.xremap.nixosModules.default
  # ];

  options.bigbother.bb-mouse-drift = {
    enable = lib.mkEnableOption "bb-mouse-drift";
    enableSafeSpace = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Enable safe space for mouse drift";
    };
  };
  
  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ 
      pkgs.ydotool
      bb-mouse-drift
    ];

    # Daemon to run ydotool
    systemd.services.ydotoold = {
      enable = true;
      serviceConfig = {
        ExecStart = "${pkgs.ydotool}/bin/ydotoold -P 777";
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

    services.xremap = lib.mkIf cfg.enableSafeSpace {
      userName = "nixos";
      serviceMode = "user";
      withKDE = true;
      yamlConfig = ''
      keymap:
      - name: test
        remap:
          KEY_SPACE: 
            launch: ["${pkgs.bash}/bin/bash", "-c", "[ $(( RANDOM % 100 )) -lt 90 ] && YDOTOOL_SOCKET=/tmp/.ydotool_socket ${pkgs.ydotool}/bin/ydotool key 57:1 57:0"]
      '';
    };
  };
}