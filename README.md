# BigBother
## An annoying, yet functional linux distribution

The goal of this distribution is to push the limits of an annoying user experience, while still remaining fully functional.
It is based on Ubuntu LTS 22.04 and comes with features you would normally not find in other distributions, such as having **Microsoft Edge** as default browser and **Telemetry enabled** by default wherever possible.

Hope you have a misserable experience! 


## Broad overview of features
- KDE desktop environment
- Microsoft Edge as default browser
- Snap is the default package manager
- Accidental boot protection
- Telemetry enabled by default
- Custom login screen
- Nano is aliased to VIM
- Calamares installer
- Custom sudo config

## Getting started
**INSTALL AT YOUR OWN RISK**

Why would you want to do this to yourself? Anyways, to get this installed, check the github releases for torrent file which will include the ISO. Boot up the ISO, preferably in an Virtual Machine.

## Found something useful?  
Please create issues in the repository if you have any ideas of how the user experience can deteriorate.
## Building from source
The entire build is done from Ansible, the only shell scripts used is for running the chroot environment, as the ansible chroot module is not good enough. 

To build this project, run `ansible-playbook build.yaml` with root privileges. You may have to install some dependencies on your own, such as **debootstrap**, but that is your problem, not mine.
