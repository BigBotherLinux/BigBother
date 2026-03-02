{ pkgs, ... }:
pkgs.testers.runNixOSTest {
  name = "iso-installer-starts";

  nodes.machine = {
    imports = [ ../modules/bb-installer.nix ];

    hardware.graphics.enable = true;
    bigbother.bb-installer.enable = true;
  };

  testScript = ''
    machine.wait_for_unit("bb-installer-cage.service")
  '';
}
