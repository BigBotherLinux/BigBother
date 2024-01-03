{ config, pkgs, inputs, system, ... }:
{

  imports = [
    ./modules
    inputs.xremap.nixosModules.default
  ];
  bigbother.osInfo.enable = true; # version numbers in lsb-release
  bigbother.bb-mouse-drift.enable = true;
  bigbother.accidental-boot-protection.enable = true;
  bigbother.sudo.enable = true;
  bigbother.sddm.enable = true;
  bigbother.theme.enable = true;

  # we need it specifically for hyper-v, 
  # im not sure if nixos-generate-config would have picked this up, so maybe it is only needed in the iso
  virtualisation.hypervGuest.enable = true; 

  nixpkgs.config.allowUnfree = true;

  nix.settings = {
    experimental-features = [ "nix-command" "flakes"];
  };

  boot.plymouth.logo = inputs.bigbother-theme + "/images/logo.png";


  nixpkgs.config.packageOverrides = localPkgs: {
    calamares-nixos-extensions = inputs.calamares-bb.packages.${system}.calamares-nixos-extensions;
  };
  home-manager = {
    useGlobalPkgs = true;
    useUserPackages = true;
    sharedModules = [ 
      # inputs.bigbother-theme.homeManagerModules.bigbother-theme 
      inputs.plasma-manager.homeManagerModules.plasma-manager 
    ];  
    extraSpecialArgs = { inherit inputs; };

    # Enable home manager for the user
    # FYI: calamares will go in to this file and do a string replace on the username. 
    # It searches for 'users.nixos' and replaces it with 'users.<username>'
    users.nixos = import ./home.nix;  
  };
  environment = { 
    systemPackages = with pkgs; [ 
      microsoft-edge
      inputs.bigbother-theme.packages.${system}.bb-kde-theme
      inputs.bigbother-theme.packages.${system}.gust-cursor-theme
    ];

    # Set shell settings, such as alias to vim
    interactiveShellInit = ''
      alias nano='vim'
    '';
  };
}
