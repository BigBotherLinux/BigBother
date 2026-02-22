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
