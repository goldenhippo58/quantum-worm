// src/transfer/file_transfer.rs

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn send_file(file_path: &str) -> io::Result<Vec<Vec<u8>>> {
    let mut file = File::open(file_path).await?;
    let mut chunks = Vec::new();

    loop {
        let mut buffer = vec![0; 10 * 1024 * 1024]; // 10 MB chunks
        let bytes_read = file.read(&mut buffer).await?;

        if bytes_read == 0 {
            break;
        }

        buffer.truncate(bytes_read);
        chunks.push(buffer);
    }

    Ok(chunks)
}

pub async fn receive_file(file_path: &str, chunks: Vec<Vec<u8>>) -> io::Result<()> {
    let mut file = File::create(file_path).await?;

    for chunk in chunks {
        file.write_all(&chunk).await?;
    }

    Ok(())
}
