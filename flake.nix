# Build with nix build .\#nixosConfigurations.bigbotherpc.config.formats.isogen

{
  description = "BigBrother Distro";

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

  outputs = { self, nixpkgs, nixos-generators, calamares-bb, home-manager, plasma-manager, ... }: {


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
          #home-manager.sharedModules = [ plasma-manager.homeManagerModules.plasma-manager ];
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
          #home-manager.sharedModules = [ plasma-manager.homeManagerModules.plasma-manager ];
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
    packages.x86_64-linux.makeTorrent = nixpkgs.legacyPackages.x86_64-linux.stdenv.mkDerivation {
    name = "make-torrent";
    buildInputs = [ nixpkgs.legacyPackages.x86_64-linux.mktorrent ];

    # Use the output of the bigbotherinstaller as the source
    src = self.nixosConfigurations.bigbotherinstaller.config.formats.isogen;

    unpackPhase = "true";

    version = "1.1";
    trackers = [ 
      "udp://fosstorrents.com:6969/announce" 
      "http://fosstorrents.com:6969/announce" 
      "udp://tracker.opentrackr.org:1337/announce"
      "udp://tracker.openbittorrent.com:6969/announce"
      "http://tracker.openbittorrent.com:80/announce"
      "udp://93.158.213.92:1337/announce"
      ];
    comment ="BigBother Linux distro <https://github.com/BigBotherLinux/BigBother>";
    iso_readme = "This is an iso for the Linux distro BigBother, read more about it here: https://github.com/BigBotherLinux/BigBother";


    installPhase = ''
      mkdir -p $out
      cp $src $out/BigBother-v$version.iso
      echo "$iso_readme" > $out/no-need-to-readme.txt

      tracker_args=""
      for tracker in ''${trackers[@]}; do
        tracker_args="$tracker_args -a $tracker"
      done

      mktorrent $tracker_args -c "$comment" --name "BigBother v$version installer iso" -o $out/BigBotherv$version.torrent $out
    '';
  };
  };
}
