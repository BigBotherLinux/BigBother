mod complete;
mod disclaimer;
mod disk_selection;
mod features;
mod hostname;
mod installing;
mod keyboard;
mod password_setup;
mod summary;
mod terms;
mod timezone;
mod user_setup;
mod welcome;

use crate::{
    install::start_installation,
    state::{InstallerState, Page},
};

pub fn render_page(ui: &mut eframe::egui::Ui, state: &mut InstallerState) {
    match state.current_page {
        Page::Welcome => welcome::render(ui, state),
        Page::Disclaimer => disclaimer::render(ui, state),
        Page::TermsOfSubmission => terms::render(ui, state),
        Page::UserSetup => user_setup::render(ui, state),
        Page::PasswordSetup => password_setup::render(ui, state),
        Page::TimezoneSelection => timezone::render(ui, state),
        Page::KeyboardSelection => keyboard::render(ui, state),
        Page::DiskSelection => disk_selection::render(ui, state),
        Page::FeatureSelection => features::render(ui, state),
        Page::HostnameSetup => hostname::render(ui, state),
        Page::Summary => summary::render(ui, state),
        Page::Installing => installing::render(ui, state),
        Page::Complete => complete::render(ui, state),
    }
}

pub fn start_install_if_ready(state: &mut InstallerState) {
    // Always use start_installation - it will dry-run if BB_PROD != "true"
    // This ensures we always show the actual commands that would be executed
    start_installation(state);
}
