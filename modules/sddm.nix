{ config, lib, ... }:
let
  cfg = config.bigbother.sddm;
in
{
  options.bigbother.sddm = {
    enable = lib.mkEnableOption "Big Bother sddm config";
    description = "Enable Big Bother sddm config";
  };

  config = lib.mkIf cfg.enable {
    # Login screen: do not remember last logged in user
    services.displayManager.sddm.settings.Users = {
      RememberLastUser = false;
      RememberLastSession = false;
      MinimumUid = "10000";
    };
  };
}
