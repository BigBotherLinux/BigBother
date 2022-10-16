# BigBother
## An annoying, yet functional linux distribution

The goal of this distribution is to push the limits of an annoying user experience, while still remaining fully functional.
It is based on Ubuntu LTS 22.04 and comes with features you would normally not find in other distributions, such as having **Microsoft Edge** as default browser and **Telemetry enabled** by default wherever possible.
This is not meant to be taken seriously, as this is a learning project.

Hope you have a **misserable** experience! 


## Broad overview of features
- Microsoft Edge as default browser
- Snap is the default package manager
- Accidental boot protection
- Telemetry enabled by default
- Neutral login screen
- Nano is aliased to VIM
- Sudo is customized to insult you on incorrect password attempts

More in depth details about the features are found here

## Getting started
**INSTALL AT YOUR OWN RISK**

Why would you want to do this to yourself? Anyways, to get this installed, check the [github releases](https://github.com/BigBotherLinux/BigBother/releases) for torrent file which will include the ISO. Boot up the ISO, preferably in an Virtual Machine.


## Found something useful?  
Please create issues in the [repository](https://github.com/BigBotherLinux/BigBother/issues) if you have any ideas of how the user experience can deteriorate. [Inspirational comic strip](https://feelafraidcomic.com/60.php)
## Need help?

There is nothing wrong in seeking help, lots of great support can be found [here](https://www.healthygamer.gg/about-us)

## Features 
### Accidental boot protection
This is a custom menu entry in the boot menu that will be default unless something else is chosen.
Failing to choose something else it will automatically shut down the system. This is to prevent the obsure problem of an accidental boot.

### Customizations to the login
The login screen (SDDM) is customized to never remember the username, so the user will have to type both the username and password to log in. This is to avoid leaking personal information IRL. Imagine if someone walked by your locked computer and saw your username..

### KDE Desktop Environment tweaks
- Telemetry is enabled
- Volume slider is in increments of 3. This part only works when using a volume knob or media keys.
- Discover (Application finder) is configured to use **Snap** by default
- Double clicking on the top bar will minimize the window instead of maximizing it.
- Animation speed is increased, which will allow the user time to think before their next action.

### Other customizations
- Sudo is configured to insult you when password is incorrect


## Building from source
The entire build is done from Ansible, the only shell scripts used is for running the chroot environment, as the ansible chroot module is not good enough. 

To build this project, run `ansible-playbook build.yaml --ask-become-pass`.  You may have to install some dependencies on your own, such as **debootstrap**, but that is your problem, not mine.

### Build dependencies
  * debootstrap
  * grub