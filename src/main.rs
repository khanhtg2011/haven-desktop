use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "haven-desktop",
    author = "Khanhs <khanhtg2011@gmail.com>",
    version = "4.29.0",
    about = "Haven Desktop - Client engine for loopback audio capture, overlay, and global Push-To-Talk hotkeys"
)]
pub struct Cli {
    #[arg(long, default_value = "https://haven.khanhs.io.vn")]
    pub server_url: String,

    #[arg(long, default_value = "top-left")]
    pub overlay_position: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    tracing::info!(server_url = %cli.server_url, "Starting Haven Desktop Client Subsystems");

    let _hotkeys = haven_desktop_core::hotkeys::HotkeyManager::new();
    let overlay = haven_desktop_core::overlay::VoiceOverlayManager::default();
    let audio_supported = haven_desktop_core::audio::AudioCaptureService::is_supported();

    tracing::info!(
        audio_supported = audio_supported,
        overlay_position = %overlay.config.position,
        "Haven Desktop subsystems active"
    );

    println!("=======================================================");
    println!("  ⬡ Haven Desktop Engine (Rust Edition)");
    println!("  Server: {}", cli.server_url);
    println!("  Loopback Audio Capture: {}", if audio_supported { "Ready" } else { "Unavailable" });
    println!("  Global Hotkeys & Voice Overlay: Active");
    println!("=======================================================");
    println!("Running in background. Press Ctrl+C to terminate.");

    tokio::signal::ctrl_c().await?;
    tracing::info!("Haven Desktop shut down cleanly.");
    Ok(())
}
