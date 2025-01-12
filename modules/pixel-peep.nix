{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.pixelPeep = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable the pixel peep systemd user service and timer.";
    };
    storageLocation = mkOption {
      type = types.str;
      default = "~/pixel-peep";
      description = "Location of where it should store the screenshots";
    };
  };

  config = mkIf config.bigbother.pixelPeep.enable {
    systemd.user.services."pixel-peep" = {
      description = "Pixel Peep screenshot service";
      startAt = "*-*-* *:*:00";
      preStart = "${pkgs.coreutils}/bin/mkdir -p ${config.bigbother.pixelPeep.storageLocation}";
      script = "${pkgs.kdePackages.spectacle}/bin/spectacle -f -b -n -o ${config.bigbother.pixelPeep.storageLocation}/evidence-$(${pkgs.coreutils-full}/bin/date +%Y-%m-%d-%H:%M:%S).png";
      serviceConfig = {
        Type = "oneshot";
      };
      wantedBy = [ "default.target" ];
    };
  };
}
