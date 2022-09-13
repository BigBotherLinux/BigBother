# BigBother
An Ubuntu based distribution made to be an annoyance to the user, while still being a fully usable distro. It is likely the first Linux distribution with Microsoft Edge as default browser.

## Features
 - KDE desktop environment
 - Microsoft Edge as default browser
 - Snap is the default package manager
 - Accidental boot protection
 - Telemetry enabled by default
 - Custom login screen
 - Custom sudo config
 

## Technical details
The entire build is done from ansible, the only shell scipts used is for running the chroot environment, as the ansible chroot module is not good enough.
Calamares
