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
    bun2nix.url = "github:nix-community/bun2nix";
  bun2nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      nixos-generators,
      crane,
      bun2nix,
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

      # POC Installer ISO - minimal, auto-starts bb-installer
      nixosConfigurations.bb-installer-poc = nixpkgs.lib.nixosSystem {
        specialArgs = { inherit inputs self outputs; };
        system = "x86_64-linux";
        modules = [
          ./modules/installer-iso-poc.nix
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

          testUefiScript = pkgs.writeShellScriptBin "testBB-uefi" ''
            DISK_IMAGE=""
            ISO_PATH=""
            BUILD_ISO=false

            # Parse arguments
            while [[ $# -gt 0 ]]; do
              case $1 in
                -iso)
                  if [[ -n "$2" && "$2" != -* ]]; then
                    # Path provided after -iso
                    ISO_PATH="$2"
                    shift 2
                  else
                    # No path, build the ISO
                    BUILD_ISO=true
                    shift
                  fi
                  ;;
                *)
                  DISK_IMAGE="$1"
                  shift
                  ;;
              esac
            done

            # Default disk image if not specified
            DISK_IMAGE="''${DISK_IMAGE:-test-disk.qcow2}"

            # Build ISO if -iso flag was provided without a path
            if [ "$BUILD_ISO" = true ]; then
              echo "Building installer ISO..."
              nix build .#nixosConfigurations.bb-installer-poc.config.system.build.isoImage
              ISO_PATH="./result/iso/bigbother-poc.iso"
            fi

            echo "Starting QEMU with UEFI firmware..."
            echo "Disk image: $DISK_IMAGE"
            [ -n "$ISO_PATH" ] && echo "ISO image: $ISO_PATH"

            # Build QEMU command
            QEMU_ARGS=(
              -enable-kvm
              -m 4G
              -smp 2
              -bios ${pkgs.OVMF.fd}/FV/OVMF.fd
              -vga virtio
              -display gtk
              -usb
              -device usb-tablet
              -device virtio-keyboard-pci
            )

            # Configure drives with boot priority
            if [ -n "$ISO_PATH" ]; then
              # ISO gets bootindex=0 (highest priority), HDD gets bootindex=1
              QEMU_ARGS+=(
                -drive file="$ISO_PATH",id=cdrom,media=cdrom,readonly=on,if=none
                -device ide-cd,drive=cdrom,bootindex=0
                -drive file="$DISK_IMAGE",id=hdd,format=qcow2,if=none
                -device virtio-blk-pci,drive=hdd,bootindex=1
              )
            else
              # No ISO, just the HDD
              QEMU_ARGS+=(-drive file="$DISK_IMAGE",format=qcow2,if=virtio)
            fi

            ${pkgs.qemu}/bin/qemu-system-x86_64 "''${QEMU_ARGS[@]}"
          '';
        in {
          default = craneLib.devShell {
            # Checks to run in the dev shell
            checks = self.checks.${system};

            # Additional packages for development
            packages = with pkgs; [
              testScript
              testUefiScript
              rustToolchain
              cargo-watch
              cargo-edit
              qemu
              OVMF
              just
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
        (import ./packages { pkgs = legacyPkgs; bun2nix = bun2nix.packages.${system}.default; }) // {
          # bb-installer package (crane-built)
          bb-installer = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
            # Wrap the binary with runtime dependencies
            postInstall = ''
              # Copy all .nix files from the repo for installation
              mkdir -p $out/share/bb-flake
              cd ${./.}
              find . -name "*.nix" -type f ! -path "*/target/*" ! -path "*/.git/*" -exec sh -c '
                mkdir -p "$out/share/bb-flake/$(dirname "$1")"
                cp "$1" "$out/share/bb-flake/$1"
              ' _ {} \;

              # Copy flake.lock if it exists
              if [ -f flake.lock ]; then
                cp flake.lock $out/share/bb-flake/
              fi

              wrapProgram $out/bin/bb-installer \
                --prefix PATH : ${pkgs.lib.makeBinPath (with pkgs; [
                  parted
                  util-linux
                  e2fsprogs
                  dosfstools
                  nixos-install-tools
                  mkpasswd
                ])} \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath (mkBuildInputs pkgs)} \
                --set BB_FLAKE_PATH $out/share/bb-flake
            '';
            nativeBuildInputs = (mkNativeBuildInputs pkgs) ++ [ pkgs.makeWrapper ];
          });
        }
      );
    };
}
