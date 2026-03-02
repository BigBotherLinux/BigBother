{ ... }:
{
  projectRootFile = "flake.nix";

  settings.global.excludes = [ ".github/ISSUE_TEMPLATE/**" ];

  programs = {
    nixfmt.enable = true;
    shfmt.enable = true;
    rustfmt.enable = true;
    just.enable = true;
    mdformat.enable = true;
  };
}
