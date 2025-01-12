{
  pkgs ? import <nixpkgs> { },
}:
let
  testScript = pkgs.writeShellScriptBin "testBB" ''
    if [ -f "/dev/shm/bigbother.qcow2" ]; then
      echo "Removing old image"
      rm -f /dev/shm/bigbother.qcow2
    fi
    nix build .\#nixosConfigurations.bb.config.formats.vm && ./result/run-bigbother-vm
  '';
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    testScript
  ];

  # shellHook = ''
  #   # ...
  #   alias ,vb='nix build .\#nixosConfigurations.bb.config.formats.vm && ./result/run-bigbother-vm || rm /dev/shm/bigbother.qcow2'
  #
  # '';
}
