# Build with nix build .\#nixosConfigurations.bigbotherpc.config.formats.isogen

{
  description = "BigBrother Distro";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    home-manager = {
      url = "github:nix-community/home-manager/release-23.05";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixos-generators = {
      url = "github:nix-community/nixos-generators";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    calamares-bb = {
      url = "github:hauskens/calamares-nixos-extensions";
      #url = "git+file:///home/hausken/Projects/BigBother/calamares"; # for testing calamares changes locally.
    };

    bigbother-theme = {
      url = "github:BigBotherLinux/bigbother-theme";
      #url = "git+file:///home/hausken/Projects/BigBother/bigbother-theme"; # for testing calamares changes locally.
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, nixos-generators, calamares-bb, home-manager, bigbother-theme, ... }: 
  let 
    version = "1.2";
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    nixosModules.bigbotherinstaller = {config, ...}: {
      nixpkgs.hostPlatform = "x86_64-linux";
      imports = [
        nixos-generators.nixosModules.all-formats
      ];
      
      formatConfigs.isogen = {config, modulesPath, ...}: {
        imports = ["${toString modulesPath}/installer/cd-dvd/installation-cd-graphical-calamares-plasma5.nix"];
        isoImage.squashfsCompression = "zstd -Xcompression-level 3";
        formatAttr = "isoImage";
        fileExtension = ".iso";
      };
    };

    # This is the configuration used when calamares installer installs the system.
    nixosConfigurations.bigbotherpc = nixpkgs.lib.nixosSystem {
      modules = [
        ./os.nix
        ./configuration.nix
        home-manager.nixosModules.home-manager
        ({ pkgs, ... }: { 
          home-manager.sharedModules = [ bigbother-theme.homeManagerModules.gust-cursor-theme  ];  

          nixpkgs.config.packageOverrides = localPkgs: {
            # TODO: Find a way to import this in a nicer way...
            gust-cursor-theme = bigbother-theme.packages.${system}.gust-cursor-theme;
          };
        })
      ];    
    };
      
    # This is the configuration used inside the ISO
    nixosConfigurations.bigbotherinstaller = nixpkgs.lib.nixosSystem {
      modules = [
        self.nixosModules.bigbotherinstaller
        ./os.nix
        ./installer.nix
        home-manager.nixosModules.home-manager
        ({ pkgs, ... }: {
            environment.systemPackages = [ 
              calamares-bb.packages.x86_64-linux.calamares-nixos-extensions 
            ];
            home-manager.sharedModules = [ bigbother-theme.homeManagerModules.gust-cursor-theme  ];  
            
            nixpkgs.config.packageOverrides = localPkgs: {
              calamares-nixos-extensions = calamares-bb.packages.x86_64-linux.calamares-nixos-extensions;
              # TODO: Find a way to import this in a nicer way...
              gust-cursor-theme = bigbother-theme.packages.${system}.gust-cursor-theme;
            };

            # Copy the files needed for the installer to the ISO, Calamares will copy these onto the system
            environment.etc."bigbother/os.nix".source = "${self}/os.nix";
            environment.etc."bigbother/flake.nix".source = "${self}/flake.nix";
            environment.etc."bigbother/flake.lock".source = "${self}/flake.lock";
            environment.etc."bigbother/home.nix".source = "${self}/home.nix";
            environment.etc."bigbother/modules".source = "${self}/modules/";
          })
      ];    
    };

    # Generate iso and torrent
    packages.x86_64-linux.makeTorrent = nixpkgs.legacyPackages.x86_64-linux.stdenv.mkDerivation {
      name = "make-torrent";
      buildInputs = [ nixpkgs.legacyPackages.x86_64-linux.mktorrent ];

      src = self.nixosConfigurations.bigbotherinstaller.config.formats.isogen; # Use the output of the bigbotherinstaller as the source
      unpackPhase = "true"; # Don't unpack the iso, we just want to use it as a source

      version = version;
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
        mkdir -p $out/BigBother-v$version-installer-iso
        cp $src $out/BigBother-v$version-installer-iso/BigBother-v$version.iso
        echo "$iso_readme" > $out/BigBother-v$version-installer-iso/no-need-to-readme.txt

        tracker_args=""
        for tracker in ''${trackers[@]}; do
          tracker_args="$tracker_args -a $tracker"
        done

        mktorrent $tracker_args -c "$comment" --name "BigBother-v$version-installer-iso" -o $out/BigBotherv$version.torrent $out/BigBother-v$version-installer-iso
      '';
    };


    # To install this package:
    # TODO: Move this into it's own module
        # ({ pkgs, ... }: {
        #     environment.systemPackages = [ 
        #       self.packages.x86_64-linux.welcome-app
        #     ];
        #     # Autostart configuration
        #     services.xserver.displayManager.sessionCommands = pkgs.lib.mkForce ''
        #       mkdir -p $HOME/.config/autostart
        #       cat >$HOME/.config/autostart/welcome-app.desktop <<EOF
        #       [Desktop Entry]
        #       Type=Application
        #       Exec=${self.packages.x86_64-linux.welcome-app}/bin/welcome-app
        #       Name=Welcome App
        #       Comment=Welcome screen for BigBrother Distro
        #       EOF
        #     '';
        #   })

    packages.x86_64-linux.welcome-app = pkgs.python3Packages.buildPythonApplication rec {
      pname = "welcome-app";
      version = "1.0";

      src = ./welcome-screen;
      
      propagatedBuildInputs = with pkgs.python3Packages; [
          pyqt5
          pkgs.qt5.qtbase 
          pkgs.qt5.qtx11extras 
          pkgs.qt5.qtwayland
          
        ];
      dontWrapQtApps = true; # Prevent double-wrapping

      nativeBuildInputs = with pkgs; [
        qt5.wrapQtAppsHook  # This hook automatically wraps Qt applications
      ];

      installPhase = ''
        mkdir -p $out/bin
        mkdir -p $out/static
        cp main.py $out/bin/${pname}
        cp logo.png $out/static/logo.png
        chmod +x $out/bin/${pname}
      '';
      postInstall = ''
        wrapQtApp $out/bin/${pname}
      '';
    };

    
  };
}
