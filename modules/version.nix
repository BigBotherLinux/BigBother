# yoink -> https://github.com/snowflakelinux/snowflake-modules/blob/main/modules/version.nix

{
  lib,
  config,
  options,
  pkgs,
  ...
}:
let
  cfg = config.system.nixos;
  needsEscaping = s: null != builtins.match "[a-zA-Z0-9]+" s;
  escapeIfNeccessary =
    s: if needsEscaping s then s else ''"${lib.escape [ "\$" "\"" "\\" "\`" ] s}"'';
  attrsToText =
    attrs:
    lib.concatStringsSep "\n" (
      lib.mapAttrsToList (n: v: ''${n}=${escapeIfNeccessary (toString v)}'') attrs
    )
    + "\n";
  osReleaseContents = {
    NAME = "BigBother";
    ID = "bigbother";
    VERSION = "${cfg.release} (${cfg.codeName})";
    VERSION_CODENAME = lib.toLower cfg.codeName;
    VERSION_ID = cfg.release;
    BUILD_ID = cfg.version;
    PRETTY_NAME = "BigBother ${cfg.release} (${cfg.codeName})";
    LOGO = "nix-snowflake-white";
    HOME_URL = "https://github.com/BigBotherLinux/BigBother"; # TODO CHANGE THIS
    DOCUMENTATION_URL = "";
    SUPPORT_URL = "";
    BUG_REPORT_URL = "";
  };
  initrdReleaseContents = osReleaseContents // {
    PRETTY_NAME = "${osReleaseContents.PRETTY_NAME} (Initrd)";
  };
  initrdRelease = pkgs.writeText "initrd-release" (attrsToText initrdReleaseContents);
in
{
  options.bigbother.osInfo = {
    enable = lib.mkEnableOption "BigBother Main System";
  };

  config = lib.mkIf config.bigbother.osInfo.enable {
    environment.etc."os-release".text = lib.mkForce (attrsToText osReleaseContents);
    environment.etc."lsb-release".text = lib.mkForce (attrsToText {
      LSB_VERSION = "${cfg.release} (${cfg.codeName})";
      DISTRIB_ID = "bigbother";
      DISTRIB_RELEASE = cfg.release;
      DISTRIB_CODENAME = lib.toLower cfg.codeName;
      DISTRIB_DESCRIPTION = "Big Bother ${cfg.release} (${cfg.codeName})";
    });
    boot.initrd.systemd.contents."/etc/os-release".source = lib.mkForce initrdRelease;
    boot.initrd.systemd.contents."/etc/initrd-release".source = lib.mkForce initrdRelease;

    system.nixos.distroName = "BigBother";
    system.nixos.distroId = "bigbother";
  };
}
