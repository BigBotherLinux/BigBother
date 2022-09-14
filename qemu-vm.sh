#!/bin/bash
# this just starts a VM with the created ISO
qemu-system-x86_64 -cdrom ./build/BigBother.iso -boot order=d -m 8G -cpu host -enable-kvm -smp 4 -device virtio-net,netdev=vmnic -netdev user,id=vmnic -hda /home/hausken/VM/ubuntu.qcow2