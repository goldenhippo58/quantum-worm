use quantum_worm::{receive_file, send_file};

#[tokio::main]
async fn main() {
    let source_file = "txts/source_file.txt"; // Make sure this path exists
    let destination_file = "txts/destination_file.txt"; // Ensure this path is valid and writable

    let sent_data = send_file(source_file).await.unwrap();
    receive_file(destination_file, sent_data).await.unwrap();

    println!("File transfer complete and verified.");
}
