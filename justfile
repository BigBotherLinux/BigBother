# Update bun.nix lock files for bun2nix packages

update-incel-bun:
    #!/usr/bin/env bash
    set -euo pipefail
    src=$(nix eval --raw .#incel.src)
    nix run github:nix-community/bun2nix -- \
        --lock-file "$src/bun.lock" \
        --output-file packages/incel-bun.nix

update-werd-bun:
    #!/usr/bin/env bash
    set -euo pipefail
    src=$(nix eval --raw .#werd.src)
    nix run github:nix-community/bun2nix -- \
        --lock-file "$src/bun.lock" \
        --output-file packages/werd-bun.nix

update-bun: update-incel-bun update-werd-bun

test-vm:
    nix build .#nixosConfigurations.bb.config.formats.vm && ./result/run-bigbother-vm

# Build the installer ISO and boot it under UEFI, installing onto DISK
test-iso disk="test-disk.qcow2":
    #!/usr/bin/env bash
    set -euo pipefail
    # Once installed, boot that disk without the ISO by running: testBB-uefi
    # testBB-uefi lives in the default dev shell; fall back to it when running
    # from outside.
    if command -v testBB-uefi >/dev/null; then
      testBB-uefi "{{ disk }}" -iso
    else
      nix develop -c testBB-uefi "{{ disk }}" -iso
    fi

# Run the bevy prototype locally, with dynamic linking for fast rebuilds
dev-bevy *args:
    #!/usr/bin/env bash
    set -euo pipefail
    # dynamic_linking is a dev-only flag: it makes cargo link bevy as a shared
    # object, which only resolves when cargo itself launches the binary.
    run=(cargo run --manifest-path bb-installer-bevy/Cargo.toml --features bevy/dynamic_linking {{ args }})
    if [ -n "${BB_BEVY_SHELL:-}" ]; then
      "${run[@]}"
    else
      nix develop .#bevy -c "${run[@]}"
    fi

# Build the bevy variant of the ISO and boot it under UEFI
test-iso-bevy disk="test-disk.qcow2":
    #!/usr/bin/env bash
    set -euo pipefail
    nix build .#nixosConfigurations.bb-iso-bevy.config.system.build.isoImage
    iso=$(echo ./result/iso/*.iso)
    if command -v testBB-uefi >/dev/null; then
      testBB-uefi "{{ disk }}" -iso "$iso"
    else
      nix develop -c testBB-uefi "{{ disk }}" -iso "$iso"
    fi
