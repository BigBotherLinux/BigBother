{
  description = "BigBrother Distro";

  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixos-generators = {
      url = "github:nix-community/nixos-generators";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    calamares-bb = {
      url = "github:hauskens/calamares-nixos-extensions";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    bigbother-theme = {
      url = "github:BigBotherLinux/kde-theme";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    plasma-manager = {
      url = "github:pjones/plasma-manager";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.home-manager.follows = "home-manager";
    };

    xremap.url = "github:xremap/nix-flake";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      nixos-generators,
      ...
    }@inputs:
    let
      inherit (self) outputs;
      version = "1.9";
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      # nixosModules.bigbotherinstaller = {config, ...}: {
      #   nixpkgs.hostPlatform = system;
      #   imports = [
      #     nixos-generators.nixosModules.all-formats
      #   ];
      #
      #   formatConfigs.isogen = {
      #     config,
      #     modulesPath,
      #     ...
      #   }: {
      #     imports = ["${toString modulesPath}/installer/cd-dvd/installation-cd-graphical-calamares-plasma5.nix"];
      #     isoImage.squashfsCompression = "zstd -Xcompression-level 3";
      #     # Custom iso splash image
      #     isoImage.splashImage = inputs.bigbother-theme + "/images/splashImage.png";
      #     isoImage.efiSplashImage = inputs.bigbother-theme + "/images/splashImage.png";
      #     isoImage.grubTheme = inputs.bigbother-theme + "/grub2-theme";
      #     formatAttr = "isoImage";
      #     fileExtension = ".iso";
      #   };
      # };

      nixosConfigurations.bb = nixpkgs.lib.nixosSystem {
        specialArgs = { inherit inputs self outputs; };
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          nixos-generators.nixosModules.all-formats
        ];
      };

      devShell = forAllSystems (system: import ./shell.nix { pkgs = nixpkgs.legacyPackages.${system}; });
      overlays = import ./overlays.nix { inherit inputs; };
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);
      packages = forAllSystems (system: import ./packages nixpkgs.legacyPackages.${system});
    };
}
