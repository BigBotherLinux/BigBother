# CLAUDE.md — bb-installer-bevy

An installer for BigBother built as a game. It replaces the eframe installer in
`../bb-installer`, so it has to collect the same data and run the same install
steps. Everything around that is free to be a game.

Each themed page has a plan in `plans/` (e.g. `plans/secondary-tos.md`).

Build, dev shell, kiosk module and VM test are covered in the root `CLAUDE.md`
(section "Bevy installer prototype"). Run cargo directly here; the user's shell
already has the bevy env loaded.

## What a successful install needs

This is what the old installer (`../bb-installer/src/install.rs`, `state.rs`)
collects and does. The bevy installer can dress it up however it likes, but the
end result must be the same.

### Preflight

- UEFI: `/sys/firmware/efi` must exist. Only systemd-boot is configured, so
  BIOS boots are unsupported.
- Network: `ping -c 1 -W 3 1.1.1.1`. Needed because `nixos-install` downloads
  from the cache. `src/network.rs` already covers this.
- In production both must pass. In preview mode they may fail.

### Data to collect

| Field | Used for | Old installer behaviour |
|----------------|---------------------------------------------|-------------------------|
| Disk | the device that gets wiped | `detect_disks()`: `/sys/block`, skips loop/ram/zram/dm-/sr/fd and anything < 1 GB |
| Username | `bigbother.primaryUser`, `users.users.<name>` | heavy joke validation (see below), must still be a valid Linux username |
| Password | `hashedPassword` via `mkpasswd -m sha-512` | always `1234`, whatever the user picks |
| Timezone | `time.timeZone` | IANA id, e.g. `Europe/Oslo` |
| Keyboard | `services.xserver.xkb.layout`, `console.keyMap` | xkb id, e.g. `no` |
| Hostname | `networking.hostName` | 1–63 chars, `[a-z0-9-]`, no leading or trailing `-` |
| Age bracket | encrypted attestation (see step 8) | `bb_age_attestation::types::AgeBracket::from(age)` |
| Consent | gate only | format disk, unfree software, own risk / free will |

The feature toggles ("Mandatory Optional Features") are pure theatre. They must
all be on to continue, and none of them are written anywhere: the installed
system's features come from `configuration.nix`.

### Install sequence

Runs in a background thread. Every command is logged; the log is shown at the end.

1. **Partition** (GPT): `parted -s <dev> mklabel gpt`,
   `mkpart ESP fat32 1MiB 513MiB`, `set 1 esp on`,
   `mkpart nixos ext4 513MiB 100%`, then `partprobe <dev>`,
   `udevadm settle --timeout=10`, sleep 2s.
1. **Format**: `mkfs.fat -F 32 -n NIXBOOT <p1>`, `mkfs.ext4 -L NIXROOT -F <p2>`.
   Partition names get a `p` (`nvme0n1p1`, `mmcblk0p1`) for nvme and mmcblk
   devices. Then `udevadm settle`, sleep 2s.
1. **Mount**: `/dev/disk/by-label/NIXROOT` → `/mnt`, `mkdir /mnt/boot`,
   `/dev/disk/by-label/NIXBOOT` → `/mnt/boot`.
1. **Swap**: 2 GB `/mnt/.swapfile` (`dd`, `chmod 600`, `mkswap`, `swapon`).
   This is for the install only; the generated config does not enable it.
1. **Copy flake**: `cp -r $BB_FLAKE_PATH/. /mnt/etc/nixos`.
   `BB_FLAKE_PATH` defaults to `/etc/bb-flake`, which the ISO fills with the
   flake source (`installer-iso.nix`).
1. **Hardware config**: `nixos-generate-config --root /mnt --no-filesystems`.
1. **Write `/mnt/etc/nixos/installer.nix`** with systemd-boot
   (`canTouchEfiVariables = true`), the two filesystems by label, and the user,
   timezone, keyboard and hostname. See `generate_installer_nix`. User
   description is `"Citizen <name>"`. `configuration.nix` imports
   `installer.nix` and `hardware-configuration.nix` only if they exist. No git
   repo is needed because the copied flake is a plain directory.
1. **`nixos-install --impure --flake /mnt/etc/nixos#bb --root /mnt --no-root-passwd`**.
   This is the long step.
1. **Age attestation**: serialize the bracket to JSON, encrypt with
   `bb_age_attestation::crypto::encrypt`, write
   `/mnt/root/.local/share/bb-age-attestation/attestation.age`. If this fails,
   log a warning and carry on.
1. **Finalize**: `swapoff /mnt/.swapfile`, `umount -R /mnt`. Offer a reboot
   (`reboot`) and tell the user to remove the installation media.

The progress weights the old installer used: partitioning 0.10, formatting 0.18,
mounting 0.23, swap 0.28, copy 0.32, config 0.37, nixos-install 0.50,
finalizing 0.95.

### Safety rules

- **Dry run by default.** Only `BB_PROD=true` runs real commands. Otherwise
  each command and file write is logged as `[DRY-RUN]` and skipped, and disks
  come from `mock_disks()`. Keep that split.
- The disclaimer at the start is the only sincere warning. The real device path
  and size must still be visible when the disk is picked, whatever game is
  wrapped around it.
- Once the install starts there is no going back.

### Not ported yet

- `modules/bb-installer-bevy.nix` does not set `BB_PROD` or `BB_FLAKE_PATH`, and
  does not add `parted`, `e2fsprogs`, `dosfstools` to the system. The old module
  (`modules/bb-installer.nix`) does all three.
- The crate does not depend on `bb-age-attestation` (it is outside the
  workspace on purpose, so it needs a path dependency).
- `mkpasswd` must be on `PATH` in the live environment.

## The humour

BigBother is a calm, sincere institution that thinks it is being reasonable.
The joke is bureaucracy treating something absurd as routine paperwork. It never
winks, never uses exclamation marks to sell a joke, never explains it, and
never goes random or "whimsical". When it fails to be funny it should read as
dry, not as trying hard. Avoid the tone of a forum commenter doing a bit.

### Patterns already in use

- **Bureaucratic renaming.** Page titles: "Terms of Submission", "Citizen
  Registration", "Password Security Theater", "Temporal Jurisdiction", "Input
  Device Registration", "Storage Requisition", "Mandatory Optional Features",
  "Communications Checkpoint", "Pre-Installation Briefing", "Installation
  Monitor". Timezones get surveillance names ("Nordic Oversight Time",
  "Universal Tracking Coordinate"), keyboard layouts too ("Norwegian (Nordic
  Observation)").
- **Choice that isn't one.** The password is always `1234` after a two-step
  questionnaire ("Select your philosophical approach", "What should your
  password remind you of?") and a checkbox "I accept my uniquely and randomly
  generated password". Every feature must be on. Turning off telemetry turns
  every other feature off one by one, then turns telemetry back on.
- **Declining degrades.** "Decline" → "This is very unusual, are you sure?" →
  "[ Declining is not supported ]", which also accepts the terms.
- **Rules that contradict each other.** The username must have exactly one
  digit, larger than 7, not at the end. No "test", no uppercase ("All letters
  are uppercase, are you angry over something?"), no special characters ("This
  is not a password"). The first valid name is then "already taken". Pressing
  Enter shows "Please click the Continue button to proceed".
- **Honest admissions.** "Didn't really bother implementing partitioning or
  encryption, so this is all or nothing...". The README does the same thing.
- **Fake legal language with one off word.** "We value your privacy,
  literally." "6-9 business centuries." "You have 'nothing to hide'". "Due to
  GDPR and other inconveniences". "You just lost the game" numbered as clause 4,
  out of order between 8 and 6.
- **Overstated compliance.** Age verification is "THRILLED", "nothing could
  excite us more". The age stepper starts at 1, goes up one at a time with a
  250 ms cooldown, and caps at 150.
- **Loyalty language.** "Our source is open. Your curtains should be too..",
  "By clicking 'Install', you confirm your eternal loyalty to BigBother.",
  "BigBother welcomes you, citizen.", "Embrace the gaze of BigBother and let it
  rest upon you."
- **The eye.** A surveillance eye watches from the header. Sprites live in
  `assets/Eyes_V2`.

### Already in this crate

- A fake cursor (`src/cursor.rs`): the real pointer is locked and hidden, and
  ours is drawn and fed to picking. Write `FakeCursor::position` to move it.
- `CursorAtraction` on a node pushes the cursor away (despite the name). The
  welcome screen's Install button uses it.
- Scroll direction is decided by the user's first scroll, and then inverted
  from it.
- The ToS is a scroll area. The secondary ToS (below) is not ported to the
  feathers rewrite yet.
- The contract is signed with the mouse on a canvas (`src/pages/sign.rs`).
  Continue unlocks after 3000 inked pixels.

### The secondary ToS (removed, to be ported)

The consent questions were removed in `7fddb03` ("start from scratch with
feather as lib"). Read them with `git show 7fddb03^:bb-installer-bevy/src/tos.rs`.
That commit also removed the branching dialogue engine (`src/dialogue.rs`,
`src/script.rs`: `ask`, `select`, `say`, `rate` with `at_least`/`at_most`
refusals, `.aside()` small print, `.record()` into an `Answers` resource).

The flow:

1. "Are you here of your own free will?" (aside: "We need your consent to proceed.")
   - "No, I was coerced" → "We cannot legally proceed without your consent.
     Please shut down your computer." → "Sorry, I changed my mind" / "Exit".
   - "Yes, I consent" → 2.
1. "Do you believe in free will?"
   - **No** → "Then how exactly could you consent to something you did not
     choose?" Only button: "I was destined to click this button too!" (aside:
     "There is no need to deliberate.") → "Correct. And how could consent work
     in a legal system anyway, if you did not choose to consent?" → "Exactly,
     it wouldn't" (aside: "You may click the button. It is the only button.")
     → "Your consent has therefore been backdated to the beginning of the
     universe." → "I love BigBother" (aside: "Thank you. That was very freely
     given.")
   - **Yes** → "Good. Meaning you chose to consent to this." (aside: "Any legal
     system implies determinism for consent to work as a concept.")
1. "Select what you wish to pledge.": browsing data / firstborn / remaining
   dignity (aside: "Exactly one. Pledging nothing is not an option.")
1. "Your answers have been noted and forwarded." (aside: "Forwarded where is on
   a need-to-know basis. You do not need to know.")
1. Ratings, each refusing answers it doesn't like:
   - "How is your experience so far?" Below 8: "That cannot be right. Move it
     up and try again." (aside: "Be honest. There is a correct answer.")
   - "How likely are you to recommend BigBother…?" Below 10: "Anything under 10
     counts as a complaint about you." (aside: "Family includes people you have
     not met yet.")
   - "How much do you trust this installer?" Above 3: "Overconfidence noted.
     Lower it." (aside: "This is the only question where a low score is the
     honest one.")

The asides carry most of the jokes: the question is plain, and the small print
underneath is where the institution shows itself.
