use std::io::Write;
use args::Args;
use clap::Parser;
use windows::Media::Control::{GlobalSystemMediaTransportControlsSessionManager as SessionManager, GlobalSystemMediaTransportControlsSessionMediaProperties as Properties};

mod args;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn get_properties() -> Result<Properties> {
    let session_manager = SessionManager::RequestAsync()?.await?;
    let session = session_manager.GetCurrentSession()?;
    let properties = session.TryGetMediaPropertiesAsync()?.await?;
    Ok(properties)
}

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let mut previous_title = String::new();
    loop {
        if let Ok(properties) = get_properties().await {
            let title = properties.Title()?.to_string();
            let artist = properties.Artist()?.to_string();

            if !title.is_empty() && title != previous_title {
                std::io::stdout().flush()?;
                print!("\x1B[1A\x1B[2K");
                println!("{} \x1B[90m{}\x1B[0m", title, artist);
                previous_title = title.clone();
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}