{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    home-manager.url = "github:nix-community/home-manager/release-23.05";
    home-manager.inputs.nixpkgs.follows = "nixpkgs";

    plasma-manager.url = "github:pjones/plasma-manager";
    plasma-manager.inputs.nixpkgs.follows = "nixpkgs";
    plasma-manager.inputs.home-manager.follows = "home-manager";
    nixos-generators = {
      url = "github:nix-community/nixos-generators";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, nixos-generators, ... }:
  with import nixpkgs { system = "x86_64-linux"; };
{
      defaultPackage.x86_64-linux = stdenv.mkDerivation {
        pname = "calamares-nixos-extensions";
        version = "0.3.12";

        src = self;

        installPhase = ''
          runHook preInstall
          mkdir -p $out/{lib,share}/calamares
          cp -r ./calamares/modules $out/lib/calamares/
          cp -r ./calamares/config/* $out/share/calamares/
          cp -r ./calamares/branding $out/share/calamares/
          cp ./calamares/modules/nixos/bigbother-config.nix $out/share/calamares/bigbother-config.nix
          runHook postInstall
        '';

        meta = with lib; {
          description = "Calamares modules for NixOS";
          homepage = "https://github.com/NixOS/calamares-nixos-extensions";
          license = with licenses; [ gpl3Plus bsd2 cc-by-40 cc-by-sa-40 cc0 ];
          maintainers = with maintainers; [ vlinkz ];
          platforms = platforms.linux;
        };
      };
    
    # packages.x86_64-linux = {
    #   iso = nixos-generators.nixosGenerate {
    #     system = "x86_64-linux";
    #     modules = [
    #       # you can include your own nixos configuration here, i.e.
    #       # ./configuration.nix
    #       ./os.nix
    #       (nixpkgs + "/nixos/modules/installer/cd-dvd/installation-cd-graphical-calamares-plasma5.nix")
    #       (nixpkgs + "/nixos/modules/installer/cd-dvd/channel.nix")
    #       #(import ./calamares/modules/nixos/bigbother-config.nix { username = "test1"; fullname = "test1full"; nixversion = "23.05"; pkgs = nixpkgs; })
    #     ];
    #     format = "install-iso";
        

    #     # optional arguments:
    #     # explicit nixpkgs and lib:
    #     # pkgs = nixpkgs.legacyPackages.x86_64-linux;
    #     # lib = nixpkgs.legacyPackages.x86_64-linux.lib;
    #     # additional arguments to pass to modules:
    #     # specialArgs = { myExtraArg = "foobar"; };
        
    #     # you can also define your own custom formats
    #     # customFormats = { "myFormat" = <myFormatModule>; ... };
    #     # format = "myFormat";
    #   };
    # };
  };

  # let
  #   #localCalamares = import ./bigbother-calamares.nix { inherit (pkgs) stdenv lib; };
  #   nixos-version = "23.05";
  # in
  # {


  #   os = {
  #   # nixpkgs.config.packageOverrides = pkgs: {
  #   #   calamares-nixos-extensions = localCalamares;
  #   # };
  #   imports = [
  #     <nixpkgs/nixos/modules/installer/cd-dvd/installation-cd-graphical-calamares-plasma5.nix>
  #     # Provide an initial copy of the NixOS channel so that the user
  #     # doesn't need to run "nix-channel --update" first.
  #     <nixpkgs/nixos/modules/installer/cd-dvd/channel.nix>

  #     # Import the main config for the installer
  #     #(import ./calamares/modules/nixos/bigbother-config.nix { username = "test1"; fullname = "test1full"; nixversion = nixos-version; pkgs = inputs.nixpkgs ; home-manager = inputs.home-manager; })
  #   ];

  #   services.xserver.layout = "no";
  #   isoImage.squashfsCompression = "gzip -Xcompression-level 1";
  #   nixpkgs.config.allowUnfree = true;.
  #   system.stateVersion = nixos-version;
  # };
  # };
}
