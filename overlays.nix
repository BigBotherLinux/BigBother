{ inputs, ... }:
rec {
  additions =
    final: _prev:
    import ./packages {
      pkgs = final;
      bun2nix = final.inputs.bun2nix.default;
    };

  modifications = _final: prev: { };

  flake-inputs = final: _: {
    inputs = builtins.mapAttrs (
      _: flake:
      let
        legacyPackages = (flake.legacyPackages or { }).${final.system} or { };
        packages = (flake.packages or { }).${final.system} or { };
      in
      if legacyPackages != { } then legacyPackages else packages
    ) inputs;
  };

  stable-packages = final: _prev: {
    stable = import inputs.nixpkgs-stable {
      inherit (final) system;
      config.allowUnfree = true;
      config.overlays = [ modifications ];
    };
  };
}
