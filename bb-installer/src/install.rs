//! Disk detection and installation logic

use crate::state::{DiskInfo, InstallProgress, InstallStatus, InstallerState};
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn detect_disks() -> Vec<DiskInfo> {
    let mut disks = Vec::new();

    // Read from /sys/block to find block devices
    if let Ok(entries) = fs::read_dir("/sys/block") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip non-disk devices (loop, ram, etc.)
            if name.starts_with("loop")
                || name.starts_with("ram")
                || name.starts_with("zram")
                || name.starts_with("dm-")
                || name.starts_with("sr")
                || name.starts_with("fd")
            {
                continue;
            }

            let path = format!("/dev/{}", name);
            let sys_path = entry.path();

            // Get size in bytes
            let size_bytes = fs::read_to_string(sys_path.join("size"))
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .map(|sectors| sectors * 512)
                .unwrap_or(0);

            // Skip very small devices (< 1GB)
            if size_bytes < 1_000_000_000 {
                continue;
            }

            // Get model name
            let model = fs::read_to_string(sys_path.join("device/model"))
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| {
                    fs::read_to_string(sys_path.join("device/name"))
                        .ok()
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|| "Unknown Device".to_string())
                });

            disks.push(DiskInfo {
                name,
                path,
                size_bytes,
                model,
            });
        }
    }

    // Sort by name for consistent ordering
    disks.sort_by(|a, b| a.name.cmp(&b.name));
    disks
}

pub fn mock_disks() -> Vec<DiskInfo> {
    vec![
        DiskInfo {
            name: "sda".to_string(),
            path: "/dev/sda".to_string(),
            size_bytes: 256_000_000_000,
            model: "Samsung SSD 860".to_string(),
        },
        DiskInfo {
            name: "sdb".to_string(),
            path: "/dev/sdb".to_string(),
            size_bytes: 1_000_000_000_000,
            model: "WDC WD10EZEX".to_string(),
        },
        DiskInfo {
            name: "nvme0n1".to_string(),
            path: "/dev/nvme0n1".to_string(),
            size_bytes: 512_000_000_000,
            model: "Samsung 970 EVO Plus".to_string(),
        },
    ]
}

fn hash_password(password: &str) -> Result<String, String> {
    // Use mkpasswd to generate a SHA-512 password hash
    // This is available in the NixOS live environment
    let output = Command::new("mkpasswd")
        .args(["-m", "sha-512", password])
        .output()
        .map_err(|e| format!("Failed to run mkpasswd: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "mkpasswd failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn log_message(progress: &Arc<Mutex<InstallProgress>>, msg: &str) {
    // Always print to terminal for visibility
    eprintln!("{}", msg);
    if let Ok(mut p) = progress.lock() {
        p.output_log.push(msg.to_string());
    }
}

fn set_status(progress: &Arc<Mutex<InstallProgress>>, status: InstallStatus) {
    if let Ok(mut p) = progress.lock() {
        p.status = status;
    }
}

fn set_error(progress: &Arc<Mutex<InstallProgress>>, error: &str) {
    if let Ok(mut p) = progress.lock() {
        p.status = InstallStatus::Failed;
        p.error_message = Some(error.to_string());
        p.output_log.push(format!("ERROR: {}", error));
    }
}

fn run_command(
    cmd: &str,
    args: &[&str],
    progress: &Arc<Mutex<InstallProgress>>,
    production_mode: bool,
) -> Result<(), String> {
    log_message(progress, &format!("$ {} {}", cmd, args.join(" ")));

    if !production_mode {
        log_message(
            progress,
            &format!(
                "  [DRY-RUN] Command not executed (BB_PROD != true), would have run: $ {} {}",
                cmd,
                args.join(" ")
            ),
        );
        return Ok(());
    }


    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn {}: {}", cmd, e))?;

    // Read stdout and stderr concurrently to avoid deadlock
    let stdout_handle = child.stdout.take().map(|stdout| {
        let progress = Arc::clone(progress);
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                log_message(&progress, &format!("  {}", line));
            }
        })
    });

    let stderr_handle = child.stderr.take().map(|stderr| {
        let progress = Arc::clone(progress);
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                log_message(&progress, &format!("  [stderr] {}", line));
            }
        })
    });

    // Wait for output threads to complete
    if let Some(handle) = stdout_handle {
        let _ = handle.join();
    }
    if let Some(handle) = stderr_handle {
        let _ = handle.join();
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for {}: {}", cmd, e))?;

    if !status.success() {
        return Err(format!(
            "{} failed with exit code: {:?}",
            cmd,
            status.code()
        ));
    }

    Ok(())
}

fn write_file(
    path: &str,
    content: &str,
    progress: &Arc<Mutex<InstallProgress>>,
    production_mode: bool,
) -> Result<(), String> {
    log_message(progress, &format!("Writing file: {}", path));

    // Always log the content preview
    let preview: String = content.lines().take(20).collect::<Vec<_>>().join("\n");
    log_message(progress, "--- File content preview ---");
    for line in preview.lines() {
        log_message(progress, &format!("  {}", line));
    }
    if content.lines().count() > 20 {
        log_message(progress, "  ... (truncated)");
    }
    log_message(progress, "--- End preview ---");

    if !production_mode {
        log_message(progress, "  [DRY-RUN] File not written (BB_PROD != true)");
        return Ok(());
    }

    fs::write(path, content).map_err(|e| format!("Failed to write {}: {}", path, e))
}

fn create_dir(
    path: &str,
    progress: &Arc<Mutex<InstallProgress>>,
    production_mode: bool,
) -> Result<(), String> {
    log_message(progress, &format!("Creating directory: {}", path));

    if !production_mode {
        log_message(
            progress,
            "  [DRY-RUN] Directory not created (BB_PROD != true)",
        );
        return Ok(());
    }

    fs::create_dir_all(path).map_err(|e| format!("Failed to create {}: {}", path, e))
}

pub fn generate_installer_nix(state: &InstallerState) -> String {
    let password_hash = hash_password(&state.user_config.password)
        .unwrap_or_else(|_| "$6$rounds=5000$invalid$hash".to_string());

    format!(
        r#"{{ ... }}:
{{
  # Citizen Configuration - Generated by BigBother Installer
  # "Your compliance has been noted."

  # Boot Loader Configuration
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Filesystem Configuration
  fileSystems."/" = {{
    device = "/dev/disk/by-label/NIXROOT";
    fsType = "ext4";
  }};

  fileSystems."/boot" = {{
    device = "/dev/disk/by-label/NIXBOOT";
    fsType = "vfat";
  }};

  # Swap Configuration
#  swapDevices = [ {{
#    device = "/.swapfile";
#  }} ];

  # User Configuration
  users.users.{username} = {{
    isNormalUser = true;
    description = "Citizen {username}";
    extraGroups = [ "wheel" "networkmanager" "video" "audio" ];
    hashedPassword = "{password_hash}";
  }};

  # Timezone Configuration
  time.timeZone = "{timezone}";

  # Keyboard Configuration
  services.xserver.xkb.layout = "{keyboard}";
  console.keyMap = "{keyboard}";

  # Hostname Configuration
  networking.hostName = "{hostname}";

  # Auto-login for convenience (and monitoring)
  services.displayManager.autoLogin = {{
    enable = true;
    user = "{username}";
  }};
}}
"#,
        username = state.user_config.username,
        password_hash = password_hash,
        timezone = state.user_config.timezone,
        keyboard = state.user_config.keyboard_layout,
        hostname = state.user_config.hostname,
    )
}

pub fn start_installation(state: &InstallerState) {
    // In dry-run mode, use a mock disk if none selected
    let disk = match state.get_selected_disk() {
        Some(d) => d.clone(),
        None => {
            if state.production_mode {
                // Production mode requires a real disk selection
                return;
            }
            // Use mock disk for dry-run demonstration
            DiskInfo {
                name: "mock-disk".to_string(),
                path: "/dev/sdX".to_string(),
                size_bytes: 256_000_000_000,
                model: "Mock Disk (dry-run)".to_string(),
            }
        }
    };

    let progress = Arc::clone(&state.install_progress);
    let flake_path = state.flake_path.clone();
    let installer_nix_content = generate_installer_nix(state);
    let production_mode = state.production_mode;

    thread::spawn(move || {
        install_system(
            &disk,
            &flake_path,
            &installer_nix_content,
            &progress,
            production_mode,
        );
    });
}

fn install_system(
    disk: &DiskInfo,
    flake_path: &str,
    installer_nix_content: &str,
    progress: &Arc<Mutex<InstallProgress>>,
    production_mode: bool,
) {
    log_message(progress, "=== BigBother Installation Starting ===");
    if !production_mode {
        log_message(progress, "");
        log_message(progress, "*** DRY-RUN MODE (BB_PROD != true) ***");
        log_message(progress, "*** No actual changes will be made ***");
        log_message(progress, "");
    }
    log_message(
        progress,
        &format!("Target device: {} ({})", disk.path, disk.size_human()),
    );
    log_message(progress, "");

    // Step 1: Partition disk
    set_status(progress, InstallStatus::Partitioning);
    log_message(progress, "Creating partition table...");

    if let Err(e) = partition_disk(&disk.path, progress, production_mode) {
        set_error(progress, &format!("Partitioning failed: {}", e));
        return;
    }

    // Step 2: Format partitions
    set_status(progress, InstallStatus::Formatting);
    log_message(progress, "Formatting partitions...");

    let efi_part = format!("{}1", disk.path);
    let root_part = format!("{}2", disk.path);

    // Handle nvme naming convention
    let (efi_part, root_part) = if disk.path.contains("nvme") || disk.path.contains("mmcblk") {
        (format!("{}p1", disk.path), format!("{}p2", disk.path))
    } else {
        (efi_part, root_part)
    };

    if let Err(e) = run_command(
        "mkfs.fat",
        &["-F", "32", "-n", "NIXBOOT", &efi_part],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to format EFI partition: {}", e));
        return;
    }

    if let Err(e) = run_command(
        "mkfs.ext4",
        &["-L", "NIXROOT", "-F", &root_part],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to format root partition: {}", e));
        return;
    }

    // Wait for udev to register the partition labels
    log_message(progress, "Waiting for partition labels to register...");
    if let Err(e) = run_command(
        "udevadm",
        &["settle", "--timeout=10"],
        progress,
        production_mode,
    ) {
        log_message(
            progress,
            &format!("Warning: udevadm settle failed: {}", e),
        );
    }

    // Additional wait for label propagation
    if production_mode {
        std::thread::sleep(std::time::Duration::from_secs(2));
    } else {
        log_message(progress, "  [DRY-RUN] Would wait 2 seconds for label propagation");
    }

    // Step 3: Mount filesystems
    set_status(progress, InstallStatus::Mounting);
    log_message(progress, "Mounting filesystems...");

    // Debug: List available labels
    log_message(progress, "Available partition labels:");
    if let Err(e) = run_command(
        "ls",
        &["-la", "/dev/disk/by-label/"],
        progress,
        production_mode,
    ) {
        log_message(progress, &format!("Warning: Could not list labels: {}", e));
    }

    // Mount by label for consistency
    if let Err(e) = run_command(
        "mount",
        &["/dev/disk/by-label/NIXROOT", "/mnt"],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to mount root: {}", e));
        return;
    }

    // Create and mount boot directory
    if let Err(e) = create_dir("/mnt/boot", progress, production_mode) {
        set_error(progress, &format!("Failed to create /mnt/boot: {}", e));
        return;
    }

    if let Err(e) = run_command(
        "mount",
        &["/dev/disk/by-label/NIXBOOT", "/mnt/boot"],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to mount boot: {}", e));
        return;
    }

    // Step 3.5: Create swap file
    set_status(progress, InstallStatus::CreatingSwap);
    log_message(progress, "Creating swap file (2GB)...");

    if let Err(e) = run_command(
        "dd",
        &[
            "if=/dev/zero",
            "of=/mnt/.swapfile",
            "bs=1M",
            "count=2048",
            "status=progress",
        ],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to create swap file: {}", e));
        return;
    }

    if let Err(e) = run_command(
        "chmod",
        &["600", "/mnt/.swapfile"],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to set swap file permissions: {}", e));
        return;
    }

    if let Err(e) = run_command("mkswap", &["/mnt/.swapfile"], progress, production_mode) {
        set_error(progress, &format!("Failed to format swap file: {}", e));
        return;
    }

    if let Err(e) = run_command("swapon", &["/mnt/.swapfile"], progress, production_mode) {
        set_error(progress, &format!("Failed to activate swap: {}", e));
        return;
    }

    // Step 4: Copy flake
    set_status(progress, InstallStatus::CopyingFlake);
    log_message(progress, "Deploying BigBother configuration...");

    let dest_flake = "/mnt/etc/nixos";
    if let Err(e) = create_dir(dest_flake, progress, production_mode) {
        set_error(progress, &format!("Failed to create {}: {}", dest_flake, e));
        return;
    }

    if let Err(e) = run_command(
        "cp",
        &["-r", &format!("{}/.", flake_path), dest_flake],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to copy flake: {}", e));
        return;
    }

    // Step 5: Generate hardware configuration
    set_status(progress, InstallStatus::GeneratingConfig);
    log_message(progress, "Generating hardware configuration...");

    if let Err(e) = run_command(
        "nixos-generate-config",
        &["--root", "/mnt", "--no-filesystems"],
        progress,
        production_mode,
    ) {
        set_error(
            progress,
            &format!("Failed to generate hardware config: {}", e),
        );
        return;
    }

    // Step 6: Generate installer.nix with user configuration
    log_message(progress, "Generating citizen configuration...");

    let installer_path = format!("{}/installer.nix", dest_flake);
    if let Err(e) = write_file(
        &installer_path,
        installer_nix_content,
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to write installer.nix: {}", e));
        return;
    }

    // Step 6.5: Initialize git repo and add generated configs so flake can see them
    log_message(progress, "Initializing git repository for flake...");

    if let Err(e) = run_command(
        "git",
        &["-C", dest_flake, "init"],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to initialize git repo: {}", e));
        return;
    }

    log_message(progress, "Registering configuration files with git...");

    if let Err(e) = run_command(
        "git",
        &["-C", dest_flake, "add", "."],
        progress,
        production_mode,
    ) {
        set_error(progress, &format!("Failed to add configs to git: {}", e));
        return;
    }

    // Step 7: Run nixos-install
    set_status(progress, InstallStatus::RunningNixosInstall);
    log_message(progress, "");
    log_message(progress, "=== Running nixos-install ===");
    log_message(
        progress,
        "This may take a while. BigBother is watching your progress.",
    );
    log_message(progress, "");

    let install_result = run_command(
        "nixos-install",
        &[
            "--impure",
            "--flake",
            &format!("{}#bb", dest_flake),
            "--root",
            "/mnt",
            "--no-root-passwd",
        ],
        progress,
        production_mode,
    );

    if let Err(e) = install_result {
        set_error(progress, &format!("nixos-install failed: {}", e));
        return;
    }

    // Step 8: Finalize
    set_status(progress, InstallStatus::Finalizing);
    log_message(progress, "");
    log_message(progress, "Finalizing installation...");

    // Disable swap before unmounting
    let _ = run_command("swapoff", &["/mnt/.swapfile"], progress, production_mode);

    // Unmount all filesystems
    let _ = run_command("umount", &["-R", "/mnt"], progress, production_mode);

    set_status(progress, InstallStatus::Complete);
    log_message(progress, "");
    log_message(progress, "=== Installation Complete ===");
    if !production_mode {
        log_message(progress, "");
        log_message(progress, "*** DRY-RUN completed successfully ***");
        log_message(
            progress,
            "*** Set BB_PROD=true to perform actual installation ***",
        );
    } else {
        log_message(progress, "BigBother welcomes you, citizen.");
        log_message(
            progress,
            "Please reboot to begin your new life under our watchful care.",
        );
    }
}

fn partition_disk(
    device: &str,
    progress: &Arc<Mutex<InstallProgress>>,
    production_mode: bool,
) -> Result<(), String> {
    // Create GPT partition table with:
    // - 512MB EFI System Partition
    // - Rest as Linux root

    log_message(progress, "Creating GPT partition table...");

    // Use parted for partitioning
    run_command(
        "parted",
        &["-s", device, "mklabel", "gpt"],
        progress,
        production_mode,
    )?;

    log_message(progress, "Creating EFI partition (512MB)...");
    run_command(
        "parted",
        &["-s", device, "mkpart", "ESP", "fat32", "1MiB", "513MiB"],
        progress,
        production_mode,
    )?;
    run_command(
        "parted",
        &["-s", device, "set", "1", "esp", "on"],
        progress,
        production_mode,
    )?;

    log_message(progress, "Creating root partition...");
    run_command(
        "parted",
        &["-s", device, "mkpart", "nixos", "ext4", "513MiB", "100%"],
        progress,
        production_mode,
    )?;

    // Inform kernel of partition table changes
    log_message(progress, "Informing kernel of partition changes...");
    run_command("partprobe", &[device], progress, production_mode)?;

    // Wait for partition devices to appear
    log_message(progress, "Waiting for partition devices...");
    if let Err(e) = run_command(
        "udevadm",
        &["settle", "--timeout=10"],
        progress,
        production_mode,
    ) {
        log_message(
            progress,
            &format!("Warning: udevadm settle failed: {}", e),
        );
    }

    if production_mode {
        std::thread::sleep(std::time::Duration::from_secs(2));
    } else {
        log_message(progress, "  [DRY-RUN] Would wait 2 seconds for devices");
    }

    Ok(())
}

pub fn get_common_timezones() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Europe/London", "Greenwich Mean Surveillance Time"),
        ("Europe/Berlin", "Central European Observation Time"),
        ("Europe/Paris", "Paris Monitoring Zone"),
        ("Europe/Oslo", "Nordic Oversight Time"),
        ("Europe/Stockholm", "Scandinavian Surveillance Time"),
        ("America/New_York", "Eastern Surveillance Time"),
        ("America/Chicago", "Central Observation Time"),
        ("America/Denver", "Mountain Monitoring Time"),
        ("America/Los_Angeles", "Pacific Oversight Time"),
        ("Asia/Tokyo", "Tokyo Tracking Zone"),
        ("Asia/Shanghai", "Beijing Supervision Time"),
        ("Australia/Sydney", "Australian Monitoring Time"),
        ("UTC", "Universal Tracking Coordinate"),
    ]
}

pub fn get_keyboard_layouts() -> Vec<(&'static str, &'static str)> {
    vec![
        ("us", "US English (Standard Surveillance Layout)"),
        ("gb", "UK English (Commonwealth Monitoring)"),
        ("de", "German (Enhanced Tracking)"),
        ("fr", "French (AZERTY Monitoring)"),
        ("no", "Norwegian (Nordic Observation)"),
        ("se", "Swedish (Scandinavian Layout)"),
        ("dk", "Danish (Nordic Variant)"),
        ("es", "Spanish (Iberian Tracking)"),
        ("it", "Italian (Mediterranean Monitor)"),
        ("pl", "Polish (Eastern European)"),
        ("ru", "Russian (Cyrillic Observation)"),
        ("jp", "Japanese (Eastern Tracking)"),
    ]
}
