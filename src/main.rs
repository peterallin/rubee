use anyhow::{Context, Result};
use bytes::BytesMut;
use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio_serial::SerialStream;

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

    let mut ash = ASH::new(stream);
    ash.write(&[0, 0, 0, 9, 0x7e]).await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    Ok(())
}

struct ASH {
    write_serial: WriteHalf<SerialStream>,
}

impl ASH {
    fn new(stream: SerialStream) -> Self {
        let (mut read_serial, write_serial) = tokio::io::split(stream);

        tokio::spawn(async move {
            let mut buf = BytesMut::with_capacity(128);
            loop {
                let count = match read_serial.read_buf(&mut buf).await {
                    Ok(count) => count,
                    Err(e) => {
                        eprintln!("Error reading from port: {e}");
                        continue;
                    }
                };
                println!("{:x?}", &buf[0..count]);
            }
        });

        Self {
            write_serial,
        }
    }

    async fn write(&mut self, data: &[u8]) -> Result<()> {
        self.write_serial.write_all(data).await?;
        Ok(())
    }
}
