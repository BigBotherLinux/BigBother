{
  config,
  pkgs,
  lib,
  ...
}:

with lib;
{
  options.bigbother.adboost = {
    enable = mkOption {
      type = types.bool;
      default = true;
      description = "Enable Microsoft Edge with AdBoost, the extension that adds ads to web pages.";
    };
  };

  config = mkIf config.bigbother.adboost.enable {
    environment.systemPackages = [ pkgs.edge-adboost ];
  };
}
