{ config, pkgs, ... }: {

  environment.systemPackages = [ 
    pkgs.neofetch 
    pkgs.btop 
  ]; 
  
  nixpkgs.config.packageOverrides = pkgs: {
    firefox = pkgs.microsoft-edge;  
  };
  environment.plasma5.excludePackages = with pkgs.libsForQt5; [
    elisa
    gwenview
    okular
    oxygen
    khelpcenter
    plasma-browser-integration
    print-manager
    kwalletmanager
  ];

  networking.hostName = "bigbother";

  services.xserver.layout = "no";

}
