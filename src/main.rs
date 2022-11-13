use anyhow::{Context, Result};
use clap::Parser;

mod ash;

#[derive(Debug, clap::Parser)]
struct Options {
    device: String,
    #[clap(default_value = "115200")]
    bitrate: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::parse();
    let port = tokio_serial::new(options.device, options.bitrate);
    let stream = tokio_serial::SerialStream::open(&port).context("Failed to open serial port")?;

    let mut ash = ash::Ash::new(stream);
    ash.write(&[0, 0, 0, 9, 0x7e]).await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    Ok(())
}
