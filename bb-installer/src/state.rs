//! Installer state management

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckStatus {
    Pending,
    Running,
    Passed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct PreflightState {
    pub internet: CheckStatus,
    pub uefi: CheckStatus,
    pub internet_error: Option<String>,
    pub uefi_error: Option<String>,
    pub checks_started: bool,
}

impl Default for PreflightState {
    fn default() -> Self {
        Self {
            internet: CheckStatus::Pending,
            uefi: CheckStatus::Pending,
            internet_error: None,
            uefi_error: None,
            checks_started: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Welcome,
    Disclaimer,
    TermsOfSubmission,
    UserSetup,
    PasswordSetup,
    TimezoneSelection,
    KeyboardSelection,
    DiskSelection,
    FeatureSelection,
    HostnameSetup,
    Summary,
    Installing,
    Complete,
}

impl Page {
    pub fn index(&self) -> usize {
        match self {
            Page::Welcome => 0,
            Page::Disclaimer => 1,
            Page::TermsOfSubmission => 2,
            Page::UserSetup => 3,
            Page::PasswordSetup => 4,
            Page::TimezoneSelection => 5,
            Page::KeyboardSelection => 6,
            Page::DiskSelection => 7,
            Page::FeatureSelection => 8,
            Page::HostnameSetup => 9,
            Page::Summary => 10,
            Page::Installing => 11,
            Page::Complete => 12,
        }
    }

    pub fn from_index(index: usize) -> Option<Page> {
        match index {
            0 => Some(Page::Welcome),
            1 => Some(Page::Disclaimer),
            2 => Some(Page::TermsOfSubmission),
            3 => Some(Page::UserSetup),
            4 => Some(Page::PasswordSetup),
            5 => Some(Page::TimezoneSelection),
            6 => Some(Page::KeyboardSelection),
            7 => Some(Page::DiskSelection),
            8 => Some(Page::FeatureSelection),
            9 => Some(Page::HostnameSetup),
            10 => Some(Page::Summary),
            11 => Some(Page::Installing),
            12 => Some(Page::Complete),
            _ => None,
        }
    }

    pub fn total() -> usize {
        12
    }

    pub fn title(&self) -> &'static str {
        match self {
            Page::Welcome => "Welcome to BigBother",
            Page::Disclaimer => "Disclaimer",
            Page::TermsOfSubmission => "Terms of Submission",
            Page::UserSetup => "Citizen Registration",
            Page::PasswordSetup => "Password Security Theater",
            Page::TimezoneSelection => "Temporal Jurisdiction",
            Page::KeyboardSelection => "Input Device Registration",
            Page::DiskSelection => "Storage Requisition",
            Page::FeatureSelection => "Mandatory Optional Features",
            Page::HostnameSetup => "Communications Checkpoint",
            Page::Summary => "Pre-Installation Briefing",
            Page::Installing => "Installation Monitor",
            Page::Complete => "Installation Complete",
        }
    }

    pub fn next(&self) -> Option<Page> {
        match self {
            Page::Welcome => Some(Page::Disclaimer),
            Page::Disclaimer => Some(Page::TermsOfSubmission),
            Page::TermsOfSubmission => Some(Page::UserSetup),
            Page::UserSetup => Some(Page::PasswordSetup),
            Page::PasswordSetup => Some(Page::TimezoneSelection),
            Page::TimezoneSelection => Some(Page::KeyboardSelection),
            Page::KeyboardSelection => Some(Page::DiskSelection),
            Page::DiskSelection => Some(Page::FeatureSelection),
            Page::FeatureSelection => Some(Page::HostnameSetup),
            Page::HostnameSetup => Some(Page::Summary),
            Page::Summary => Some(Page::Installing),
            Page::Installing => Some(Page::Complete),
            Page::Complete => None,
        }
    }

    pub fn prev(&self) -> Option<Page> {
        match self {
            Page::Welcome => None,
            Page::Disclaimer => Some(Page::Welcome),
            Page::TermsOfSubmission => Some(Page::Disclaimer),
            Page::UserSetup => Some(Page::TermsOfSubmission),
            Page::PasswordSetup => Some(Page::UserSetup),
            Page::TimezoneSelection => Some(Page::PasswordSetup),
            Page::KeyboardSelection => Some(Page::TimezoneSelection),
            Page::DiskSelection => Some(Page::KeyboardSelection),
            Page::FeatureSelection => Some(Page::DiskSelection),
            Page::HostnameSetup => Some(Page::FeatureSelection),
            Page::Summary => Some(Page::HostnameSetup),
            Page::Installing => None,
            Page::Complete => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub model: String,
}

impl DiskInfo {
    pub fn size_human(&self) -> String {
        let gb = self.size_bytes as f64 / 1_000_000_000.0;
        if gb >= 1000.0 {
            format!("{:.1} TB", gb / 1000.0)
        } else {
            format!("{:.1} GB", gb)
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub timezone: String,
    pub keyboard_layout: String,
    pub hostname: String,
}

/// Completely meaningless password configuration options
/// All of these are ignored - password is always set to "1234"
#[derive(Debug, Clone)]
pub struct PasswordTheater {
    // Reveal step
    pub reveal_step: RevealStep,
    pub password_philosophy: PasswordPhilosophy,
    pub memorable_source: MemorableSource,

    // Final acknowledgment
    pub accept_ministry_override: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RevealStep {
    Philosophy,      // First: ask about password philosophy
    MemorableSource, // Second: ask about memorable source
    FinalReveal,     // Third: show the "1234" password
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PasswordPhilosophy {
    Nihilistic,
    Optimistic,
    Fatalistic,
    Kafkaesque,
    Stoic,
    Paranoid,
    Defeatist,
}

impl PasswordPhilosophy {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Nihilistic => "Nihilistic",
            Self::Optimistic => "Optimistic",
            Self::Fatalistic => "Fatalistic",
            Self::Kafkaesque => "Kafkaesque",
            Self::Stoic => "Stoic",
            Self::Paranoid => "Paranoid",
            Self::Defeatist => "Defeatist",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Nihilistic => "Nothing matters, so why would you care?",
            Self::Optimistic => "Maybe this time you'll remember it?",
            Self::Fatalistic => "The Ministry will decide what's best",
            Self::Kafkaesque => {
                "Your password must contain 8 characters, a number, a symbol, a haiku, and the approval of a committee that meets quarterly."
            }
            Self::Stoic => {
                "You cannot control the hacker, you can only control your reaction to theft."
            }
            Self::Paranoid => "You want the strongest password possible.",
            Self::Defeatist => "You'll never remember it, so you will just reset it anyways.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemorableSource {
    ChildhoodTrauma,
    FirstCrush,
    EmbarrassingMoment,
    ForgottenDreams,
}

impl MemorableSource {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ChildhoodTrauma => "Childhood Trauma",
            Self::FirstCrush => "First Crush",
            Self::EmbarrassingMoment => "Most Embarrassing Moment",
            Self::ForgottenDreams => "Forgotten Dreams",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ChildhoodTrauma => "Nothing says 'secure' like suppressed memories",
            Self::FirstCrush => "They never texted back, but the password remains",
            Self::EmbarrassingMoment => "You'll never forget... unfortunately",
            Self::ForgottenDreams => "Just like your career aspirations",
        }
    }
}

impl Default for PasswordTheater {
    fn default() -> Self {
        Self {
            reveal_step: RevealStep::Philosophy,
            password_philosophy: PasswordPhilosophy::Nihilistic,
            memorable_source: MemorableSource::ChildhoodTrauma,
            accept_ministry_override: false,
        }
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            password_confirm: String::new(),
            timezone: "Europe/London".to_string(),
            keyboard_layout: "us".to_string(),
            hostname: "bigbother-node".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FeatureConfig {
    // Browser & Shell
    pub edge_browser: bool,
    pub nano_vim_alias: bool,
    pub sudo_insults: bool,

    // Visual "Enhancements"
    pub lowercase_font: bool,
    pub cursor_shift: bool,
    pub trackpoint_drift: bool,

    // Security Theater
    pub accidental_boot_protection: bool,
    pub login_amnesia: bool,

    // Productivity Features
    pub telemetry: bool,

    // Notifications
    pub system_notifications: bool,

    // Productivity Tools
    pub productivity_tools: bool,
}

impl FeatureConfig {
    pub fn new() -> Self {
        Self {
            edge_browser: false,
            nano_vim_alias: false,
            sudo_insults: false,
            lowercase_font: false,
            cursor_shift: false,
            trackpoint_drift: false,
            accidental_boot_protection: false,
            login_amnesia: false,
            telemetry: true,
            system_notifications: false,
            productivity_tools: false,
        }
    }

    /// Returns true only if all features are enabled
    pub fn all_enabled(&self) -> bool {
        self.edge_browser
            && self.nano_vim_alias
            && self.sudo_insults
            && self.lowercase_font
            && self.cursor_shift
            && self.trackpoint_drift
            && self.accidental_boot_protection
            && self.login_amnesia
            && self.telemetry
            && self.system_notifications
            && self.productivity_tools
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallStatus {
    NotStarted,
    Partitioning,
    Formatting,
    Mounting,
    CreatingSwap,
    CopyingFlake,
    GeneratingConfig,
    RunningNixosInstall,
    Finalizing,
    Complete,
    Failed,
}

impl InstallStatus {
    pub fn message(&self) -> &'static str {
        match self {
            Self::NotStarted => "Preparing installation...",
            Self::Partitioning => "Partitioning disk for optimal surveillance...",
            Self::Formatting => "Formatting partitions with secure filesystem...",
            Self::Mounting => "Mounting filesystems...",
            Self::CreatingSwap => "Creating swap space...",
            Self::CopyingFlake => "Deploying BigBother configuration...",
            Self::GeneratingConfig => "Generating configuration...",
            Self::RunningNixosInstall => "Installing BigBother (this will take a while)...",
            Self::Finalizing => "Finalizing installation...",
            Self::Complete => "Installation complete!",
            Self::Failed => "Installation failed!",
        }
    }

    pub fn progress(&self) -> f32 {
        match self {
            Self::NotStarted => 0.0,
            Self::Partitioning => 0.1,
            Self::Formatting => 0.18,
            Self::Mounting => 0.23,
            Self::CreatingSwap => 0.28,
            Self::CopyingFlake => 0.32,
            Self::GeneratingConfig => 0.37,
            Self::RunningNixosInstall => 0.5,
            Self::Finalizing => 0.95,
            Self::Complete => 1.0,
            Self::Failed => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstallProgress {
    pub status: InstallStatus,
    pub output_log: Vec<String>,
    pub error_message: Option<String>,
}

impl Default for InstallProgress {
    fn default() -> Self {
        Self {
            status: InstallStatus::NotStarted,
            output_log: Vec::new(),
            error_message: None,
        }
    }
}

pub struct InstallerState {
    pub current_page: Page,
    /// Production mode - only true if BB_PROD=true environment variable is set.
    /// When false, all disk operations and commands are simulated (printed only).
    pub production_mode: bool,
    pub disclaimer_format_accepted: bool,
    pub disclaimer_unfree_accepted: bool,
    pub disclaimer_surveillance_accepted: bool,
    pub terms_accepted: bool,
    pub user_config: UserConfig,
    pub feature_config: FeatureConfig,
    pub password_theater: PasswordTheater,
    pub available_disks: Vec<DiskInfo>,
    pub selected_disk: Option<usize>,
    pub install_progress: Arc<Mutex<InstallProgress>>,
    pub flake_path: String,
    pub decline_attempts: u32,
    /// The first valid username entered - will be "already taken"
    pub taken_username: Option<String>,
    /// Set after clicking Continue on user setup, to show validation errors
    pub username_validated: bool,
    /// Show warning when user presses Enter in username field
    pub show_enter_warning: bool,
    /// Cascade animation state for telemetry disable
    pub feature_cascade_active: bool,
    pub feature_cascade_start_time: Option<f64>,
    pub feature_cascade_index: usize,
    pub preflight: Arc<Mutex<PreflightState>>,
}

impl InstallerState {
    pub fn new() -> Self {
        Self::new_with_page(Page::Welcome)
    }

    pub fn new_with_page(starting_page: Page) -> Self {
        // Production mode only enabled if BB_PROD=true is explicitly set
        let production_mode = std::env::var("BB_PROD")
            .map(|v| v == "true")
            .unwrap_or(false);

        Self {
            current_page: starting_page,
            production_mode,
            disclaimer_format_accepted: false,
            disclaimer_unfree_accepted: false,
            disclaimer_surveillance_accepted: false,
            terms_accepted: false,
            user_config: UserConfig::default(),
            feature_config: FeatureConfig::new(),
            password_theater: PasswordTheater::default(),
            available_disks: Vec::new(),
            selected_disk: None,
            install_progress: Arc::new(Mutex::new(InstallProgress::default())),
            flake_path: std::env::var("BB_FLAKE_PATH")
                .unwrap_or_else(|_| "/etc/bb-flake".to_string()),
            decline_attempts: 0,
            taken_username: None,
            username_validated: false,
            show_enter_warning: false,
            feature_cascade_active: false,
            feature_cascade_start_time: None,
            feature_cascade_index: 0,
            preflight: Arc::new(Mutex::new(PreflightState::default())),
        }
    }

    pub fn can_proceed(&self) -> bool {
        match self.current_page {
            Page::Welcome => {
                let preflight = self.preflight.lock().unwrap();
                if self.production_mode {
                    preflight.internet == CheckStatus::Passed
                        && preflight.uefi == CheckStatus::Passed
                } else {
                    // In dev mode, allow proceeding once checks finish (even if failed)
                    matches!(
                        preflight.internet,
                        CheckStatus::Passed | CheckStatus::Failed
                    ) && matches!(preflight.uefi, CheckStatus::Passed | CheckStatus::Failed)
                }
            }
            Page::Disclaimer => {
                self.disclaimer_format_accepted
                    && self.disclaimer_unfree_accepted
                    && self.disclaimer_surveillance_accepted
            }
            Page::TermsOfSubmission => {
                self.terms_accepted
                    && self.disclaimer_format_accepted
                    && self.disclaimer_unfree_accepted
                    && self.disclaimer_surveillance_accepted
            }
            Page::UserSetup => true,
            Page::PasswordSetup => self.password_theater.accept_ministry_override,
            Page::TimezoneSelection => !self.user_config.timezone.is_empty(),
            Page::KeyboardSelection => !self.user_config.keyboard_layout.is_empty(),
            Page::DiskSelection => self.selected_disk.is_some(),
            Page::FeatureSelection => self.feature_config.all_enabled(),
            Page::HostnameSetup => self.validate_hostname().is_none(),
            Page::Summary => true,
            Page::Installing => false,
            Page::Complete => false,
        }
    }

    pub fn validate_username(&self) -> Option<&'static str> {
        let username = &self.user_config.username;
        if username.is_empty() {
            return Some("Username required");
        }
        if username.len() > 14 {
            return Some("Too long (maximum 14 characters)");
        }
        let digit_count = username.chars().filter(|c| c.is_ascii_digit()).count();
        if digit_count < 1 {
            return Some("Must contain at least 1 number");
        }
        if username.len() < 5 {
            return Some("Too short (minimum 5 characters)");
        }
        if digit_count > 1 {
            return Some("Must have no more than 1 number");
        }
        if username
            .chars()
            .filter_map(|c| c.to_digit(10))
            .any(|d| d < 8)
        {
            return Some("Number looks weak, must be larger than 7");
        }
        if username
            .chars()
            .last()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            return Some("Your number cannot be at the end, thats just too predictable..");
        }
        if username.to_lowercase().contains("test") {
            return Some("Username contains 'test', please choose a different username");
        }
        if username
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
        {
            return Some("All letters are uppercase, are you angry over something?");
        }
        if username.chars().any(|c| c.is_uppercase()) {
            return Some("Username cannot contain uppercase letters");
        }
        let special_char_count = username
            .chars()
            .filter(|c| !c.is_ascii_alphabetic() && !c.is_ascii_digit())
            .count();
        if special_char_count > 0 {
            return Some("This is not a password, remove those special characters...");
        }
        if !username
            .chars()
            .next()
            .map(|c| c.is_ascii_lowercase())
            .unwrap_or(false)
        {
            return Some("Must begin with a letter");
        }
        if !username
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
        {
            return Some("Invalid characters detected (use a-z, 0-9, _, -)");
        }
        None
    }

    pub fn validate_hostname(&self) -> Option<&'static str> {
        let hostname = &self.user_config.hostname;
        if hostname.is_empty() {
            return Some("Network designation required");
        }
        if hostname.len() > 63 {
            return Some("Designation too long (maximum 63 characters)");
        }
        if !hostname
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Some("Invalid characters (use a-z, 0-9, -)");
        }
        if hostname.starts_with('-') || hostname.ends_with('-') {
            return Some("Cannot start or end with hyphen");
        }
        None
    }

    pub fn next_page(&mut self) {
        if let Some(next) = self.current_page.next() {
            self.current_page = next;
        }
    }

    pub fn prev_page(&mut self) {
        if let Some(prev) = self.current_page.prev() {
            self.current_page = prev;
        }
    }

    pub fn get_selected_disk(&self) -> Option<&DiskInfo> {
        self.selected_disk.and_then(|i| self.available_disks.get(i))
    }
}
