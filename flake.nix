{
  description = "BigBrother Distro";

  inputs = {
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
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      nixos-generators,
      crane,
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

      # Crane setup for Rust development
      mkCraneLib = system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };
        in
        (crane.mkLib pkgs).overrideToolchain rustToolchain;

      # Build inputs needed for bb-installer (GUI app)
      mkBuildInputs = pkgs: with pkgs; [
        fontconfig
        freetype
        libxkbcommon
        libGL
        wayland
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libxcb
      ];

      mkNativeBuildInputs = pkgs: with pkgs; [
        pkg-config
      ];

      # Common args for crane builds
      mkCommonArgs = system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          craneLib = mkCraneLib system;
        in
        {
          src = craneLib.cleanCargoSource ./bb-installer;
          strictDeps = true;
          buildInputs = mkBuildInputs pkgs;
          nativeBuildInputs = mkNativeBuildInputs pkgs;
        };
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

      # BigBother Installer ISO
      nixosConfigurations.bb-iso = nixpkgs.lib.nixosSystem {
        specialArgs = { inherit inputs self outputs; };
        system = "x86_64-linux";
        modules = [
          ./modules/installer-iso.nix
        ];
      };

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          craneLib = mkCraneLib system;
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };
          commonArgs = mkCommonArgs system;

          # Build deps only (for caching)
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          testScript = pkgs.writeShellScriptBin "testBB" ''
            if [ -f "/dev/shm/bigbother.qcow2" ]; then
              echo "Removing old image"
              rm -f /dev/shm/bigbother.qcow2
            fi
            nix build .\#nixosConfigurations.bb.config.formats.vm && ./result/run-bigbother-vm
          '';
        in {
          default = craneLib.devShell {
            # Checks to run in the dev shell
            checks = self.checks.${system};

            # Additional packages for development
            packages = with pkgs; [
              testScript
              rustToolchain
              cargo-watch
              cargo-edit
            ];

            # Set library paths for GUI development
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (mkBuildInputs pkgs);

            LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";
          };
        }
      );

      # Crane-based checks for CI
      checks = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          craneLib = mkCraneLib system;
          commonArgs = mkCommonArgs system;
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in {
          # Build the crate as part of checks
          bb-installer = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });

          # Run clippy
          bb-installer-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          # Check formatting
          bb-installer-fmt = craneLib.cargoFmt {
            src = craneLib.cleanCargoSource ./bb-installer;
          };
        }
      );

      overlays = import ./overlays.nix { inherit inputs; };
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          craneLib = mkCraneLib system;
          commonArgs = mkCommonArgs system;
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          legacyPkgs = nixpkgs.legacyPackages.${system};
        in
        (import ./packages legacyPkgs) // {
          # Crane-built bb-installer
          bb-installer-crane = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
            # Wrap the binary with runtime dependencies
            postInstall = ''
              wrapProgram $out/bin/bb-installer \
                --prefix PATH : ${pkgs.lib.makeBinPath (with pkgs; [
                  parted
                  util-linux
                  e2fsprogs
                  dosfstools
                  nixos-install-tools
                  mkpasswd
                ])} \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath (mkBuildInputs pkgs)}
            '';
            nativeBuildInputs = (mkNativeBuildInputs pkgs) ++ [ pkgs.makeWrapper ];
          });
        }
      );
    };
}
