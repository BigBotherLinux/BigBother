{
  inputs,
  system,
  self,
}:
let
  crane = import ./lib/crane.nix { inherit inputs system; };
  inherit (crane)
    pkgs
    rustToolchain
    craneLib
    buildInputs
    ;

  scripts = import ./scripts.nix { inherit pkgs; };
in
{
  default = craneLib.devShell {
    checks = self.checks.${system};

    packages =
      (builtins.attrValues scripts)
      ++ (with pkgs; [
        rustToolchain
        cargo-watch
        cargo-edit
        qemu
        OVMF
        just
      ]);

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";
  };
}
