use quantum_worm::{receive_file, send_file};

#[tokio::test]
async fn test_file_transfer() {
    let test_file = "txts/test_file.txt"; // Make sure this path exists
    let received_file = "txts/received_file.txt"; // Ensure this path is valid and writable

    let sent_data = send_file(test_file).await.unwrap();
    receive_file(received_file, sent_data).await.unwrap();

    // Add assertions to verify the file contents if needed
}
