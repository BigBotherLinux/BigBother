{ ... }:
{
  projectRootFile = "flake.nix";

  programs = {
    nixfmt.enable = true;
    shfmt.enable = true;
    rustfmt.enable = true;
    just.enable = true;
    mdformat.enable = true;
  };
}
