# BigBother
## An annoying, yet functional linux distribution

The goal of this distribution is to push the limits of an annoying user experience, while still remaining fully functional.

It is based on NixOS using flakes and comes with features you would normally not find in other distributions, such as having **Microsoft Edge** as default browser and genuinely inconvenient features.

## Why use this distro?

The simple answer is: **Don't.**

If you are still not conviced, here are some of the features in this distro:

- Microsoft Edge as default browser
- [Incel](https://github.com/kluzzebass/incel) and [Werd](https://github.com/kluzzebass/werd) preinstalled
- [Adboost extension](https://github.com/surprisetalk/AdBoost) preloaded in Microsoft Edge
- Default system font [only supports lowercase letters](#system-font)
- The cursor's active click point has been [shifted](#cursor)
- Cursor will [slowly drift](#trackpoint-drift-simulation) in a direction
- [Accidental boot protection](#accidental-boot-protection)
- Steep learning curve if you want install or update the system
- Telemetry enabled by default
- [Login screen](#customizations-to-the-login-screen) configured to not remember username
- Sudo is customized to insult you on incorrect password attempts
- Nano is aliased to VIM

Much more in depth details about the features are found [here](#features)

## Full Feature List
⚠️ Trigger warning ⚠️ 

### Wallpaper
Wallpaper named "Crowded" is included, it scales to most screen resolutions, so the cursor background should be the same size as the user's cursor regardless of the user's screen size.

![Wallpaper Preview](images/wallpaper-preview.png)

### Accidental boot protection
This is a custom menu entry in the boot menu that will be default unless something else is chosen.

Failing to choose something else within 5 sec it will automatically shut down the system.

A cognitive test and a mandatory breathing exercise is also included to ensure you are a concious and willing participant in using the system.

![Accidentail boot protection](images/boot-protection.png)

### The login screen
The login screen SDDM theme is customized:
- Will never remember your username
- Passwords are obviously not something you should hide from someone
- A watchful eye is will ensure you dont hide anything

![Login screen](images/login-screen.png)

### TrackPoint drift
We simulate a slight drift in the mouse cursor to make it feel like the good old days using a TrackPoint.

![preview trackpoint drift](images/mouse-drift-preview.gif)

### "Productivity tools"
We have some great additions for "productivity" bundled with the system. Not that we think you would do anything useful anyways..

#### Incel
Incel is a spreadsheet application with **exactly one** cell per sheet.

You can read more about the project [here](https://github.com/kluzzebass/incel).

![Incel](images/incel-preview.png)

#### Werd
Werd is a word processor that allows **exactly one** word.
You can read more about the project [here](https://github.com/kluzzebass/werd).

![Werd](images/werd-preview.png)

#### Adboost
Adboost is a browser extension oposite to what an adblocker would do, it adds ads to websites.
You can read more about the project [here](https://github.com/surprisetalk/AdBoost).

![Adboost](images/adboost-preview.png)

### Notifications
A notification service will pull up inspirational system messages.

![Notification preview](images/notifications-preview.png)

### Other minor features
- The system's default font has all capital letters replaced with lowercase letters (i.e `A = a`).
- Sometimes prevents space key from being used, ensuring your text has that cozy, compact feel.
- `nano` is an alias to `vim`
- `sudo` is configured to insult you when password is incorrect
- The default cursor is modified to have the click spot on bottom right corner ![new cursor](images/cursor-shift-new.png) instead of top left ![old cursor](images/cursor-shift-old.png)



### VM mouse containment
Putting cursor at the edges of the screen will lock the computer, this is especially usefull for people running this inside a virtual machine when they move the cursor outside of the virtual machine.
These edges will trigger lock screen:

![VM mouse containment](images/vm-screen-edge.png)

### Other customizations


## Getting started
**⚠️INSTALL AT YOUR OWN RISK⚠️**

You should preferably run this in a virtual machine, not on physical hardware. 
Some of the implementations have too much permissions and is considered to be insecure.

### Get the ISO

Check the [github releases](https://github.com/BigBotherLinux/BigBother/releases) for torrent file which will include the latest ISO.

### Building the iso

To build iso:
`nix build .#nixosConfigurations.bb-iso.config.system.build.isoImage`


### Need help?

There is nothing wrong in seeking help, however i doubt you will find it here.


## Found something useful?  
If you have any ideas of how the user experience can deteriorate, please create [issues](https://github.com/BigBotherLinux/BigBother/issues) in the project repo. 

Contributions are also welcome with these guidelines:

- System has to be usable
- User should not be required to do anything other than go through the installer.
- ISO and system has to be reprodusable
- Features should not cause data loss or be destructive
- It should **not** spark joy

<img src="https://feelafraidcomic.com/comics/2011-05-13-welcome-to-hell.png" width="375" height="563">

[feelafraidcomic.com](https://feelafraidcomic.com/60.php) - [@feel_afraid](https://twitter.com/feel_afraid)

## Feature wishlist
This is a list of features not yet implemented. 
- Add system font with only lower-case letters
- "Start" button icon
- Find out a way to set up a theme(the nix way).
- Create a welcome-screen
- New calamares module
- Set up a script or a alias for updating or installing packages. User will have to dig into the nix config themselves, but at least they could get some pointers on where to begin.
- Go over licenses to ensure all is ok


## Special thanks

- Instagram @jfb_fit for making the logo
- [SnowflakeOS](https://github.com/snowflakelinux/) for inspiration and versioning implementation
- [calamares nixos extension](https://github.com/NixOS/calamares-nixos-extensions) for fork and inspiration
- [Arc KDE theme](https://github.com/PapirusDevelopmentTeam/arc-kde) for inspiration on global theme implementation
- ChatGPT/Claude for providing slop
