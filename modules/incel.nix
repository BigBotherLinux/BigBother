{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.incel = {
    enable = mkOption {
      type = types.bool;
      default = true;
      description = "Enable Incel, the involuntarily single-celled spreadsheet.";
    };
  };

  config = mkIf config.bigbother.incel.enable {
    environment.systemPackages = [ pkgs.incel ];
  };
}
