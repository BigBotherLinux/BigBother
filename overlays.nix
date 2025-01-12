{ inputs, ... }:
rec {
  additions = final: _prev: import ./packages final.pkgs;

  modifications = _final: prev: {
    # python312 = prev.python312.override {
    #   packageOverrides = _python-self: python-super: {
    #     faster-whisper = python-super.faster-whisper.overridePythonAttrs (_oldAttrs: {
    #       pythonRelaxDeps = [
    #         "tokenizers"
    #         "av"
    #       ];
    #     });
    #     opentelemetry-proto = python-super.opentelemetry-proto.overridePythonAttrs (_oldAttrs: {
    #       pythonRelaxDeps = [ "protobuf" ];
    #     });
    #   };
    # };
  };

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
