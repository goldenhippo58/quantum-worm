// src/transfer/mod.rs

pub mod file_transfer;

// Re-export functions for easier access if needed
pub use file_transfer::{send_file, receive_file};
