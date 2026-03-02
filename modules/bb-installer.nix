{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.bigbother.bb-installer;
  bb-installer = pkgs.callPackage ../packages/bb-installer.nix { };
in
{
  options.bigbother.bb-installer = {
    enable = lib.mkEnableOption "bb-installer cage kiosk service";
    prod = lib.mkEnableOption "production mode (sets BB_PROD=true in the service)";
  };

  config = lib.mkIf cfg.enable {

    systemd.defaultUnit = "graphical.target";

    # Disable getty on tty1 so cage can use it
    systemd.services."getty@tty1".enable = false;
    systemd.services."autovt@tty1".enable = false;

    systemd.services.bb-installer-cage = {
      description = "BigBother Installer in Cage";
      wantedBy = [ "graphical.target" ];
      after = [
        "systemd-user-sessions.service"
        "multi-user.target"
      ];
      conflicts = [ "getty@tty1.service" ];
      serviceConfig = {
        Type = "simple";
        User = "root";
        Environment = [
          "PATH=/run/current-system/sw/bin:/run/wrappers/bin"
        ]
        ++ lib.optional cfg.prod "BB_PROD=true";
        ExecStart = pkgs.writeShellScript "bb-installer-cage" ''
          export XDG_RUNTIME_DIR=/run/bb-installer
          export BB_FLAKE_PATH=/etc/bb-flake
          mkdir -p $XDG_RUNTIME_DIR
          chmod 700 $XDG_RUNTIME_DIR
          export LIBSEAT_BACKEND=builtin
          exec ${pkgs.cage}/bin/cage -s -- ${bb-installer}/bin/bb-installer
        '';
        StandardInput = "tty";
        StandardOutput = "tty";
        StandardError = "tty";
        TTYPath = "/dev/tty1";
        TTYReset = true;
        TTYVHangup = true;
        Restart = "on-failure";
      };
    };

    environment.systemPackages =
      with pkgs;
      [
        bb-installer
      ]
      ++ [
        cage
        parted
        e2fsprogs
        dosfstools
        git
      ];
  };
}
