#!/bin/bash
function setenvironment()
{
    export HOME=/root
    #export LC_ALL=C
    echo "hello"
    #pacman -Sy ansible --noconfirm
    ansible-playbook /src/test.yaml 2>&1
}

function prerequisite()
{
    echo "########################################"
    echo "Installing prerequisites:"
    apt update
    apt install ansible -y
}

function runAnsible()
{
    ansible-playbook /src/prepare_distro.yaml
}

function listInstalledPackages()
{
    echo "########################################"
    echo "Listing packages:"
    dpkg-query -W --showformat='${Installed-Size}\t${Package}\n' | sort -nr | less
}

function removePackages()
{
    echo "########################################"
    echo "Removing packages: "$1
    apt clean
    apt purge $1 -y
}


function cleanup()
{
    echo "########################################"
    echo "Running cleanup"
    dpkg-divert --rename --remove /sbin/initctl
    dpkg-query -W --showformat='${Package} ${Version}\n' > /src/filesystem.manifest
    cp -v /src/filesystem.manifest /src/filesystem.manifest-desktop
    sed -i '/casper/d' /src/filesystem.manifest-desktop
    sed -i '/discover/d' /src/filesystem.manifest-desktop
    sed -i '/laptop-detect/d' /src/filesystem.manifest-desktop
    sed -i '/os-prober/d' /src/filesystem.manifest-desktop
}


function main()
{
    setenvironment
    #prerequisite
    #runAnsible
    #cleanup
    echo "done"
}

main