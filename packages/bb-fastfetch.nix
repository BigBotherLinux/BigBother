{ pkgs, ... }:

let
  sadcat = ./fastfetch/sadcat.txt;
  config = pkgs.replaceVars ./fastfetch/config.jsonc {
    sadcat = sadcat;
  };
in
pkgs.writeShellScriptBin "fastfetch" ''
  exec ${pkgs.fastfetch}/bin/fastfetch --config ${config} "$@"
''
