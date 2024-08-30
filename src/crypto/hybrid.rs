use aes_gcm::aead::Aead; // Import Aead trait for encrypt/decrypt
use aes_gcm::{Aes256Gcm, KeyInit, Nonce}; // AES-GCM primitives, KeyInit is used for initializing the cipher
use generic_array::GenericArray;
use pqcrypto_kyber::kyber512;
use pqcrypto_traits::kem::{Ciphertext as KyberCiphertext, SharedSecret}; // Removed unused imports
use rand::rngs::OsRng; // Secure random number generator
use rand::RngCore; // For generating random nonces

pub fn hybrid_encrypt(
    data: &[u8],
    recipient_public_key: &kyber512::PublicKey,
) -> (Vec<u8>, Vec<u8>, [u8; 12]) {
    // Generate a random symmetric key
    let symmetric_key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&symmetric_key);

    // Generate a random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt data using AES-GCM
    let ciphertext = cipher
        .encrypt(nonce, data)
        .expect("AES-GCM encryption failed");

    // Encrypt the symmetric key using Kyber
    let (encrypted_symmetric_key, _) = kyber512::encapsulate(recipient_public_key);

    (
        ciphertext,
        encrypted_symmetric_key.as_bytes().to_vec(),
        nonce_bytes,
    )
}

pub fn hybrid_decrypt(
    ciphertext: &[u8],
    encrypted_symmetric_key: &[u8],
    nonce_bytes: &[u8; 12],
    recipient_secret_key: &kyber512::SecretKey,
) -> Result<Vec<u8>, String> {
    // Reconstruct the encrypted symmetric key
    let encrypted_key = KyberCiphertext::from_bytes(encrypted_symmetric_key)
        .map_err(|e| format!("Invalid encrypted symmetric key: {:?}", e))?;

    // Decrypt the symmetric key using Kyber
    let symmetric_key = kyber512::decapsulate(&encrypted_key, recipient_secret_key);

    // Initialize AES-GCM cipher with the decrypted symmetric key
    let cipher = Aes256Gcm::new(GenericArray::from_slice(symmetric_key.as_bytes()));

    // Create nonce from bytes
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt the ciphertext
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("AES-GCM decryption failed: {:?}", e))?;

    Ok(plaintext)
}
