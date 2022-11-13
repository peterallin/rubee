use anyhow::Result;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio_serial::SerialStream;

pub struct Ash {
    write_serial: WriteHalf<SerialStream>,
}

impl Ash {
    pub fn new(stream: SerialStream) -> Self {
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

        Self { write_serial }
    }

    // pub async fn reset(&mut self) {
    //     // self.write(&[])
    // }

    pub async fn write(&mut self, data: &[u8]) -> Result<()> {
        self.write_serial.write_all(data).await?;
        Ok(())
    }
}
