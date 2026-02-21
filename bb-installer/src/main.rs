//! BigBother NixOS Installer
//! Your Friendly Surveillance-Themed Setup Wizard

mod app;
mod install;
mod pages;
mod state;
mod theme;
mod widgets;

use app::BigBotherInstaller;
use eframe::egui;
use state::Page;

fn main() -> eframe::Result<()> {
    let is_root = unsafe { libc::geteuid() } == 0;

    // Check production mode
    let production_mode = std::env::var("BB_PROD")
        .map(|v| v == "true")
        .unwrap_or(false);

    eprintln!("╔════════════════════════════════════════════════════════════╗");
    eprintln!("║           BigBother NixOS Installer                        ║");
    eprintln!("╠════════════════════════════════════════════════════════════╣");
    if production_mode {
        eprintln!("║  ⚠️  PRODUCTION MODE - Real disk operations ENABLED        ║");
        eprintln!("║  All installation commands WILL BE EXECUTED                ║");
    } else {
        eprintln!("║  🔒 DRY-RUN MODE - No disk operations will be performed    ║");
        eprintln!("║  Set BB_PROD=true to enable real installation              ║");
    }
    eprintln!(
        "║  Running as root: {}                                        ║",
        if is_root { "yes" } else { "no " }
    );
    eprintln!("╚════════════════════════════════════════════════════════════╝");
    eprintln!();

    // Parse command line arguments for page skipping
    let starting_page = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .and_then(Page::from_index);

    if let Some(page) = &starting_page {
        eprintln!("Starting at page {}: {}", page.index(), page.title());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("BigBother Installation Terminal")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "BigBother Installer",
        options,
        Box::new(move |cc| {
            theme::configure_fonts(&cc.egui_ctx);
            theme::configure_style(&cc.egui_ctx);
            Ok(Box::new(BigBotherInstaller::new_with_page(is_root, starting_page)))
        }),
    )
}
