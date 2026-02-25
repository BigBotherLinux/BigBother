{ pkgs }:

{
  testBB-uefi = pkgs.writeShellScriptBin "testBB-uefi" ''
    DISK_IMAGE=""
    ISO_PATH=""
    BUILD_ISO=false

    while [[ $# -gt 0 ]]; do
      case $1 in
        -iso)
          if [[ -n "$2" && "$2" != -* ]]; then
            ISO_PATH="$2"
            shift 2
          else
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

    DISK_IMAGE="''${DISK_IMAGE:-test-disk.qcow2}"

    if [ "$BUILD_ISO" = true ]; then
      echo "Building installer ISO..."
      nix build .#nixosConfigurations.bb-iso.config.system.build.isoImage
      ISO_PATH="./result/iso/bigbother.iso"
    fi

    if [ ! -f "$DISK_IMAGE" ]; then
      echo "Disk image not found, creating $DISK_IMAGE..."
      ${pkgs.qemu}/bin/qemu-img create -f qcow2 "$DISK_IMAGE" 20G
    fi

    echo "Starting QEMU with UEFI firmware..."
    echo "  Disk: $DISK_IMAGE"
    [ -n "$ISO_PATH" ] && echo "  ISO:  $ISO_PATH"

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

    if [ -n "$ISO_PATH" ]; then
      QEMU_ARGS+=(
        -drive file="$ISO_PATH",id=cdrom,media=cdrom,readonly=on,if=none
        -device ide-cd,drive=cdrom,bootindex=0
        -drive file="$DISK_IMAGE",id=hdd,format=qcow2,if=none
        -device virtio-blk-pci,drive=hdd,bootindex=1
      )
    else
      QEMU_ARGS+=(-drive file="$DISK_IMAGE",format=qcow2,if=virtio)
    fi

    ${pkgs.qemu}/bin/qemu-system-x86_64 "''${QEMU_ARGS[@]}"
  '';
}
