# Quantum-Worm

Quantum-Worm is a Rust crate that implements a secure, quantum-resistant file transfer system. By leveraging post-quantum cryptographic algorithms, it ensures robust security for file transfers, protecting against potential future threats from quantum computing.

## Features

- **Quantum-Resistant Cryptography**: Utilizes state-of-the-art post-quantum cryptographic algorithms for key encapsulation.
- **Secure File Transfer**: Facilitates the secure transfer of files of any size and type.
- **Efficient Serialization**: Employs `serde` and `bincode` for fast and compact serialization of file metadata.
- **Asynchronous Operations**: Implements non-blocking file transfers using the `tokio` runtime.
- **Symmetric Encryption**: Uses AES-GCM for efficient and secure symmetric encryption of file contents.

## Installation

Add Quantum-Worm to your `Cargo.toml`:

```toml
[dependencies]
quantum-worm = "0.1.0"
```

## Usage

Here's a basic example demonstrating how to use the Quantum-Worm crate for file transfer:

```rust
use quantum_worm::{send_file, receive_file};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example file paths
    let source_file = "path/to/source/file.txt";
    let destination_file = "path/to/destination/file.txt";

    // Send file
    let file_data = tokio::fs::read(source_file).await?;
    send_file(destination_file, &file_data).await?;

    // Receive file
    let received_data = receive_file(destination_file).await?;
    tokio::fs::write(source_file, received_data).await?;

    Ok(())
}
```

## API Reference

### `send_file`

```rust
pub async fn send_file(destination: &str, data: &[u8]) -> Result<(), Error>
```

Encrypts and sends a file to the specified destination.

- `destination`: The path where the encrypted file will be saved.
- `data`: The file contents to be sent.

### `receive_file`

```rust
pub async fn receive_file(source: &str) -> Result<Vec<u8>, Error>
```

Receives and decrypts a file from the specified source.

- `source`: The path of the encrypted file to be received.
- Returns the decrypted file contents.

## Security Considerations

Quantum-Worm uses the following cryptographic primitives:

- **Kyber**: A lattice-based key encapsulation mechanism (KEM) for quantum-resistant key exchange.
- **AES-GCM**: For symmetric encryption of file contents.
- **Secure random number generation**: Provided by the `rand` crate.

While these algorithms are considered secure against known quantum attacks, it's important to keep the crate updated and follow best practices for key management and secure coding.

## Error Handling

The crate uses a custom `Error` type that encompasses various error conditions:

- IO errors
- Serialization/deserialization errors
- Encryption/decryption errors
- Key generation errors

Always handle potential errors when using the crate's functions.

## Performance Considerations

- File transfers are performed asynchronously, allowing for efficient handling of multiple transfers.
- The use of `bincode` for serialization provides a good balance between speed and compact representation.
- For large files, consider implementing chunked transfers to manage memory usage.

## Contributing

Contributions to Quantum-Worm are welcome! Please feel free to submit pull requests or create issues for bugs and feature requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

While Quantum-Worm uses post-quantum cryptographic algorithms, the security landscape is constantly evolving. Always stay informed about the latest developments in cryptography and update your security practices accordingly.