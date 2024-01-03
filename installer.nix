{ self, config, pkgs, inputs, system, ... }: 
{
  environment.systemPackages = [ 
    inputs.calamares-bb.packages.${system}.calamares-nixos-extensions 
    pkgs.neofetch 
    pkgs.btop 
  ];

  # TODO: Maybe a more elegfant way to do this?
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

  # Copy the files needed for the installer to the ISO, Calamares will copy these onto the system
  environment.etc."bigbother/os.nix".source = "${self}/os.nix";
  environment.etc."bigbother/flake.nix".source = "${self}/flake.nix";
  environment.etc."bigbother/flake.lock".source = "${self}/flake.lock";
  environment.etc."bigbother/home.nix".source = "${self}/home.nix";
  environment.etc."bigbother/modules".source = "${self}/modules/";
}