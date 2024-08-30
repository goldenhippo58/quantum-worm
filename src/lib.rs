// src/lib.rs

pub mod transfer;

// Re-export the send_file and receive_file functions
pub use transfer::file_transfer::{send_file, receive_file};
