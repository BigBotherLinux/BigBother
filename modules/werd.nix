{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.werd = {
    enable = mkOption {
      type = types.bool;
      default = true;
      description = "Enable werd, the one-word word processor.";
    };
  };

  config = mkIf config.bigbother.werd.enable {
    environment.systemPackages = [ pkgs.werd ];
  };
}
