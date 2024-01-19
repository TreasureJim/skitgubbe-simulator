use tokio_tungstenite;
use url;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let addr = std::env::args().nth(1).expect("No arg for address");
    let url = url::Url::parse(&(format!("ws://{addr}/queue"))).expect("Invalid ws url");

    let (mut stream, _) = tokio_tungstenite::connect_async(&url)
        .await
        .expect("connecting to address");
    println!("Connected to {addr}");

    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => println!("Received a message: {}", msg),
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }
}
