# Build with nix build .\?submodules=1\#iso

{
  description = "BigBrother NixOS ISO";

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

  outputs = { self, nixpkgs, nixos-generators, ... }@inputs:
  with import nixpkgs { system = "x86_64-linux"; config.allowUnfree = true; isoImage.squashfsCompression = "gzip -Xcompression-level 1";};
{
    packages.x86_64-linux = {
      ## Custom calamares build
      bigbother_calamares = stdenv.mkDerivation {
        name = "calamares-nixos-extensions";
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
          cp ./calamares/modules/nixos/flake.nix $out/share/calamares/flake.nix
          cp ./calamares/modules/nixos/flake.lock $out/share/calamares/flake.lock
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
      
      nixosConfigurations.nixpc = 
       {
        bigbotherpc = nixpkgs.lib.nixosSystem {
          specialArgs = {inherit inputs system;};
          modules = [
            ./calamares/modules/nixos/bigbother-config.nix
            #./os.nix
          ];
        };
      };

      ## Iso generation
      iso = 
      let 
        custom_calamares = self.packages.x86_64-linux.bigbother_calamares;
      in 
      nixos-generators.nixosGenerate {
        system = "x86_64-linux";
        modules = [
          (nixpkgs + "/nixos/modules/installer/cd-dvd/installation-cd-graphical-calamares-plasma5.nix")
          (nixpkgs + "/nixos/modules/installer/cd-dvd/channel.nix")
          ({ pkgs, ... }: {
            environment.systemPackages = [ custom_calamares ];
            nixpkgs.config.packageOverrides = localPkgs: {
              calamares-nixos-extensions = custom_calamares;
            };
          })
          (import ./calamares/modules/nixos/bigbother-config.nix { inherit pkgs; })
          #(import ./os.nix { inherit pkgs config; })
        ];
 
        format = "install-iso";
      };
    };
  };
}
