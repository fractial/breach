use std::fmt::format;
use std::fs::File;
use std::io::Write;
use windows::Media::Control::{GlobalSystemMediaTransportControlsSessionManager as MediaSessionManager, GlobalSystemMediaTransportControlsSessionMediaProperties};
use windows::Storage::Streams::{DataReader, InputStreamOptions, IRandomAccessStreamReference};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn get_media_properties() -> Result<GlobalSystemMediaTransportControlsSessionMediaProperties> {
    let session_manager = MediaSessionManager::RequestAsync()?.await?;
    let session = session_manager.GetCurrentSession()?;

    let properties = session.TryGetMediaPropertiesAsync()?.await?;

    Ok(properties)
}

async fn thumbnail_to_bytes(thumbnail: &IRandomAccessStreamReference) -> Result<Vec<u8>> {
    let stream = thumbnail.OpenReadAsync()?.await?;

    let reader = DataReader::CreateDataReader(&stream)?;
    reader.SetInputStreamOptions(InputStreamOptions::None)?;

    reader.LoadAsync(u32::MAX)?.await?;

    let size = reader.UnconsumedBufferLength()? as usize;
    let mut buffer = vec![0; size];
    reader.ReadBytes(&mut buffer)?;

    Ok(buffer)
}

fn png_from_bytes(bytes: &[u8], filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(bytes)?;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(properties) = get_media_properties().await {
        println!("{}", properties.Title()?);
        println!("\x1B[90m{}\x1B[0m", properties.Artist()?);
        if let Ok(thumbnail) = properties.Thumbnail() {
            let thumbnail_bytes = thumbnail_to_bytes(&thumbnail).await?;
            let _ = png_from_bytes(&thumbnail_bytes, &format!("{}.png", properties.Title()?.to_string()))?;
        }
    } else {
        println!("No `properties` found")
    }

    Ok(())
}