{
  inputs,
  system,
  self,
  treefmt,
  ...
}:
let
  crane = import ./lib/crane.nix { inherit inputs system; };
  inherit (crane)
    pkgs
    craneLib
    commonArgs
    cargoArtifacts
    ;

  # Ensure the ISO closure contains no unfree packages (x86_64-linux only).
  isoUnfreeCheck =
    if system == "x86_64-linux" then
      {
        iso-no-unfree = builtins.seq self.nixosConfigurations.bb-iso.config.system.build.toplevel.drvPath (
          pkgs.runCommand "iso-no-unfree" { } "touch $out"
        );

        iso-installer-starts = pkgs.callPackage ./tests/iso-installer.nix { };
      }
    else
      { };
in
isoUnfreeCheck
// {
  bb-installer = craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;
    }
  );

  bb-installer-clippy = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets -- --deny warnings";
    }
  );

  bb-installer-fmt = craneLib.cargoFmt {
    src = craneLib.cleanCargoSource ./bb-installer;
  };

  # Pre-commit hooks check
  pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
    src = ./.;
    default_stages = [ "pre-commit" ];
    hooks = {
      # ========== General ==========
      check-added-large-files = {
        enable = true;
        excludes = [
          "\\.png"
          "\\.jpg"
          "\\.github/ISSUE_TEMPLATE/.*"
        ];
      };
      check-case-conflicts.enable = true;
      check-executables-have-shebangs.enable = true;
      check-shebang-scripts-are-executable.enable = false;
      check-merge-conflicts.enable = true;
      detect-private-keys.enable = true;
      fix-byte-order-marker.enable = true;
      mixed-line-endings.enable = true;
      trim-trailing-whitespace.enable = true;

      forbid-submodules = {
        enable = true;
        name = "forbid submodules";
        description = "forbids any submodules in the repository";
        language = "fail";
        entry = "submodules are not allowed in this repository:";
        types = [ "directory" ];
      };

      deadnix = {
        enable = true;
        excludes = [ "packages/.*-bun\\.nix$" ];
        settings = {
          noLambdaArg = true;
          exclude = [
            "packages/incel-bun.nix"
            "packages/werd-bun.nix"
          ];
        };
      };

      # ========== formatting ==========
      treefmt = {
        enable = true;
        package = treefmt;
      };

    };
  };
}
