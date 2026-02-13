{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.bigbother.bb-bp;

  # Wrapper script that runs bb-bp inside cage with seatd
  bbBpWrapper = pkgs.writeShellScript "bb-bp-wrapper" ''
    # Create runtime directory
    export XDG_RUNTIME_DIR=/run/bb-bp
    mkdir -p $XDG_RUNTIME_DIR
    chmod 700 $XDG_RUNTIME_DIR

    # Use seatd for seat management (works before logind sessions exist)
    export LIBSEAT_BACKEND=builtin

    # Run bb-bp inside cage (kiosk Wayland compositor)
    ${pkgs.cage}/bin/cage -s -- ${cfg.package}/bin/bb-bp

    # Switch to TTY7 for display manager
    ${pkgs.util-linux}/bin/chvt 7
  '';
in
{
  options.bigbother.bb-bp = {
    enable = mkEnableOption "BigBother Pre-Login Splash Screen";

    package = mkOption {
      type = types.package;
      default = pkgs.bb-bp;
      description = "The bb-bp package to use";
    };
  };

  config = mkIf cfg.enable {
    # Add the packages to system packages
    environment.systemPackages = [ cfg.package pkgs.cage ];

    # Create the systemd service
    systemd.services.bb-bp = {
      description = "BigBother Pre-Login Splash Screen";
      wantedBy = [ "graphical.target" ];
      before = [ "display-manager.service" ];
      after = [ "systemd-user-sessions.service" ];

      serviceConfig = {
        Type = "oneshot";
        RemainAfterExit = true;
        ExecStart = "${bbBpWrapper}";
        StandardInput = "tty";
        StandardOutput = "tty";
        TTYPath = "/dev/tty1";
        TTYReset = true;
        TTYVHangup = true;
      };
    };

    # Make the display manager wait for our service (but not require it)
    systemd.services.display-manager = {
      after = [ "bb-bp.service" ];
      wants = [ "bb-bp.service" ];
    };
  };
}
