# Build with nix build .\#nixosConfigurations.bigbotherpc.config.formats.isogen

{
  description = "BigBrother NixOS ISO";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    home-manager.url = "github:nix-community/home-manager/release-23.05";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";
    #calamares-bb.url = "git+file:///home/hausken/Projects/BigBother/calamares";
    calamares-bb.url = "github:hauskens/calamares-nixos-extensions";
    plasma-manager.url = "github:pjones/plasma-manager";
    plasma-manager.inputs.nixpkgs.follows = "nixpkgs";
    plasma-manager.inputs.home-manager.follows = "home-manager";
    nixos-generators = {
      url = "github:nix-community/nixos-generators";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, nixos-generators, calamares-bb, home-manager, ... }: {


    nixosModules.bigbotherinstaller = {config, ...}: {
      imports = [
        nixos-generators.nixosModules.all-formats
      ];
      nixpkgs.hostPlatform = "x86_64-linux";
      
      formatConfigs.isogen = {config, modulesPath, ...}: {
        imports = ["${toString modulesPath}/installer/cd-dvd/installation-cd-graphical-calamares-plasma5.nix"];
        isoImage.squashfsCompression = "zstd -Xcompression-level 3";
        formatAttr = "isoImage";
        fileExtension = ".iso";
      };
    };

    nixosConfigurations.bigbotherpc = nixpkgs.lib.nixosSystem {
      modules = [
        ./os.nix
        ./configuration.nix
        home-manager.nixosModules.home-manager
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.nixos = import ./home.nix;
        }
      ];    
    };
      
    nixosConfigurations.bigbotherinstaller = nixpkgs.lib.nixosSystem {
      modules = [
        self.nixosModules.bigbotherinstaller
        ./os.nix
        ./installer.nix
        home-manager.nixosModules.home-manager
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.nixos = import ./home.nix;
        }
        ({ pkgs, ... }: {
            environment.systemPackages = [ calamares-bb.packages.x86_64-linux.calamares-nixos-extensions ];
            nixpkgs.config.packageOverrides = localPkgs: {
              calamares-nixos-extensions = calamares-bb.packages.x86_64-linux.calamares-nixos-extensions;
            };
            environment.etc."bigbother/os.nix".source = "${self}/os.nix";
            environment.etc."bigbother/flake.nix".source = "${self}/flake.nix";
            environment.etc."bigbother/flake.lock".source = "${self}/flake.lock";
            environment.etc."bigbother/home.nix".source = "${self}/home.nix";
            environment.etc."bigbother/modules".source = "${self}/modules/";
          })
      ];    
    };
  };
}
