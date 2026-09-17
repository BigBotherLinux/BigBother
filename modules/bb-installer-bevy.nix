{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.bigbother.bb-installer-bevy;

  # Applied to the app only, never to cage: forcing llvmpipe breaks the
  # compositor's EGL setup, and pinning lavapipe stops wlroots matching a
  # Vulkan device to the DRM node, so cage dies before the app ever starts.
  clientEnv = lib.optionals cfg.softwareRendering [
    "LIBGL_ALWAYS_SOFTWARE=1"
    "VK_ICD_FILENAMES=/run/opengl-driver/share/vulkan/icd.d/lvp_icd.x86_64.json"
    "WGPU_BACKEND=vulkan"
  ];

  clientPrefix = lib.optionalString (
    clientEnv != [ ]
  ) "${pkgs.coreutils}/bin/env ${lib.escapeShellArgs clientEnv}";
in
{
  options.bigbother.bb-installer-bevy = {
    enable = lib.mkEnableOption "bb-installer-bevy cage kiosk service";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ../packages/bb-installer-bevy.nix { };
      defaultText = lib.literalExpression "pkgs.callPackage ../packages/bb-installer-bevy.nix { }";
      description = "The bevy installer package to run. Overridable so tests can inject a build.";
    };

    softwareRendering = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = ''
        Force the app (not the compositor) onto llvmpipe/lavapipe software
        rendering. Needed in VMs and on the ISO when no GPU driver is available.
      '';
    };

    extraEnvironment = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [ "WGPU_BACKEND=gl" ];
      description = "Extra environment entries for the kiosk service.";
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.defaultUnit = "graphical.target";

    # Free up tty1 so cage can own it.
    systemd.services."getty@tty1".enable = false;
    systemd.services."autovt@tty1".enable = false;

    hardware.graphics.enable = true;

    systemd.services.bb-installer-bevy-cage = {
      description = "BigBother Installer (bevy) in Cage";
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
          "RUST_BACKTRACE=1"
        ]
        ++ cfg.extraEnvironment;
        ExecStart = pkgs.writeShellScript "bb-installer-bevy-cage" ''
          export XDG_RUNTIME_DIR=/run/bb-installer-bevy
          mkdir -p $XDG_RUNTIME_DIR
          chmod 700 $XDG_RUNTIME_DIR
          export LIBSEAT_BACKEND=builtin
          exec ${pkgs.cage}/bin/cage -s -- ${clientPrefix} ${cfg.package}/bin/bb-installer-bevy
        '';
        StandardInput = "tty";
        StandardOutput = "journal+console";
        StandardError = "journal+console";
        TTYPath = "/dev/tty1";
        TTYReset = true;
        TTYVHangup = true;
        Restart = "on-failure";
      };
    };

    environment.systemPackages = [
      cfg.package
      pkgs.cage
    ];
  };
}
