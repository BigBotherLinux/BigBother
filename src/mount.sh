#!/bin/bash
# Mounts chroot dirs needed
function mountdirs()
{
    mount -t proc none /proc
    mount -t sysfs none /sys
    mount -t devpts none /dev/pts
}

mountdirs
# >> /src/log/mount.txt