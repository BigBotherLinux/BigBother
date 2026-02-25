{ pkgs }:

{
  testBB-uefi = pkgs.writeShellScriptBin "testBB-uefi" ''
    DISK_IMAGE=""
    ISO_PATH=""
    BUILD_ISO=false

    # Parse arguments
    while [[ $# -gt 0 ]]; do
      case $1 in
        -iso)
          if [[ -n "$2" && "$2" != -* ]]; then
            # Path provided after -iso
            ISO_PATH="$2"
            shift 2
          else
            # No path, build the ISO
            BUILD_ISO=true
            shift
          fi
          ;;
        *)
          DISK_IMAGE="$1"
          shift
          ;;
      esac
    done

    # Default disk image if not specified
    DISK_IMAGE="''${DISK_IMAGE:-test-disk.qcow2}"

    # Build ISO if -iso flag was provided without a path
    if [ "$BUILD_ISO" = true ]; then
      echo "Building installer ISO..."
      nix build .#nixosConfigurations.bb-installer.config.system.build.isoImage
      ISO_PATH="./result/iso/bigbother-poc.iso"
    fi

    echo "Starting QEMU with UEFI firmware..."
    echo "Disk image: $DISK_IMAGE"
    [ -n "$ISO_PATH" ] && echo "ISO image: $ISO_PATH"

    # Build QEMU command
    QEMU_ARGS=(
      -enable-kvm
      -m 4G
      -smp 2
      -bios ${pkgs.OVMF.fd}/FV/OVMF.fd
      -vga virtio
      -display gtk
      -usb
      -device usb-tablet
      -device virtio-keyboard-pci
    )

    # Configure drives with boot priority
    if [ -n "$ISO_PATH" ]; then
      # ISO gets bootindex=0 (highest priority), HDD gets bootindex=1
      QEMU_ARGS+=(
        -drive file="$ISO_PATH",id=cdrom,media=cdrom,readonly=on,if=none
        -device ide-cd,drive=cdrom,bootindex=0
        -drive file="$DISK_IMAGE",id=hdd,format=qcow2,if=none
        -device virtio-blk-pci,drive=hdd,bootindex=1
      )
    else
      # No ISO, just the HDD
      QEMU_ARGS+=(-drive file="$DISK_IMAGE",format=qcow2,if=virtio)
    fi

    ${pkgs.qemu}/bin/qemu-system-x86_64 "''${QEMU_ARGS[@]}"
  '';
}
