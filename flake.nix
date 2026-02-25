{
  description = "BigBrother Distro";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixos-generators = {
      url = "github:nix-community/nixos-generators";
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

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";

    bun2nix = {
      url = "github:nix-community/bun2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    {
      self,
      nixpkgs,
      nixos-generators,
      bun2nix,
      ...
    }@inputs:
    let
      inherit (self) outputs;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;

      # Treefmt evaluation for formatting checks
      treefmtEval = forAllSystems (
        system: inputs.treefmt-nix.lib.evalModule nixpkgs.legacyPackages.${system} ./treefmt.nix
      );
    in
    {

      # Main configuration used by the installer
      nixosConfigurations.bb = nixpkgs.lib.nixosSystem {
        specialArgs = { inherit inputs self outputs; };
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          nixos-generators.nixosModules.all-formats
        ];
      };

      # BigBother Installer ISO
      nixosConfigurations.bb-iso = nixpkgs.lib.nixosSystem {
        specialArgs = { inherit inputs self outputs; };
        system = "x86_64-linux";
        modules = [
          ./installer-iso.nix
        ];
      };

      devShells = forAllSystems (system: import ./devShells.nix { inherit inputs system self; });

      # Checks for CI (imported from checks.nix)
      checks = forAllSystems (
        system:
        import ./checks.nix {
          inherit inputs system self;
          treefmt = treefmtEval.${system}.config.build.wrapper;
        }
      );

      overlays = import ./overlays.nix { inherit inputs; };
      formatter = forAllSystems (system: treefmtEval.${system}.config.build.wrapper);

      packages = forAllSystems (
        system:
        let
          crane = import ./lib/crane.nix { inherit inputs system; };
          legacyPkgs = import nixpkgs {
            inherit system;
            config.allowUnfree = true;
          };
        in
        import ./packages {
          pkgs = legacyPkgs;
          bun2nix = bun2nix.packages.${system}.default;
          inherit (crane) craneLib commonArgs cargoArtifacts;
          cranePkgs = crane.pkgs;
        }
      );
    };
}
