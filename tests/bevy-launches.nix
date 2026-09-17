# Boots a VM with the bevy installer kiosk and proves the app reached the
# renderer: it prints BB_BEVY_READY on its first rendered frames.
#
# The VM has no GPU, so this runs on mesa's lavapipe (software Vulkan).
{ pkgs, ... }:
pkgs.testers.runNixOSTest {
  name = "bevy-launches";

  nodes.machine = {
    imports = [ ../modules/bb-installer-bevy.nix ];

    bigbother.bb-installer-bevy = {
      enable = true;
      softwareRendering = true;
    };

    virtualisation.memorySize = 4096;
    virtualisation.cores = 4;
    # Match how testBB-uefi boots the ISO, so the compositor sees the same GPU.
    virtualisation.qemu.options = [ "-vga virtio" ];
  };

  testScript = ''
    machine.start()
    machine.wait_for_unit("bb-installer-bevy-cage.service")
    # Software rasterising the first frames is slow, so allow a generous window.
    machine.wait_until_succeeds(
        "journalctl -u bb-installer-bevy-cage.service | grep -q BB_BEVY_READY", timeout=180
    )
  '';
}
