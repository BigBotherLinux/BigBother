{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.fastfetch = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable bb-fastfetch with theme";
    };
  };

  config = mkIf config.bigbother.fastfetch.enable {
    environment.systemPackages = [ pkgs.bb-fastfetch ];
  };
}
