{ pkgs }:

{
  testBB = pkgs.writeShellScriptBin "testBB" ''
    if [ -f "/dev/shm/bigbother.qcow2" ]; then
      echo "Removing old image"
      rm -f /dev/shm/bigbother.qcow2
    fi
    nix build .\#nixosConfigurations.bb.config.formats.vm && ./result/run-bigbother-vm
  '';

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

  makeRelease = pkgs.writeShellScriptBin "makeRelease" ''
    set -euo pipefail

    # Ask for version
    read -rp "Enter version number (e.g. 2.0): " VERSION
    if [ -z "$VERSION" ]; then
      echo "Error: Version cannot be empty"
      exit 1
    fi

    DIST_DIR="dist"
    OUTPUT_DIR="$DIST_DIR/BigBother-v''${VERSION}-installer-iso"
    ISO_NAME="BigBother-v''${VERSION}.iso"
    TORRENT_NAME="$DIST_DIR/BigBotherv''${VERSION}.torrent"

    echo "==> Building ISO..."
    nix build .#nixosConfigurations.bb-iso.config.system.build.isoImage

    # Find the built ISO
    ISO_PATH=$(find result/iso/ -name "*.iso" -type l -o -name "*.iso" -type f | head -n1)
    if [ -z "$ISO_PATH" ]; then
      echo "Error: No ISO found in result/iso/"
      exit 1
    fi
    echo "==> Found ISO: $ISO_PATH"

    # Create output directory and copy ISO
    mkdir -p "$OUTPUT_DIR"
    cp "$ISO_PATH" "$OUTPUT_DIR/$ISO_NAME"
    echo "This is an iso for the Linux distro BigBother, read more about it here: https://github.com/BigBotherLinux/BigBother" \
      > "$OUTPUT_DIR/no-need-to-readme.txt"

    echo "==> Creating torrent..."
    ${pkgs.mktorrent}/bin/mktorrent \
      -a udp://fosstorrents.com:6969/announce \
      -a http://fosstorrents.com:6969/announce \
      -a udp://tracker.opentrackr.org:1337/announce \
      -a udp://tracker.openbittorrent.com:6969/announce \
      -a http://tracker.openbittorrent.com:80/announce \
      -a udp://93.158.213.92:1337/announce \
      -c "BigBother Linux distro <https://github.com/BigBotherLinux/BigBother>" \
      --name "BigBother-v''${VERSION}-installer-iso" \
      -o "$TORRENT_NAME" \
      "$OUTPUT_DIR"

    echo ""
    echo "==> Done!"
    echo "    ISO:     $OUTPUT_DIR/$ISO_NAME"
    echo "    Torrent: $TORRENT_NAME"
  '';
}
